use actix_remote::*;


#[derive(Debug, Message, Serialize, Deserialize)]
pub struct TestMessage {
    pub msg: String,
}

impl RemoteMessage for TestMessage {
    fn type_id() -> &'static str {
        "TestMessage"
    }
}