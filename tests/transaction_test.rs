use ratpm::backend::fedora::types::PackageSpec;
use ratpm::core::transaction::{Transaction, TransactionProgress, TransactionState};

#[test]
fn test_empty_transaction() {
    let transaction = Transaction::new();
    assert!(transaction.is_empty());
    assert_eq!(transaction.total_packages(), 0);
    assert_eq!(transaction.download_size, 0);
}

#[test]
fn test_add_install() {
    let mut transaction = Transaction::new();

    let pkg = PackageSpec::new(
        "vim".to_string(),
        "9.0.0".to_string(),
        "x86_64".to_string(),
        "fedora".to_string(),
    );

    transaction.add_install(pkg, 5_000_000);

    assert!(!transaction.is_empty());
    assert_eq!(transaction.total_packages(), 1);
    assert_eq!(transaction.download_size, 5_000_000);
    assert_eq!(transaction.install_size, 5_000_000);
}

#[test]
fn test_add_remove() {
    let mut transaction = Transaction::new();

    let pkg = PackageSpec::new(
        "vim".to_string(),
        "9.0.0".to_string(),
        "x86_64".to_string(),
        "@System".to_string(),
    );

    transaction.add_remove(pkg, 5_000_000);

    assert!(!transaction.is_empty());
    assert_eq!(transaction.total_packages(), 1);
    assert_eq!(transaction.install_size, -5_000_000);
}

#[test]
fn test_add_upgrade() {
    let mut transaction = Transaction::new();

    let old_pkg = PackageSpec::new(
        "vim".to_string(),
        "8.2.0".to_string(),
        "x86_64".to_string(),
        "@System".to_string(),
    );

    let new_pkg = PackageSpec::new(
        "vim".to_string(),
        "9.0.0".to_string(),
        "x86_64".to_string(),
        "fedora".to_string(),
    );

    transaction.add_upgrade(old_pkg, new_pkg, 4_500_000, 5_000_000);

    assert!(!transaction.is_empty());
    assert_eq!(transaction.total_packages(), 1);
    assert_eq!(transaction.download_size, 5_000_000);
    assert_eq!(transaction.install_size, 500_000);
}

#[test]
fn test_transaction_progress() {
    let mut progress = TransactionProgress::new(10, 50_000_000);

    assert_eq!(progress.state, TransactionState::Pending);
    assert_eq!(progress.total_packages, 10);
    assert_eq!(progress.total_bytes, 50_000_000);
    assert_eq!(progress.percentage(), 0.0);

    progress.packages_processed = 5;
    assert_eq!(progress.percentage(), 50.0);

    progress.packages_processed = 10;
    assert_eq!(progress.percentage(), 100.0);
}

#[test]
fn test_transaction_with_multiple_operations() {
    let mut transaction = Transaction::new();

    let install1 = PackageSpec::new(
        "vim".to_string(),
        "9.0.0".to_string(),
        "x86_64".to_string(),
        "fedora".to_string(),
    );

    let install2 = PackageSpec::new(
        "emacs".to_string(),
        "29.1".to_string(),
        "x86_64".to_string(),
        "fedora".to_string(),
    );

    let remove = PackageSpec::new(
        "nano".to_string(),
        "7.2".to_string(),
        "x86_64".to_string(),
        "@System".to_string(),
    );

    transaction.add_install(install1, 5_000_000);
    transaction.add_install(install2, 10_000_000);
    transaction.add_remove(remove, 500_000);

    assert_eq!(transaction.total_packages(), 3);
    assert_eq!(transaction.install.len(), 2);
    assert_eq!(transaction.remove.len(), 1);
    assert_eq!(transaction.download_size, 15_000_000);
    assert_eq!(transaction.install_size, 14_500_000);
}

#[test]
fn test_package_spec_nevra() {
    let spec = PackageSpec::new(
        "vim".to_string(),
        "9.0.0".to_string(),
        "x86_64".to_string(),
        "fedora".to_string(),
    );

    assert_eq!(spec.to_nevra(), "vim-9.0.0.x86_64");
}
