use super::*;

#[test]
fn invalid_sequence() {
    let data: &[u8] = b"\xce\xba\xe1\xbd\xb9\xcf\x83\xce\xbc\xce\xb5\xf4\x90\x80\x80edited";
    let bytes = BytesMut::from(data).freeze();
    let tested_bytes = bytes.slice(11..14);

    let result = validate_utf8_bytes(tested_bytes);

    match result {
        Err(ProtocolError::Io(err)) => {
            assert_eq!(err.kind(), std::io::ErrorKind::InvalidData, "Error kind should be `Other`");
            assert_eq!(err.to_string(), ERROR_INVALID_UTF8_SEQUENCE_MESSAGE.to_owned());
        },
        Err(_) => assert!(false, "Result should be ProtocolError::Io"),
        Ok(_) => assert!(false, "Result should be an error")
    }
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
