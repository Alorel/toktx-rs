use std::io;
use std::process::ExitStatus;

/// Conversion error
#[derive(Debug, thiserror::Error)]
pub enum ToKtxError {
    /// Failed to get source path
    #[error("Failed to get source path: {_0}")]
    SourcePath(io::Error),

    /// Error spawning toktx process
    #[error("Error spawning toktx process: {_0}")]
    Spawn(io::Error),

    /// Exited with nonzero exit code
    #[error("Exited with status {status}: {}", String::from_utf8_lossy(&stderr))]
    ExitStatus {
        /// Process exit status
        status: ExitStatus,

        /// Process `stderr` output
        stderr: Vec<u8>,
    }
}
