//! Length-prefixed JSON requests and responses shared by the TCP server and client.

use std::io;

use actix_codec::{Decoder, Encoder};
use actix_web::web::{BufMut, BytesMut};
use byteorder::{BigEndian, ByteOrder};
use serde::{Deserialize, Serialize};
use serde_json as json;

/// Client request
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", content = "data")]
pub enum ChatRequest {
    /// List rooms
    List,
    /// Join rooms
    Join(String),
    /// Send message
    Message(String),
    /// Ping
    Ping,
}

/// Server response
#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd", content = "data")]
pub enum ChatResponse {
    Ping,

    /// List of rooms
    Rooms(Vec<String>),

    /// Joined
    Joined(String),

    /// Message
    Message(String),
}

/// Codec for Client -> Server transport
#[allow(dead_code)] // Unused by the TCP client binary.
pub struct ChatCodec;

impl Decoder for ChatCodec {
    type Item = ChatRequest;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            let _ = src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ChatRequest>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<ChatResponse> for ChatCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: ChatResponse, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg)?;
        let msg_ref: &[u8] = msg.as_ref();

        let size = u16::try_from(msg_ref.len()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "chat frame exceeds 65535 bytes")
        })?;

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(size);
        dst.put(msg_ref);

        Ok(())
    }
}

/// Codec for Server -> Client transport
#[allow(dead_code)] // Unused by the TCP server binary.
pub struct ClientChatCodec;

impl Decoder for ClientChatCodec {
    type Item = ChatResponse;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        let size = {
            if src.len() < 2 {
                return Ok(None);
            }
            BigEndian::read_u16(src.as_ref()) as usize
        };

        if src.len() >= size + 2 {
            let _ = src.split_to(2);
            let buf = src.split_to(size);
            Ok(Some(json::from_slice::<ChatResponse>(&buf)?))
        } else {
            Ok(None)
        }
    }
}

impl Encoder<ChatRequest> for ClientChatCodec {
    type Error = io::Error;

    fn encode(&mut self, msg: ChatRequest, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let msg = json::to_string(&msg)?;
        let msg_ref: &[u8] = msg.as_ref();

        let size = u16::try_from(msg_ref.len()).map_err(|_| {
            io::Error::new(io::ErrorKind::InvalidData, "chat frame exceeds 65535 bytes")
        })?;

        dst.reserve(msg_ref.len() + 2);
        dst.put_u16(size);
        dst.put(msg_ref);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_a_request_split_across_reads() {
        let mut encoded = BytesMut::new();
        ClientChatCodec
            .encode(ChatRequest::Join("room".into()), &mut encoded)
            .unwrap();
        let mut input = encoded.split_to(1);
        assert!(ChatCodec.decode(&mut input).unwrap().is_none());
        input.extend_from_slice(&encoded.split_to(encoded.len() - 1));
        assert!(ChatCodec.decode(&mut input).unwrap().is_none());
        input.extend_from_slice(&encoded);
        assert!(
            matches!(ChatCodec.decode(&mut input).unwrap(), Some(ChatRequest::Join(room)) if room == "room")
        );
        assert!(input.is_empty());
    }

    #[test]
    fn rejects_responses_that_exceed_the_wire_length_limit() {
        let mut output = BytesMut::new();
        let err = ChatCodec
            .encode(ChatResponse::Message("x".repeat(65536)), &mut output)
            .unwrap_err();
        assert_eq!(err.kind(), io::ErrorKind::InvalidData);
        assert!(output.is_empty());
    }
}
