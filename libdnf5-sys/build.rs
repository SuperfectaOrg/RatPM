use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");

    if pkg_config::probe_library("libdnf5").is_err() {
        eprintln!("WARNING: libdnf5 not found via pkg-config");
        eprintln!("Attempting to find libdnf5 manually...");

        let possible_paths = vec![
            "/usr/lib64",
            "/usr/lib",
            "/usr/local/lib64",
            "/usr/local/lib",
        ];

        for path in possible_paths {
            println!("cargo:rustc-link-search=native={}", path);
        }
    }

    println!("cargo:rustc-link-lib=dnf5");
    println!("cargo:rustc-link-lib=dnf5-base");
    println!("cargo:rustc-link-lib=dnf5-repo");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .allowlist_function("dnf5_.*")
        .allowlist_type("dnf5_.*")
        .allowlist_var("DNF5_.*")
        .prepend_enum_name(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings");
}
