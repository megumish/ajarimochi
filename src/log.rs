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
    fn success(&mut self, s: &str) {
        let _ = self
            .writer
            .write(format!("{} {}", "[+]".green().bold(), s).as_bytes());
    }

    fn error(&mut self, s: &str) {
        let _ = self
            .writer
            .write(format!("{} {}", "[x]".red().bold(), s).as_bytes());
    }
}

pub enum LogMessage {
    Error(String),
    Exit,
}

pub async fn run<W>(
    mut logger: Logger<W>,
    mut receiver: impl Stream<Item = LogMessage> + std::marker::Unpin,
) where
    W: std::io::Write,
{
    loop {
        match receiver.next().await {
            Some(LogMessage::Error(s)) => {
                logger.error(&s);
            }
            Some(LogMessage::Exit) => {
                logger.success("exit...");
                return;
            }
            None => logger.success("AFJNaiksj"),
        }
    }
}

pub async fn end(mut sender: impl Sink<LogMessage> + std::marker::Unpin) {
    let _ = sender.send(LogMessage::Exit).await;
}
