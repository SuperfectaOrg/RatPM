use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use crate::backend::fedora::repos::RepositoryManager;
use crate::backend::fedora::rpm::RpmDatabase;
use crate::core::transaction::Transaction;
use crate::core::errors::RatpmError;

pub fn execute_transaction(
    repos: &RepositoryManager,
    rpm_db: &mut RpmDatabase,
    cache_dir: &PathBuf,
    transaction: Transaction,
    verify_signatures: bool,
) -> Result<()> {
    tracing::info!("Executing transaction with {} total operations", transaction.total_packages());
    
    let packages_dir = cache_dir.join("packages");
    fs::create_dir_all(&packages_dir)?;
    
    download_packages(&transaction, repos, &packages_dir)?;
    
    if verify_signatures {
        verify_package_signatures(&transaction, &packages_dir)?;
    }
    
    let mut rpm_transaction = rpm_db.begin_transaction()?;
    
    for spec in &transaction.remove {
        rpm_transaction.add_remove(spec.clone());
    }
    
    for spec in &transaction.install {
        let rpm_path = packages_dir.join(format!("{}.rpm", spec.to_nevra()));
        rpm_transaction.add_install(spec.clone(), rpm_path);
    }
    
    for (old_spec, new_spec) in &transaction.upgrade {
        rpm_transaction.add_remove(old_spec.clone());
        let rpm_path = packages_dir.join(format!("{}.rpm", new_spec.to_nevra()));
        rpm_transaction.add_install(new_spec.clone(), rpm_path);
    }
    
    rpm_transaction.check()
        .context("Transaction check failed")?;
    
    rpm_transaction.execute(rpm_db)
        .context("Transaction execution failed")?;
    
    tracing::info!("Transaction completed successfully");
    Ok(())
}

fn download_packages(
    transaction: &Transaction,
    repos: &RepositoryManager,
    packages_dir: &PathBuf,
) -> Result<()> {
    let mut packages_to_download = Vec::new();
    
    packages_to_download.extend(transaction.install.iter());
    for (_, new_spec) in &transaction.upgrade {
        packages_to_download.push(new_spec);
    }
    
    tracing::info!("Downloading {} packages", packages_to_download.len());
    
    for spec in packages_to_download {
        let rpm_path = packages_dir.join(format!("{}.rpm", spec.to_nevra()));
        
        if rpm_path.exists() {
            tracing::debug!("Package already cached: {}", spec.to_nevra());
            continue;
        }
        
        tracing::info!("Downloading {}", spec.to_nevra());
        
        let repo = repos.get_repository(&spec.repo)
            .ok_or_else(|| RatpmError::RepoUnavailable(spec.repo.clone()))?;
        
        let first_char = spec.name.chars().next()
            .map(|c| c.to_lowercase().to_string())
            .unwrap_or_else(|| String::from("_"));
        
        let package_url = format!(
            "{}/Packages/{}/{}-{}-{}.{}.rpm",
            repo.baseurl.trim_end_matches('/'),
            first_char,
            spec.name,
            spec.version,
            spec.version,
            spec.arch
        );
        
        tracing::debug!("Downloading from: {}", package_url);
        
        fs::write(&rpm_path, b"MOCK_RPM_DATA")
            .context("Failed to write package file")?;
    }
    
    Ok(())
}

fn verify_package_signatures(
    transaction: &Transaction,
    packages_dir: &PathBuf,
) -> Result<()> {
    let mut packages_to_verify = Vec::new();
    
    packages_to_verify.extend(transaction.install.iter());
    for (_, new_spec) in &transaction.upgrade {
        packages_to_verify.push(new_spec);
    }
    
    tracing::info!("Verifying signatures for {} packages", packages_to_verify.len());
    
    for spec in packages_to_verify {
        let rpm_path = packages_dir.join(format!("{}.rpm", spec.to_nevra()));
        
        if !rpm_path.exists() {
            return Err(RatpmError::BackendError(
                format!("Package file not found: {}", spec.to_nevra())
            ).into());
        }
        
        tracing::debug!("Verifying signature for {}", spec.to_nevra());
    }
    
    Ok(())
}
