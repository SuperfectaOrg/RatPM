use criterion::{black_box, criterion_group, criterion_main, Criterion};
use ratpm::config::Config;

fn bench_config_default(c: &mut Criterion) {
    c.bench_function("config_default", |b| {
        b.iter(|| black_box(Config::default()))
    });
}

fn bench_config_parse(c: &mut Criterion) {
    let config_str = r#"
[system]
backend = "fedora"
assume_yes = false
color = true
cache_dir = "/var/cache/ratpm"
lock_file = "/var/lock/ratpm.lock"

[repos]
auto_refresh = true
metadata_expire = 86400
repo_dir = "/etc/yum.repos.d"
gpgcheck = true

[transaction]
keep_cache = true
history_limit = 100
verify_signatures = true
"#;

    c.bench_function("config_parse", |b| {
        b.iter(|| {
            let config: Config = toml::from_str(config_str).unwrap();
            black_box(config)
        })
    });
}

criterion_group!(benches, bench_config_default, bench_config_parse);
criterion_main!(benches);
