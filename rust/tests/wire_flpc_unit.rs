use flpc::{
    unit::{FlpcUnit, ServerRegister},
    wire::{Encode, EncodeError},
};

// Independent bytes from README opcode 0x80 field layout.
const FRAME: &[u8] = &[0x80, 0x00];

#[test]
fn server_register_converts_and_encodes_as_flpc_unit() {
    let unit: FlpcUnit = ServerRegister::new(0).into();
    assert_eq!(unit, FlpcUnit::ServerRegister(ServerRegister::new(0)));

    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(dst[FRAME.len()], 0xaa);

    let mut short = [0xaa; FRAME.len() - 1];
    assert!(matches!(
        unit.encode_into(&mut short),
        Err(EncodeError::BufferTooSmall)
    ));
    assert_eq!(short, [0xaa]);
}
