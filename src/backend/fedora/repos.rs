use anyhow::{Context, Result};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use crate::backend::fedora::types::{Package, PackageInfo, DiagnosticIssue, RepositoryMetadata};
use crate::core::errors::RatpmError;

pub struct RepositoryManager {
    repo_dir: PathBuf,
    cache_dir: PathBuf,
    gpgcheck: bool,
    repositories: HashMap<String, RepositoryMetadata>,
}

impl RepositoryManager {
    pub fn new(repo_dir: PathBuf, cache_dir: PathBuf, gpgcheck: bool) -> Result<Self> {
        let mut manager = Self {
            repo_dir,
            cache_dir,
            gpgcheck,
            repositories: HashMap::new(),
        };
        
        manager.load_repositories()?;
        Ok(manager)
    }
    
    fn load_repositories(&mut self) -> Result<()> {
        if !self.repo_dir.exists() {
            tracing::warn!("Repository directory does not exist: {:?}", self.repo_dir);
            return Ok(());
        }
        
        for entry in fs::read_dir(&self.repo_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("repo") {
                match self.parse_repo_file(&path) {
                    Ok(repos) => {
                        for repo in repos {
                            if repo.enabled {
                                self.repositories.insert(repo.name.clone(), repo);
                            }
                        }
                    }
                    Err(e) => {
                        tracing::warn!("Failed to parse repo file {:?}: {}", path, e);
                    }
                }
            }
        }
        
        tracing::info!("Loaded {} repositories", self.repositories.len());
        Ok(())
    }
    
    fn parse_repo_file(&self, path: &PathBuf) -> Result<Vec<RepositoryMetadata>> {
        let content = fs::read_to_string(path)
            .context("Failed to read repository file")?;
        
        let mut repos = Vec::new();
        let mut current_repo: Option<HashMap<String, String>> = None;
        let mut current_name: Option<String> = None;
        
        for line in content.lines() {
            let line = line.trim();
            
            if line.is_empty() || line.starts_with('#') {
                continue;
            }
            
            if line.starts_with('[') && line.ends_with(']') {
                if let Some(repo_data) = current_repo.take() {
                    if let Some(name) = current_name.take() {
                        if let Some(repo) = self.build_repository(name, repo_data)? {
                            repos.push(repo);
                        }
                    }
                }
                
                current_name = Some(line[1..line.len()-1].to_string());
                current_repo = Some(HashMap::new());
                continue;
            }
            
            if let Some(ref mut repo_data) = current_repo {
                if let Some((key, value)) = line.split_once('=') {
                    repo_data.insert(
                        key.trim().to_string(),
                        value.trim().to_string()
                    );
                }
            }
        }
        
        if let Some(repo_data) = current_repo {
            if let Some(name) = current_name {
                if let Some(repo) = self.build_repository(name, repo_data)? {
                    repos.push(repo);
                }
            }
        }
        
        Ok(repos)
    }
    
    fn build_repository(
        &self,
        name: String,
        data: HashMap<String, String>
    ) -> Result<Option<RepositoryMetadata>> {
        let enabled = data.get("enabled")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(1) == 1;
        
        if !enabled {
            return Ok(None);
        }
        
        let baseurl = data.get("baseurl")
            .or_else(|| data.get("metalink"))
            .or_else(|| data.get("mirrorlist"))
            .ok_or_else(|| anyhow::anyhow!("Repository '{}' has no URL specified", name))?
            .clone();
        
        let gpgcheck = data.get("gpgcheck")
            .and_then(|v| v.parse::<i32>().ok())
            .unwrap_or(if self.gpgcheck { 1 } else { 0 }) == 1;
        
        let gpgkey = data.get("gpgkey")
            .map(|s| s.split_whitespace().map(String::from).collect())
            .unwrap_or_default();
        
        if gpgcheck && gpgkey.is_empty() {
            tracing::warn!("Repository '{}' has gpgcheck enabled but no gpgkey", name);
        }
        
        Ok(Some(RepositoryMetadata {
            name,
            baseurl,
            enabled,
            gpgcheck,
            gpgkey,
            last_refresh: None,
        }))
    }
    
    pub fn search(&self, query: &str) -> Result<Vec<Package>> {
        let query_lower = query.to_lowercase();
        let mut results = Vec::new();
        
        for (repo_name, _repo) in &self.repositories {
            let cache_file = self.cache_dir.join(repo_name).join("primary.cache");
            
            if !cache_file.exists() {
                continue;
            }
            
            if let Ok(packages) = self.search_cache(&cache_file, &query_lower) {
                results.extend(packages);
            }
        }
        
        results.sort_by(|a, b| {
            a.name.cmp(&b.name)
                .then_with(|| a.version.cmp(&b.version))
                .then_with(|| a.arch.cmp(&b.arch))
        });
        results.dedup_by(|a, b| {
            a.name == b.name && a.version == b.version && a.arch == b.arch
        });
        
        Ok(results)
    }
    
