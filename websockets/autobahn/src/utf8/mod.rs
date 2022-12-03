#[cfg(test)]
mod tests;

use actix_http::ws::ProtocolError;

use bytes::BufMut;
use bytes::Bytes;
use bytes::BytesMut;

#[derive(Debug)]
pub struct ValidUtf8 {
    pub valid: Bytes,
    pub overflow: Option<Bytes>,
}

const UTF8_START_2_BYTE_SEQ_MASK: u8 = 0b1110_0000u8;
const UFT8_START_3_BYTE_SEQ_MASK: u8 = 0b1111_0000u8;
const UTF8_START_4_BYTE_SEQ_MASK: u8 = 0b1111_1000u8;
const UTF8_START_5_BYTE_SEQ_MASK: u8 = 0b1111_1100u8;
const UTF8_START_6_BYTE_SEQ_MASK: u8 = 0b1111_1110u8;

const UTF8_2_BYTE_SEQ: u8 = 0b11000000u8;
const UTF8_3_BYTE_SEQ: u8 = 0b11100000u8;
const UTF8_4_BYTE_SEQ: u8 = 0b11110000u8;
const UTF8_5_BYTE_SEQ: u8 = 0b11111000u8;
const UTF8_6_BYTE_SEQ: u8 = 0b11111100u8;

const MAX_ASCII_VALUE: u8 = 0x7Fu8;
const MIN_CONTINUATION: u8 = 0x80u8;
const MAX_CONTINUATION: u8 = 0xBFu8;

#[derive(Debug, PartialEq)]
pub enum ByteResult {
    Continuation,
    First(usize),
    Ok,
    Invalid,
}

fn protocol_error<T>(error: String, kind: std::io::ErrorKind) -> Result<T, ProtocolError> {
    Err(ProtocolError::Io(std::io::Error::new(kind, error)))
}

fn protocol_other_error<T>(error: String) -> Result<T, ProtocolError> {
    protocol_error(error, std::io::ErrorKind::Other)
}

fn protocol_data_error<T>(error: String) -> Result<T, ProtocolError> {
    protocol_error(error, std::io::ErrorKind::InvalidData)
}

/// This method rebuilds the code point up to the given point
///
/// You can invoke this method only for the overflowed ("unfinished") code point.
/// As the consequence:
///
/// 1. `data[0]` is always valid
/// 2. We don't need to check the last byte since it's not there
///
///
/// > From Unicode 13 spec
/// >
/// > | Code Points           | First Byte      | Second Byte     | Third Byte      | Fourth Byte     |
/// > |:----------------------|:----------------|:----------------|:----------------|:----------------|
/// > | U+0000   ..= U+007f   | `0x00 ..= 0x7f` |                 |                 |                 |
/// > | U+0080   ..= U+07FF   | `0xC2 ..= 0xDF  | `0x80 ..= 0xBF` |                 |                 |
/// > | U+0800   ..= U+0FFF   | `0xE0`          | `0xA0 ..= 0xBF` | `0x80 ..= 0xBF` |                 |
/// > | U+1000   ..= U+CFFF   | `0xE1 ..= 0xEC  | `0x80 ..= 0xBF` | `0x80 ..= 0xBF` |                 |
/// > | U+D000   ..= U+D7FF   | `0xED`          | `0x80 ..= 0x9F` | `0x80 ..= 0xBF` |                 |
/// > | U+E000   ..= U+FFFF   | `0xEE ..= 0xEF` | `0x80 ..= 0xBF` | `0x80 ..= 0xBF` |                 |
/// > | U+10000  ..= U+3FFFF  | `0xF0`          | `0x90 ..= 0xBF` | `0x80 ..= 0xBF` | `0x80 ..= 0xBF` |
/// > | U+40000  ..= U+FFFFF  | `0xF1 ..= 0xF3` | `0x80 ..= 0xBF` | `0x80 ..= 0xBF` | `0x80 ..= 0xBF` |
/// > | U+100000 ..= U+10FFFF | `0xF4`          | `0x80 ..= 0x8F` | `0x80 ..= 0xBF` | `0x80 ..= 0xBF` |
fn check_overflow(data: &[u8], expected_size: usize) -> bool {
    let len = data.len();

    let raw_1 = data[0];
    if expected_size == 2 {
        (0xC2u8..=0xDFu8).contains(&raw_1)
    } else if expected_size == 3 {
        let raw_2: u8 = if len == 2 {
            data[1]
        } else if raw_1 == 0xE0 {
            0xA0
        } else {
            0x80
        };

        match (raw_1, raw_2) {
            (0xE0, 0xA0..=0xBF) | (0xE1..=0xEC, 0x80..=0xBF) | (0xED, 0x80..=0x9F) => true,
            _ => false,
        }
    } else {
        let raw_2: u8 = if len >= 2 {
            data[1]
        } else if raw_1 == 0xF0 {
            0x90
        } else {
            0x80
        };
        let raw_3: u8 = if len == 3 { data[2] } else { 0x80 };

        match (raw_1, raw_2, raw_3) {
            (0xF0, 0x90..=0xBF, 0x80..=0xBF)
            | (0xF1..=0xF3, 0x80..=0xBF, 0x80..=0xBF)
            | (0xf4, 0x80..=0x8F, 0x80..=0xBF) => true,
            _ => false,
        }
    }
}

