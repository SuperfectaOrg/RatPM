#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

#[cfg(feature = "use_system_libdnf5")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(not(feature = "use_system_libdnf5"))]
mod mock {
    use std::os::raw::{c_char, c_int, c_void};
    
    pub type dnf5_base = c_void;
    pub type dnf5_repo = c_void;
    pub type dnf5_package = c_void;
    pub type dnf5_transaction = c_void;
    pub type dnf5_query = c_void;
    
    pub const DNF5_OK: c_int = 0;
    pub const DNF5_ERROR: c_int = 1;
    pub const DNF5_ERROR_REPO: c_int = 2;
    pub const DNF5_ERROR_PACKAGE: c_int = 3;
    pub const DNF5_ERROR_TRANSACTION: c_int = 4;
    
    extern "C" {
        pub fn dnf5_base_new() -> *mut dnf5_base;
        pub fn dnf5_base_free(base: *mut dnf5_base);
        pub fn dnf5_base_setup(base: *mut dnf5_base) -> c_int;
        pub fn dnf5_base_load_repos(base: *mut dnf5_base) -> c_int;
        
        pub fn dnf5_repo_new(base: *mut dnf5_base, id: *const c_char) -> *mut dnf5_repo;
        pub fn dnf5_repo_free(repo: *mut dnf5_repo);
        pub fn dnf5_repo_set_baseurl(repo: *mut dnf5_repo, url: *const c_char) -> c_int;
        pub fn dnf5_repo_enable(repo: *mut dnf5_repo) -> c_int;
        pub fn dnf5_repo_load(repo: *mut dnf5_repo) -> c_int;
        
        pub fn dnf5_query_new(base: *mut dnf5_base) -> *mut dnf5_query;
        pub fn dnf5_query_free(query: *mut dnf5_query);
        pub fn dnf5_query_filter_name(query: *mut dnf5_query, name: *const c_char) -> c_int;
        pub fn dnf5_query_filter_installed(query: *mut dnf5_query, installed: c_int) -> c_int;
        pub fn dnf5_query_size(query: *mut dnf5_query) -> usize;
        pub fn dnf5_query_get(query: *mut dnf5_query, index: usize) -> *mut dnf5_package;
        
        pub fn dnf5_package_get_name(pkg: *mut dnf5_package) -> *const c_char;
        pub fn dnf5_package_get_version(pkg: *mut dnf5_package) -> *const c_char;
        pub fn dnf5_package_get_arch(pkg: *mut dnf5_package) -> *const c_char;
        pub fn dnf5_package_get_summary(pkg: *mut dnf5_package) -> *const c_char;
        pub fn dnf5_package_get_download_size(pkg: *mut dnf5_package) -> u64;
        pub fn dnf5_package_get_install_size(pkg: *mut dnf5_package) -> u64;
        
        pub fn dnf5_transaction_new(base: *mut dnf5_base) -> *mut dnf5_transaction;
        pub fn dnf5_transaction_free(trans: *mut dnf5_transaction);
        pub fn dnf5_transaction_add_install(trans: *mut dnf5_transaction, pkg: *mut dnf5_package) -> c_int;
        pub fn dnf5_transaction_add_remove(trans: *mut dnf5_transaction, pkg: *mut dnf5_package) -> c_int;
        pub fn dnf5_transaction_add_upgrade(trans: *mut dnf5_transaction, pkg: *mut dnf5_package) -> c_int;
        pub fn dnf5_transaction_resolve(trans: *mut dnf5_transaction) -> c_int;
        pub fn dnf5_transaction_download(trans: *mut dnf5_transaction) -> c_int;
        pub fn dnf5_transaction_test(trans: *mut dnf5_transaction) -> c_int;
        pub fn dnf5_transaction_run(trans: *mut dnf5_transaction) -> c_int;
    }
}

#[cfg(not(feature = "use_system_libdnf5"))]
pub use mock::*;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_constants() {
        assert_eq!(DNF5_OK, 0);
        assert_eq!(DNF5_ERROR, 1);
    }
}
