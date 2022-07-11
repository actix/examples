use tokio::sync::{mpsc, oneshot};

use crate::{ConnId, Msg, RoomId};

#[derive(Debug)]
pub enum Command {
    Connect {
        conn_tx: mpsc::UnboundedSender<Msg>,
        res_tx: oneshot::Sender<ConnId>,
    },

    Disconnect {
        conn: ConnId,
    },

    List {
        res_tx: oneshot::Sender<Vec<RoomId>>,
    },

    Join {
        conn: ConnId,
        room: RoomId,
        res_tx: oneshot::Sender<()>,
    },

    Message {
        room: RoomId,
        msg: Msg,
        skip: ConnId,
        res_tx: oneshot::Sender<()>,
    },
}
