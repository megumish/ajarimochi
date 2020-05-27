use crate::log::{self, LogMessage};
use futures::prelude::*;

mod token;
mod lexer;

use token::{Kind as TokenKind, Position as TokenPosition, Token};

pub enum LexicalError {}

pub async fn run(
    pool: futures::executor::ThreadPool,
    mut code_receiver: impl AsyncBufRead + std::marker::Unpin,
    mut log_sender: impl Sink<LogMessage> + std::marker::Unpin,
) {
    let buf = {
        let mut buf = Vec::<u8>::new();
        match code_receiver.read_to_end(&mut buf).await {
            Err(e) => {
                let _ = log_sender
                    .send(LogMessage::Error(format!(
                        "failed to read code. : {}",
                        e.to_string()
                    )))
                    .await;
                let _ = log_sender.send(LogMessage::Exit).await;
                return;
            }
            Ok(_) => { /* do nothing */ }
        };
        buf
    };

    analyze(buf, pool, log_sender);
}

pub fn analyze(
    buf: Vec<u8>,
    pool: futures::executor::ThreadPool,
    mut log_sender: impl Sink<LogMessage> + std::marker::Unpin,
) {
}
