#![feature(thread_id_value, type_alias_impl_trait)]

use anyhow::Context;
use tokio::runtime::Runtime;

use self::env::Env;

mod env;
mod logger;
mod shutdown;

fn main() {
    let env = Env::load();

    let (logging_task, logger_shutdown_tx) = logger::init(&env.log_dir, env.log_level_filter);

    let runtime = Runtime::new()
        .context("failed to create tokio runtime")
        .unwrap();

    runtime.spawn(logging_task);
    let shutdown_guard = ShutdownGuard::new(runtime, logger_shutdown_tx);
}

/// Blocks on the shutdown of tasks when dropped.
struct ShutdownGuard {
    runtime: Runtime,
    logger_shutdown_tx: Option<shutdown::Sender>,
}

impl ShutdownGuard {
    fn new(runtime: Runtime, logger_shutdown_tx: shutdown::Sender) -> Self {
        ShutdownGuard {
            runtime,
            logger_shutdown_tx: Some(logger_shutdown_tx),
        }
    }
}

impl Drop for ShutdownGuard {
    fn drop(&mut self) {
        let logger_shutdown_tx = self.logger_shutdown_tx.take().unwrap();
        self.runtime.block_on(logger_shutdown_tx.shutdown());
    }
}
