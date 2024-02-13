//! A channel for shutdown notification.

use tokio::sync::mpsc;

/// Creates a shutdown channel.
pub fn channel() -> (Sender, Receiver) {
    let (notify_tx, notify_rx) = mpsc::channel(1);
    let (complete_tx, complete_rx) = mpsc::channel(1);
    let notify = notify_tx.clone();
    let complete = complete_tx.clone();
    let sender = Sender {
        notify_rx,
        notify_tx,
        complete_rx,
        complete_tx,
    };
    let receiver = Receiver {
        shutdown: false,
        notify,
        _complete: complete,
    };

    (sender, receiver)
}

/// Sending-half of the shutdown channel.
#[derive(Debug)]
pub struct Sender {
    /// Used to send the shutdown signal when dropped.
    notify_rx: mpsc::Receiver<()>,
    /// Needed to create new [`Receiver`]s.
    notify_tx: mpsc::Sender<()>,
    /// Used to wait for shutdown completion. Once all [`Receiver`]s connected to this [`Sender`]
    /// have been dropped, the sending half of this [`mpsc`] channel will close, which will wake up
    /// the corresponding task waiting for a receive.
    complete_rx: mpsc::Receiver<()>,
    /// Needed to create new [`Receiver`]s.
    complete_tx: mpsc::Sender<()>,
}

impl Sender {
    /// Sends the shutdown signal and waits for all receivers to shut down.
    ///
    /// All receivers waiting on the signal will be woken up, after which they should clean up and
    /// signal back that they have completed their shutdown.
    pub async fn shutdown(mut self) {
        // Send the shutdown signal.
        drop(self.notify_rx);

        // We must drop our copy of the completion sender or the below would deadlock.
        drop(self.complete_tx);

        // This waits for all completion senders to be dropped, since we never actually send
        // anything across the channel.
        self.complete_rx.recv().await;
    }

    /// Creates a new [`Receiver`] connected to this `Sender`.
    pub fn subscribe(&self) -> Receiver {
        Receiver {
            shutdown: false,
            notify: self.notify_tx.clone(),
            _complete: self.complete_tx.clone(),
        }
    }
}

/// Receiving-half of the shutdown channel.
#[derive(Debug)]
pub struct Receiver {
    /// `true` if the shutdown signal has been received.
    shutdown: bool,
    /// Used to listen for the shutdown signal.
    notify: mpsc::Sender<()>,
    /// Used to send the shutdown completion signal when dropped.
    _complete: mpsc::Sender<()>,
}

impl Receiver {
    /// Waits for the shutdown signal, or returns if the signal has already been received.
    pub async fn notified(&mut self) {
        // If the shutdown signal has already been received, then return immediately.
        if self.shutdown {
            return;
        }

        // Wait for the receiver to be dropped, signalling the shutdown.
        self.notify.closed().await;

        // Remember that the signal has been received.
        self.shutdown = true;
    }
}
