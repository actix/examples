//! JSON-RPC 2.0 Specification
//! See: https://www.jsonrpc.org/specification
use std::error;
use std::fmt;

use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

pub static JSONRPC_VERSION: &str = "2.0";

/// When a rpc call encounters an error, the Response Object MUST contain the
/// error member with a value that is a Object with the following members:
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorData {
    /// A Number that indicates the error type that occurred. This MUST be an integer.
    pub code: i32,

    /// A String providing a short description of the error. The message SHOULD be
    /// limited to a concise single sentence.
    pub message: String,

    /// A Primitive or Structured value that contains additional information
    /// about the error. This may be omitted. The value of this member is
    /// defined by the Server (e.g. detailed error information, nested errors
    /// etc.).
    pub data: Value,
}

impl ErrorData {
    pub fn new(code: i32, message: &str) -> Self {
        Self {
            code,
            message: String::from(message),
            data: Value::Null,
        }
    }

    pub fn std(code: i32) -> Self {
        match code {
            // Invalid JSON was received by the server. An error occurred on the server while parsing the JSON text.
            -32700 => ErrorData::new(-32700, "Parse error"),
            // The JSON sent is not a valid Request object.
            -32600 => ErrorData::new(-32600, "Invalid Request"),
            // The method does not exist / is not available.
            -32601 => ErrorData::new(-32601, "Method not found"),
            // Invalid method parameter(s).
            -32602 => ErrorData::new(-32602, "Invalid params"),
            // Internal JSON-RPC error.
            -32603 => ErrorData::new(-32603, "Internal error"),
            // The error codes from and including -32768 to -32000 are reserved for pre-defined errors. Any code within
            // this range, but not defined explicitly below is reserved for future use.
            _ => panic!("Undefined pre-defined error codes"),
        }
    }

    /// Prints out the value as JSON string.
    pub fn dump(&self) -> String {
        serde_json::to_string(self).expect("Should never failed")
    }
}

impl error::Error for ErrorData {}
impl fmt::Display for ErrorData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {})", self.code, self.message, self.data)
    }
}

/// A rpc call is represented by sending a Request object to a Server.
#[derive(Debug, Serialize, Deserialize)]
pub struct Request {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    pub jsonrpc: String,

    /// A String containing the name of the method to be invoked. Method names that begin with the word rpc followed by
    /// a period character (U+002E or ASCII 46) are reserved for rpc-internal methods and extensions and MUST NOT be
    /// used for anything else.
    pub method: String,

    /// A Structured value that holds the parameter values to be used during the invocation of the method. This member
    /// MAY be omitted.
    pub params: Vec<Value>,

    /// An identifier established by the Client that MUST contain a String, Number, or NULL value if included. If it is
    /// not included it is assumed to be a notification. The value SHOULD normally not be Null [1] and Numbers SHOULD
    /// NOT contain fractional parts.
    pub id: Value,
}

impl Request {
    /// Prints out the value as JSON string.
    pub fn dump(&self) -> String {
        serde_json::to_string(self).expect("Should never failed")
    }
}

/// When a rpc call is made, the Server MUST reply with a Response, except for in the case of Notifications. The
/// Response is expressed as a single JSON Object, with the following members:
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    /// A String specifying the version of the JSON-RPC protocol. MUST be exactly "2.0".
    pub jsonrpc: String,

    /// This member is REQUIRED on success.
    /// This member MUST NOT exist if there was an error invoking the method.
    /// The value of this member is determined by the method invoked on the Server.
    pub result: Value,

    // This member is REQUIRED on error.
    // This member MUST NOT exist if there was no error triggered during invocation.
    // The value for this member MUST be an Object as defined in section 5.1.
    pub error: Option<ErrorData>,

    /// This member is REQUIRED.
    /// It MUST be the same as the value of the id member in the Request Object.
    /// If there was an error in detecting the id in the Request object (e.g. Parse error/Invalid Request),
    /// it MUST be Null.
    pub id: Value,
}

impl Response {
    /// Prints out the value as JSON string.
    pub fn dump(&self) -> String {
        serde_json::to_string(self).expect("Should never failed")
    }
}

impl Default for Response {
    fn default() -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.into(),
            result: Value::Null,
            error: None,
            id: Value::Null,
        }
    }
}
