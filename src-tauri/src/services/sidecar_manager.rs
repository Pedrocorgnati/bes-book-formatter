use std::process::Stdio;
use tokio::process::Command;

use crate::error::AppError;
use crate::models::SidecarStatus;

/// Manages external binary sidecars (Typst, Ghostscript, EPUBCheck).
/// Checks availability, spawns processes, and handles timeouts.
pub struct SidecarManager;

impl SidecarManager {
    /// Check if a sidecar binary is available on the system.
    pub async fn check_sidecar(name: &str) -> SidecarStatus {
        let (binary, version_args): (&str, &[&str]) = match name {
            "typst" => ("typst", &["--version"]),
            "ghostscript" => ("gs", &["--version"]),
            "epubcheck" => ("java", &["-jar", "epubcheck.jar", "--version"]),
            _ => {
                return SidecarStatus {
                    name: name.to_string(),
                    available: false,
                    version: None,
                    path: None,
                    error: Some(format!("Unknown sidecar: {}", name)),
                };
            }
        };

        match Command::new(binary)
            .args(version_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .output()
            .await
        {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
                let version = if stdout.is_empty() {
                    String::from_utf8_lossy(&output.stderr).trim().to_string()
                } else {
                    stdout
                };

                // Try to find the full path using `which`
                let path = Command::new("which")
                    .arg(binary)
                    .output()
                    .await
                    .ok()
                    .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string());

                SidecarStatus {
                    name: name.to_string(),
                    available: output.status.success(),
                    version: Some(version),
                    path,
                    error: if output.status.success() {
                        None
                    } else {
                        Some(format!("Exit code: {}", output.status.code().unwrap_or(-1)))
                    },
                }
            }
            Err(e) => SidecarStatus {
                name: name.to_string(),
                available: false,
                version: None,
                path: None,
                error: Some(format!("Failed to execute: {}", e)),
            },
        }
    }

    /// Spawn Typst compiler with given arguments.
    /// Returns (stdout, stderr) on success.
    pub async fn spawn_typst(args: &[String], timeout_ms: u64) -> Result<(String, String), AppError> {
        Self::spawn_process("typst", args, timeout_ms).await
    }

    /// Spawn Ghostscript with given arguments.
    pub async fn spawn_ghostscript(args: &[String], timeout_ms: u64) -> Result<(String, String), AppError> {
        Self::spawn_process("gs", args, timeout_ms).await
    }

    /// Spawn EPUBCheck validation on an EPUB file.
    pub async fn spawn_epubcheck(epub_path: &str, timeout_ms: u64) -> Result<(String, String), AppError> {
        let args = vec![
            "-jar".to_string(),
            "epubcheck.jar".to_string(),
            epub_path.to_string(),
            "--json".to_string(),
            "-".to_string(),
        ];
        Self::spawn_process("java", &args, timeout_ms).await
    }

    /// Generic process spawner with timeout.
    async fn spawn_process(
        binary: &str,
        args: &[String],
        timeout_ms: u64,
    ) -> Result<(String, String), AppError> {
        let child = Command::new(binary)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| {
                if e.kind() == std::io::ErrorKind::NotFound {
                    AppError::sidecar_not_found(binary)
                } else {
                    AppError::sys_internal(format!("Failed to spawn {}: {}", binary, e))
                }
            })?;

        let timeout = tokio::time::Duration::from_millis(timeout_ms);
        match tokio::time::timeout(timeout, child.wait_with_output()).await {
            Ok(Ok(output)) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();

                if output.status.success() {
                    Ok((stdout, stderr))
                } else {
                    Err(AppError::sidecar_crash(
                        binary,
                        output.status.code().unwrap_or(-1),
                        &stderr,
                    ))
                }
            }
            Ok(Err(e)) => Err(AppError::sys_internal(format!("Process error for {}: {}", binary, e))),
            Err(_) => Err(AppError::sidecar_timeout(binary, timeout_ms)),
        }
    }
}
