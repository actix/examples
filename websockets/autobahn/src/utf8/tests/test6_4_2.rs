use bytestring::ByteString;

use super::*;

#[test]
fn invalid_sequence() {
    let data: &[u8] = b"\xce\xba\xe1\xbd\xb9\xcf\x83\xce\xbc\xce\xb5\xf4\x90\x80\x80edited";
    let bytes = BytesMut::from(data).freeze();
    let next_bytes = bytes.slice(11..15);
    let tested_bytes = bytes.slice(11..14);
    println!("Bytes 11,12,13,14: {:#?}", next_bytes);

    let std_result = ByteString::try_from(bytes.clone()).map_err(|e| {
        ProtocolError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}", e),
        ))
    });

    let next_result = ByteString::try_from(next_bytes).map_err(|e| {
        ProtocolError::Io(std::io::Error::new(
            std::io::ErrorKind::Other,
            format!("{}", e),
        ))
    });

    let result = validate_utf8_bytes(tested_bytes);

    println!("Std Result: {:#?}", std_result);
    println!("Next result: {:#?}", next_result);
    println!("Result: {:#?}", result);
}

#[test]
fn first_byte_type() {
    let byte: u8 = 0xf4u8;

    let expected = ByteResult::First(4);

    let result = check_byte(byte);

    assert_eq!(result, expected);
}

#[test]
fn second_byte_type() {
    let byte: u8 = 0x90u8;

    let expected = ByteResult::Continuation;

    let result = check_byte(byte);

    assert_eq!(result, expected);
}
