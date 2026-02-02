use ratpm::core::lock::{FileLockManager, LockManager};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tempfile::NamedTempFile;

#[test]
fn test_lock_acquire_and_release() {
    let temp_file = NamedTempFile::new().unwrap();
    let lock_manager = FileLockManager::new(temp_file.path().to_path_buf());

    let guard = lock_manager.acquire();
    assert!(guard.is_ok());

    drop(guard);
}

#[test]
fn test_lock_prevents_concurrent_access() {
    let temp_file = NamedTempFile::new().unwrap();
    let lock_path = temp_file.path().to_path_buf();
    let lock_manager = Arc::new(FileLockManager::new(lock_path.clone()));

    let _guard1 = lock_manager.acquire().unwrap();

    let lock_manager2 = Arc::clone(&lock_manager);
    let handle = thread::spawn(move || lock_manager2.acquire());

    thread::sleep(Duration::from_millis(100));

    let result = handle.join().unwrap();
    assert!(result.is_err());
}

#[test]
fn test_lock_released_on_drop() {
    let temp_file = NamedTempFile::new().unwrap();
    let lock_path = temp_file.path().to_path_buf();
    let lock_manager = FileLockManager::new(lock_path);

    {
        let _guard = lock_manager.acquire().unwrap();
    }

    let guard2 = lock_manager.acquire();
    assert!(guard2.is_ok());
}

#[test]
fn test_multiple_sequential_locks() {
    let temp_file = NamedTempFile::new().unwrap();
    let lock_manager = FileLockManager::new(temp_file.path().to_path_buf());

    for _ in 0..5 {
        let guard = lock_manager.acquire();
        assert!(guard.is_ok());
        drop(guard);
    }
}

#[test]
fn test_lock_across_threads() {
    let temp_file = NamedTempFile::new().unwrap();
    let lock_path = temp_file.path().to_path_buf();
    let lock_manager = Arc::new(FileLockManager::new(lock_path));

    let mut handles = vec![];

    for i in 0..3 {
        let lm = Arc::clone(&lock_manager);
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(i * 50));
            let _guard = lm.acquire().unwrap();
            thread::sleep(Duration::from_millis(10));
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}
