use flpc::{
    unit::{FlpcUnit, ServerRegister, ServerUnit},
    wire::Encode,
};

// Independent bytes from README opcode 0x80 field layout.
const FRAME: &[u8] = &[0x80, 0x00];

#[test]
fn flpc_unit_encodes_a_concrete_server_unit() {
    let unit = FlpcUnit::Server(ServerUnit::ServerRegister(ServerRegister::new(0)));
    let mut dst = [0xaa; FRAME.len() + 1];

    assert_eq!(unit.encoded_len().unwrap(), FRAME.len());
    assert_eq!(unit.encode_into(&mut dst).unwrap(), FRAME.len());
    assert_eq!(&dst[..FRAME.len()], FRAME);
    assert_eq!(dst[FRAME.len()], 0xaa);
}
