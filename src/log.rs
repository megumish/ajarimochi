use colored::*;
use futures::prelude::*;

pub enum Verbosity {
    Debug,
    Info,
    Success,
    Warn,
    Error,
    Quiet,
}

pub struct Logger<W>
where
    W: std::io::Write,
{
    pub verbosity: Verbosity,
    pub writer: W,
}

impl<W> Logger<W>
where
    W: std::io::Write,
{
    fn success(mut self, s: &str) {
        let _ = self
            .writer
            .write(format!("{} {}", "[+]".green().bold(), s).as_bytes());
    }
}

pub enum LogMessage {
    Exit,
}

pub async fn logger_run<W>(
    logger: Logger<W>,
    mut receiver: impl Stream<Item = LogMessage> + std::marker::Unpin,
) where
    W: std::io::Write,
{
    loop {
        match receiver.next().await {
            Some(LogMessage::Exit) => {
                logger.success("exit...");
                return;
            }
            _ => { /* do nothing */ }
        }
    }
}

pub async fn end(mut sender: impl Sink<LogMessage> + std::marker::Unpin) {
    let _ = sender.send(LogMessage::Exit).await;
}
