use clap::Clap;

mod interpreter;
mod lexer;
mod log;

#[derive(Clap)]
#[clap(version = "0.1.0", author = "megumish <megumish@exploitouri.st>")]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(short, long)]
    quiet: bool,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut verbosity;
    match opts.verbose {
        0 => verbosity = log::Verbosity::Error,
        1 => verbosity = log::Verbosity::Warn,
        2 => verbosity = log::Verbosity::Success,
        3 => verbosity = log::Verbosity::Info,
        4 | _ => verbosity = log::Verbosity::Debug,
    }
    if opts.quiet {
        verbosity = log::Verbosity::Quiet;
    }
    let writer = std::io::stderr();
    let logger = log::Logger { verbosity, writer };

    let pool = futures::executor::ThreadPool::new().expect("failed to build thread pool");

    let (log_tx, log_rx) = futures::channel::mpsc::unbounded::<log::LogMessage>();

    pool.spawn_ok(interpreter::run(log_tx));

    futures::executor::block_on(log::logger_run(logger, log_rx));
}
