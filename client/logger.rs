//! The logging infrastructure.
//!
//! Logging is done completely asynchronously, with the logger not writing to the log file directly
//! (which would block) but rather sending the log record to a tokio task dedicated to writing log
//! records. This way writes can also be buffered, reducing the amount of syscalls arising from
//! small and repeated writes, without any locking.

use std::fmt::Write;
use std::future::Future;
use std::num::NonZeroU64;
use std::path::{Path, PathBuf};
use std::{io, thread};

use anyhow::{Context, Result};
use bytes::BytesMut;
use futures_util::SinkExt;
use tokio::fs::{File, OpenOptions};
use tokio::sync::mpsc;
use tokio_util::codec::{Encoder, FramedWrite};

use crate::shutdown;

/// Initializes the logger, returning the logging task as well as the sender used to shutdown the
/// logging task. This function should be called as early as possible, so that no log records are
/// dropped (that is, right at the start of the program).
///
/// # Panics
///
/// Panics if called more than once.
pub fn init(log_dir: &Path, log_level_filter: log::LevelFilter) -> (LoggingTask, shutdown::Sender) {
    let (record_tx, record_rx) = mpsc::unbounded_channel();

    // We have to leak the record sender because a static reference to it is required. As a
    // byproduct this means that the sender is never going to be disconnected.
    let sender = Box::leak(Box::new(Sender {
        record_tx,
        log_level_filter,
    }));

    log::set_logger(sender).unwrap();
    log::set_max_level(log_level_filter);

    log::trace!("Initialized logger");

    let (shutdown_tx, shutdown_rx) = shutdown::channel();

    let log_name = chrono::Local::now().format("%F_%H-%M-%S.log").to_string();
    let log_path = log_dir.join(log_name);
    let logging_task = run(log_path, record_rx, shutdown_rx);

    (logging_task, shutdown_tx)
}

/// Runs the tokio task which receives the records and writes them to the log file asynchronously.
///
/// # Panics
///
/// Panics when an I/O error is encountered.
async fn run(
    log_path: PathBuf,
    record_rx: mpsc::UnboundedReceiver<Record>,
    mut shutdown_rx: shutdown::Receiver,
) {
    let file = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(log_path)
        .await
        .context("failed to open log file")
        .unwrap();
    let sink = FramedWrite::new(file, RecordEncoder);

    let mut logger = Receiver { record_rx, sink };

    log::trace!("Starting logging task");

    tokio::select! {
        res = logger.run() => res.context("logging task encountered an error"),
        () = shutdown_rx.notified() => {
            log::trace!("Shutting down logging task");

            logger.close().await
        }
    }
    // Of course, we would like to log these errors, but since they happened inside the logger
    // itself that would likely lead to more of the same error.
    .unwrap();
}

pub type LoggingTask = impl Future<Output = ()>;

/// The record-sending half of the logger.
struct Sender {
    /// The sender used to send log records to the logging task.
    record_tx: mpsc::UnboundedSender<Record>,
    /// Maximum log level. Lower level log records will be ignored.
    log_level_filter: log::LevelFilter,
}

impl log::Log for Sender {
    fn enabled(&self, metadata: &log::Metadata<'_>) -> bool {
        metadata.level() <= self.log_level_filter
    }

    fn log(&self, record: &log::Record<'_>) {
        if self.enabled(record.metadata()) {
            self.record_tx
                .send(Record {
                    time: chrono::Utc::now(),
                    level: record.level(),
                    thread_id: thread::current().id().as_u64(),
                    // We are relying on the fact that the macros from the `log` crate always
                    // specify a static module path and line number. Theoretically, one could call
                    // this function directly, but that's incorrect API usage and no crate should
                    // ever be doing that.
                    module: record.module_path_static().unwrap(),
                    line: record.line().unwrap(),
                    // Unfortunately, we can't pass the arguments as-is because they borrow from
                    // the calling scope. We have to format the arguments and send that, which
                    // means that this call will block somewhat, but at least it's limited to the
                    // arguments. Formatting the entire log record and sending that for example
                    // results in 6x the wait for the caller in the worse case.
                    msg: record.args().to_string(),
                })
                // This shouldn't happen because the logging task should *always* be the last to be
                // shut down.
                .expect("failed to send log record: receiver was disconnected unexpectedly");
        }
    }

    fn flush(&self) {}
}

/// The record-receiving half of the logger.
struct Receiver {
    /// The receiver used to receive log records from the log sender.
    record_rx: mpsc::UnboundedReceiver<Record>,
    /// The sink to feed the records into. This takes care of encoding the records, buffering the
    /// encoded bytes, as well as writing them asynchronously to the underlying file.
    sink: FramedWrite<File, RecordEncoder>,
}

impl Receiver {
    /// Runs the logging task, whose sole purpose in life is to feed received log records into a
    /// sink.
    async fn run(&mut self) -> Result<()> {
        loop {
            // This can't return `None` because the sender is leaked.
            let record = self.record_rx.recv().await.unwrap();

            self.sink
                .feed(record)
                .await
                .context("failed to append log record to the log file")?;
        }
    }

    /// Closes the channel, drains and writes the remaining records from it, and flushes the sink.
    async fn close(&mut self) -> Result<()> {
        self.record_rx.close();

        while let Some(record) = self.record_rx.recv().await {
            self.sink
                .feed(record)
                .await
                .context("failed to append log record to the log file")?;
        }

        self.sink
            .flush()
            .await
            .context("failed to flush pending log records")
    }
}

/// A log record. Unlike `log::Record`, this can be sent across a channel.
#[derive(Debug)]
struct Record {
    /// Time of sending.
    time: chrono::DateTime<chrono::Utc>,
    /// Log level.
    level: log::Level,
    /// ID of the thread the record originates from.
    thread_id: NonZeroU64,
    /// Module the record originates from.
    module: &'static str,
    /// Line in the file the record originates from.
    line: u32,
    /// Log message.
    msg: String,
}

/// Encodes records, surprisingly.
struct RecordEncoder;

impl Encoder<Record> for RecordEncoder {
    type Error = io::Error;

    fn encode(&mut self, item: Record, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let Record {
            time,
            level,
            thread_id,
            module,
            line,
            msg,
        } = item;

        let time = time.format("%T.%6f");

        writeln!(
            dst,
            "{time} {thread_id:02} {level:5} [{module}({line})] {msg}",
        )
        // Writing to `BytesMut` is infallible.
        .unwrap();

        Ok(())
    }
}
