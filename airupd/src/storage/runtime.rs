//! Represents to Airup's runtime directory.

use airupfx::fs::Lock;
use std::path::PathBuf;

/// Main navigator of Airup's runtime directory.
#[derive(Debug)]
pub struct Runtime {
    base_dir: PathBuf,
}
impl Runtime {
    /// Creates a new [`Runtime`] instance.
    pub async fn new() -> Self {
        let base_dir = airup_sdk::build::manifest().runtime_dir.clone();
        _ = tokio::fs::create_dir_all(&base_dir).await;

        Self { base_dir }
    }

    /// Acquires airup data lock.
    pub async fn lock(&self) -> std::io::Result<Lock> {
        Lock::new(self.base_dir.join("airupd.lock")).await
    }

    /// Returns path of the IPC server socket.
    pub fn ipc_server(&self) -> PathBuf {
        self.base_dir.join("airupd.sock")
    }
}
