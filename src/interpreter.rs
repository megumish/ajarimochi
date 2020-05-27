use futures::prelude::*;

use crate::log::LogMessage;

pub async fn run(mut sender: impl Sink<LogMessage> + std::marker::Unpin) {
    let _ = sender.send(LogMessage::Exit).await;
}
