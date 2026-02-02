use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ratpm::backend::fedora::types::PackageSpec;
use ratpm::core::transaction::Transaction;

fn bench_transaction_creation(c: &mut Criterion) {
    c.bench_function("transaction_new", |b| b.iter(Transaction::new));
}

fn bench_transaction_add_install(c: &mut Criterion) {
    c.bench_function("transaction_add_install", |b| {
        b.iter(|| {
            let mut transaction = Transaction::new();
            for i in 0..100 {
                let spec = PackageSpec::new(
                    format!("package-{}", i),
                    "1.0.0".to_string(),
                    "x86_64".to_string(),
                    "fedora".to_string(),
                );
                transaction.add_install(spec, 1_000_000);
            }
            black_box(transaction)
        })
    });
}

fn bench_transaction_operations(c: &mut Criterion) {
    c.bench_function("transaction_mixed_operations", |b| {
        b.iter(|| {
            let mut transaction = Transaction::new();

            for i in 0..50 {
                let spec = PackageSpec::new(
                    format!("install-{}", i),
                    "1.0.0".to_string(),
                    "x86_64".to_string(),
                    "fedora".to_string(),
                );
                transaction.add_install(spec, 1_000_000);
            }

            for i in 0..30 {
                let spec = PackageSpec::new(
                    format!("remove-{}", i),
                    "1.0.0".to_string(),
                    "x86_64".to_string(),
                    "@System".to_string(),
                );
                transaction.add_remove(spec, 500_000);
            }

            for i in 0..20 {
                let old_spec = PackageSpec::new(
                    format!("upgrade-{}", i),
                    "1.0.0".to_string(),
                    "x86_64".to_string(),
                    "@System".to_string(),
                );
                let new_spec = PackageSpec::new(
                    format!("upgrade-{}", i),
                    "2.0.0".to_string(),
                    "x86_64".to_string(),
                    "fedora".to_string(),
                );
                transaction.add_upgrade(old_spec, new_spec, 500_000, 600_000);
            }

            black_box(transaction)
        })
    });
}

criterion_group!(
    benches,
    bench_transaction_creation,
    bench_transaction_add_install,
    bench_transaction_operations
);
criterion_main!(benches);