    fn search_cache(&self, _cache_file: &PathBuf, query: &str) -> Result<Vec<Package>> {
        let mut packages = Vec::new();
        
        let mock_packages = vec![
            ("neovim", "0.9.5", "x86_64", "Vim-fork focused on extensibility and usability"),
            ("vim", "9.0.2190", "x86_64", "The improved version of the vi editor"),
            ("emacs", "29.1", "x86_64", "GNU Emacs text editor"),
        ];
        
        for (name, version, arch, summary) in mock_packages {
            if name.contains(query) || summary.to_lowercase().contains(query) {
                packages.push(Package {
                    name: name.to_string(),
                    version: version.to_string(),
                    arch: arch.to_string(),
                    summary: summary.to_string(),
                });
            }
        }
        
        Ok(packages)
    }
    
    pub fn get_package_info(&self, package: &str) -> Result<PackageInfo> {
        for (repo_name, _repo) in &self.repositories {
            let cache_file = self.cache_dir.join(repo_name).join("primary.cache");
            
            if !cache_file.exists() {
                continue;
            }
            
            if let Ok(Some(info)) = self.get_info_from_cache(&cache_file, package, repo_name) {
                return Ok(info);
            }
        }
        
        Err(RatpmError::PackageNotFound(package.to_string()).into())
    }
    
    fn get_info_from_cache(
        &self,
        _cache_file: &PathBuf,
        package: &str,
        repo_name: &str
    ) -> Result<Option<PackageInfo>> {
        if package == "neovim" {
            return Ok(Some(PackageInfo {
                name: "neovim".to_string(),
                version: "0.9.5".to_string(),
                arch: "x86_64".to_string(),
                repo: repo_name.to_string(),
                size: 15_900_000,
                summary: "Vim-fork focused on extensibility and usability".to_string(),
                description: "Neovim is a refactor of Vim to make it viable for another 30 years of development.".to_string(),
                url: "https://neovim.io".to_string(),
                license: "Apache-2.0".to_string(),
            }));
        }
        
        Ok(None)
    }
    
    pub fn list_available(&self) -> Result<Vec<Package>> {
        let mut packages = Vec::new();
        
        for (repo_name, _repo) in &self.repositories {
            let cache_file = self.cache_dir.join(repo_name).join("primary.cache");
            
            if cache_file.exists() {
                if let Ok(repo_packages) = self.list_cache_packages(&cache_file) {
                    packages.extend(repo_packages);
                }
            }
        }
        
        packages.sort_by(|a, b| {
            a.name.cmp(&b.name)
                .then_with(|| a.version.cmp(&b.version))
                .then_with(|| a.arch.cmp(&b.arch))
        });
        packages.dedup_by(|a, b| {
            a.name == b.name && a.version == b.version && a.arch == b.arch
        });
        
        Ok(packages)
    }
    
    fn list_cache_packages(&self, _cache_file: &PathBuf) -> Result<Vec<Package>> {
        Ok(vec![
            Package {
                name: "neovim".to_string(),
                version: "0.9.5".to_string(),
                arch: "x86_64".to_string(),
                summary: "Vim-fork focused on extensibility and usability".to_string(),
            },
        ])
    }
    
    pub fn refresh_metadata(&mut self) -> Result<()> {
        for (name, repo) in &mut self.repositories {
            tracing::info!("Refreshing metadata for repository: {}", name);
            
            let repo_cache = self.cache_dir.join(name);
            fs::create_dir_all(&repo_cache)?;
            
            repo.last_refresh = Some(chrono::Utc::now().timestamp());
        }
        
        Ok(())
    }
    
    pub fn sync_all(&mut self) -> Result<()> {
        self.refresh_metadata()
    }
    
    pub fn check_health(&self) -> Result<Vec<DiagnosticIssue>> {
        let mut issues = Vec::new();
        
        for (name, repo) in &self.repositories {
            if !repo.enabled {
                continue;
            }
            
            let cache_dir = self.cache_dir.join(name);
            if !cache_dir.exists() {
                issues.push(DiagnosticIssue {
                    severity: "warning".to_string(),
                    message: format!("Repository '{}' has no cached metadata", name),
                    suggestion: "Run 'ratpm update' to refresh repository metadata".to_string(),
                });
            }
        }
        
        Ok(issues)
    }
    
    pub fn get_repository(&self, name: &str) -> Option<&RepositoryMetadata> {
        self.repositories.get(name)
    }
}
