use anyhow::{Context, Result};
use nix::fcntl::{flock, FlockArg};
use nix::unistd::Pid;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::unix::io::AsRawFd;
use std::path::PathBuf;
use std::time::{Duration, Instant};
use crate::core::errors::RatpmError;

const LOCK_TIMEOUT: Duration = Duration::from_secs(30);
const LOCK_POLL_INTERVAL: Duration = Duration::from_millis(100);

pub trait LockManager {
    fn acquire(&self) -> Result<LockGuard>;
}

pub struct FileLockManager {
    lock_path: PathBuf,
}

impl FileLockManager {
    pub fn new(lock_path: PathBuf) -> Self {
        Self { lock_path }
    }
    
    fn ensure_lock_file_exists(&self) -> Result<()> {
        if let Some(parent) = self.lock_path.parent() {
            std::fs::create_dir_all(parent)
                .context("Failed to create lock directory")?;
        }
        
        if !self.lock_path.exists() {
            File::create(&self.lock_path)
                .context("Failed to create lock file")?;
        }
        
        Ok(())
    }
    
    fn read_lock_holder(&self) -> Option<Pid> {
        let mut file = File::open(&self.lock_path).ok()?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).ok()?;
        
        contents.trim().parse::<i32>().ok().map(Pid::from_raw)
    }
    
    fn is_process_alive(pid: Pid) -> bool {
        nix::sys::signal::kill(pid, None).is_ok()
    }
}

impl LockManager for FileLockManager {
    fn acquire(&self) -> Result<LockGuard> {
        self.ensure_lock_file_exists()?;
        
        let start = Instant::now();
        let mut first_attempt = true;
        
        loop {
            if !first_attempt && self.lock_path.exists() {
                if let Some(holder_pid) = self.read_lock_holder() {
                    if !Self::is_process_alive(holder_pid) {
                        tracing::warn!(
                            "Lock held by dead process (PID {}), cleaning up",
                            holder_pid
                        );
                        let _ = std::fs::remove_file(&self.lock_path);
                        self.ensure_lock_file_exists()?;
                    }
                }
            }
            first_attempt = false;
            
            let file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .open(&self.lock_path)
                .context("Failed to open lock file")?;
            
            let fd = file.as_raw_fd();
            
            match flock(fd, FlockArg::LockExclusiveNonblock) {
                Ok(_) => {
                    let pid = nix::unistd::getpid();
                    let mut file = file;
                    file.set_len(0).context("Failed to truncate lock file")?;
                    write!(file, "{}", pid).context("Failed to write PID to lock file")?;
                    file.sync_all().context("Failed to sync lock file")?;
                    
                    tracing::debug!("Acquired lock with PID {}", pid);
                    
                    return Ok(LockGuard {
                        file: Some(file),
                        path: self.lock_path.clone(),
                    });
                }
                Err(_) => {
                    if start.elapsed() >= LOCK_TIMEOUT {
                        let holder = self.read_lock_holder();
                        
                        if let Some(holder_pid) = holder {
                            return Err(RatpmError::LockHeld(holder_pid.to_string()).into());
                        } else {
                            return Err(RatpmError::LockTimeout.into());
                        }
                    }
                    
                    std::thread::sleep(LOCK_POLL_INTERVAL);
                }
            }
        }
    }
}

pub struct LockGuard {
    file: Option<File>,
    path: PathBuf,
}

impl Drop for LockGuard {
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            let fd = file.as_raw_fd();
            let _ = flock(fd, FlockArg::Unlock);
            tracing::debug!("Released lock");
        }
    }
}
