use anyhow::{Context, Result};
use crate::backend::fedora::repos::RepositoryManager;
use crate::backend::fedora::rpm::RpmDatabase;
use crate::backend::fedora::types::PackageSpec;
use crate::core::transaction::Transaction;
use crate::core::errors::RatpmError;

pub fn resolve_install(
    repos: &RepositoryManager,
    _rpm_db: &RpmDatabase,
    packages: &[String],
) -> Result<Transaction> {
    tracing::info!("Resolving installation for {} packages", packages.len());
    
    let mut transaction = Transaction::new();
    let mut added_packages = std::collections::HashSet::new();
    
    for pkg_name in packages {
        let pkg_info = repos.get_package_info(pkg_name)
            .context(format!("Package '{}' not found in any repository", pkg_name))?;
        
        let spec = PackageSpec::new(
            pkg_info.name.clone(),
            pkg_info.version.clone(),
            pkg_info.arch.clone(),
            pkg_info.repo.clone(),
        );
        
        if added_packages.insert(spec.clone()) {
            transaction.add_install(spec.clone(), pkg_info.size);
        }
        
        let dependencies = resolve_dependencies(&spec, repos, _rpm_db)?;
        for dep in dependencies {
            if !_rpm_db.is_installed(&dep.name)? && added_packages.insert(dep.clone()) {
                let dep_info = repos.get_package_info(&dep.name)?;
                transaction.add_install(dep, dep_info.size);
            }
        }
    }
    
    Ok(transaction)
}

pub fn resolve_remove(
    rpm_db: &RpmDatabase,
    packages: &[String],
) -> Result<Transaction> {
    tracing::info!("Resolving removal for {} packages", packages.len());
    
    let mut transaction = Transaction::new();
    
    for pkg_name in packages {
        let pkg_info = rpm_db.get_package_info(pkg_name)?;
        
        let spec = PackageSpec::new(
            pkg_info.name.clone(),
            pkg_info.version.clone(),
            pkg_info.arch.clone(),
            "@System".to_string(),
        );
        
        transaction.add_remove(spec, pkg_info.size);
        
        let dependents = find_dependents(&pkg_info.name, rpm_db)?;
        if !dependents.is_empty() {
            let dep_list = dependents.join(", ");
            return Err(RatpmError::DependencyConflict(
                format!(
                    "Cannot remove '{}': required by {}",
                    pkg_name, dep_list
                )
            ).into());
        }
    }
    
    Ok(transaction)
}

pub fn resolve_upgrade(
    repos: &RepositoryManager,
    rpm_db: &RpmDatabase,
) -> Result<Transaction> {
    tracing::info!("Resolving system upgrade");
    
    let mut transaction = Transaction::new();
    
    let installed = rpm_db.list_all()?;
    
    for installed_pkg in installed {
        if let Ok(available_pkg) = repos.get_package_info(&installed_pkg.name) {
            if version_compare(&available_pkg.version, &installed_pkg.version) > 0 {
                let old_spec = PackageSpec::new(
                    installed_pkg.name.clone(),
                    installed_pkg.version.clone(),
                    installed_pkg.arch.clone(),
                    "@System".to_string(),
                );
                
                let new_spec = PackageSpec::new(
                    available_pkg.name.clone(),
                    available_pkg.version.clone(),
                    available_pkg.arch.clone(),
                    available_pkg.repo.clone(),
                );
                
                let old_size = rpm_db.get_package_info(&installed_pkg.name)
                    .map(|info| info.size)
                    .unwrap_or(0);
                
                transaction.add_upgrade(old_spec, new_spec, old_size, available_pkg.size);
            }
        }
    }
    
    Ok(transaction)
}

pub fn resolve_upgrade_packages(
    repos: &RepositoryManager,
    rpm_db: &RpmDatabase,
    packages: &[String],
) -> Result<Transaction> {
    tracing::info!("Resolving upgrade for specific packages");
    
    let mut transaction = Transaction::new();
    
    for pkg_name in packages {
        if !rpm_db.is_installed(pkg_name)? {
            return Err(RatpmError::PackageNotInstalled(pkg_name.clone()).into());
        }
        
        let installed_pkg = rpm_db.get_package_info(pkg_name)?;
        let available_pkg = repos.get_package_info(pkg_name)?;
        
        if version_compare(&available_pkg.version, &installed_pkg.version) > 0 {
            let old_spec = PackageSpec::new(
                installed_pkg.name.clone(),
                installed_pkg.version.clone(),
                installed_pkg.arch.clone(),
                "@System".to_string(),
            );
            
            let new_spec = PackageSpec::new(
                available_pkg.name.clone(),
                available_pkg.version.clone(),
                available_pkg.arch.clone(),
                available_pkg.repo.clone(),
            );
            
            transaction.add_upgrade(old_spec, new_spec, installed_pkg.size, available_pkg.size);
        }
    }
    
    Ok(transaction)
}

fn resolve_dependencies(
    _package: &PackageSpec,
    _repos: &RepositoryManager,
    _rpm_db: &RpmDatabase,
) -> Result<Vec<PackageSpec>> {
    let dependencies = Vec::new();
    
    Ok(dependencies)
}

fn find_dependents(
    _package: &str,
    _rpm_db: &RpmDatabase,
) -> Result<Vec<String>> {
    let dependents = Vec::new();
    
    Ok(dependents)
}

fn version_compare(v1: &str, v2: &str) -> i32 {
    let v1_parts: Vec<&str> = v1.split('.').collect();
    let v2_parts: Vec<&str> = v2.split('.').collect();
    
    let max_len = v1_parts.len().max(v2_parts.len());
    
    for i in 0..max_len {
        let p1 = v1_parts.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        let p2 = v2_parts.get(i).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
        
        if p1 > p2 {
            return 1;
        } else if p1 < p2 {
            return -1;
        }
    }
    
    0
}