fn check_byte(byte: u8) -> ByteResult {
    if byte <= MAX_ASCII_VALUE {
        ByteResult::Ok
    } else if byte >= MIN_CONTINUATION && byte <= MAX_CONTINUATION {
        ByteResult::Continuation
    } else if byte & UTF8_START_2_BYTE_SEQ_MASK == UTF8_2_BYTE_SEQ {
        ByteResult::First(2)
    } else if byte & UFT8_START_3_BYTE_SEQ_MASK == UTF8_3_BYTE_SEQ {
        ByteResult::First(3)
    } else if byte & UTF8_START_4_BYTE_SEQ_MASK == UTF8_4_BYTE_SEQ {
        ByteResult::First(4)
    } else if byte & UTF8_START_5_BYTE_SEQ_MASK == UTF8_5_BYTE_SEQ {
        ByteResult::First(5)
    } else if byte & UTF8_START_6_BYTE_SEQ_MASK == UTF8_6_BYTE_SEQ {
        ByteResult::First(6)
    } else {
        ByteResult::Invalid
    }
}

pub fn validate_utf8_bytes(data: Bytes) -> Result<ValidUtf8, ProtocolError> {
    let len: usize = data.len();
    let mut overflow_size: usize = 0;
    let mut checked: ByteResult;

    if len == 0 {
        Ok(ValidUtf8 {
            valid: data,
            overflow: None,
        })
    } else {
        let mut index = len;
        let mut expected_overflow_size = 0;

        while index > 0 {
            index -= 1;
            let current = match data.get(index) {
                Some(b) => b,
                None => return protocol_other_error("invalid utf-8 sequence".to_owned()),
            };

            checked = check_byte(*current);

            match checked {
                ByteResult::Continuation => {
                    overflow_size += 1;
                    continue;
                }
                ByteResult::First(seq_size) => {
                    overflow_size += 1;

                    if overflow_size == seq_size {
                        index = len;
                        overflow_size = 0;
                        expected_overflow_size = 0;
                        break;
                        // we've just checked that whole code point is inside this data frame, so no overflow is required
                    }
                    if overflow_size > seq_size {
                        return protocol_data_error("invalid utf-8 sequence".to_owned());
                    }

                    expected_overflow_size = seq_size;
                    break;
                }
                ByteResult::Ok => {
                    index += 1;
                    break;
                }
                ByteResult::Invalid => {
                    return protocol_data_error("invalid utf-8 sequence".to_owned())
                }
            }
        }

        // index points at first "overflowed" byte
        if overflow_size > 0 {
            let (data, overflow) = data.split_at(index);

            if !check_overflow(overflow, expected_overflow_size) {
                return protocol_data_error("Data is not a valid utf8 string".to_owned());
            }

            let mut bytes_data = BytesMut::with_capacity(data.len());
            bytes_data.put(data);

            let mut bytes_overflow = BytesMut::with_capacity(overflow.len());
            bytes_overflow.put(overflow);

            Ok(ValidUtf8 {
                valid: bytes_data.freeze(),
                overflow: Some(bytes_overflow.freeze()),
            })
        } else {
            Ok(ValidUtf8 {
                valid: data,
                overflow: None,
            })
        }
    }
}
