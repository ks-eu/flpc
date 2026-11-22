use super::EncodeError;

pub(super) fn short_strings_len(fields: &[&str]) -> Result<usize, EncodeError> {
    let mut len = 0;
    for field in fields {
        u8::try_from(field.len()).map_err(|_| EncodeError::StringTooLong)?;
        len += 1 + field.len();
    }
    Ok(len)
}

pub(super) fn encode_frame(
    buf: &mut [u8],
    len: usize,
    opcode: u8,
    write_fields: impl FnOnce(&mut [u8]),
) -> Result<usize, EncodeError> {
    let frame = buf.get_mut(..len).ok_or(EncodeError::BufferTooSmall)?;
    frame[0] = opcode;
    write_fields(&mut frame[1..]);
    Ok(len)
}

fn write_bytes(dst: &mut &mut [u8], bytes: &[u8]) {
    let (field, remaining) = std::mem::take(dst).split_at_mut(bytes.len());
    field.copy_from_slice(bytes);
    *dst = remaining;
}

pub(super) fn write_short_string(dst: &mut &mut [u8], field: &str) {
    write_bytes(dst, &[field.len() as u8]);
    write_bytes(dst, field.as_bytes());
}

pub(super) fn long_string_len(value: &str) -> Result<usize, EncodeError> {
    u16::try_from(value.len()).map_err(|_| EncodeError::StringTooLong)?;
    Ok(2 + value.len())
}

pub(super) fn write_long_string(dst: &mut [u8], value: &str) {
    let (prefix, body) = dst.split_at_mut(2);
    prefix.copy_from_slice(&(value.len() as u16).to_be_bytes());
    body.copy_from_slice(value.as_bytes());
}
