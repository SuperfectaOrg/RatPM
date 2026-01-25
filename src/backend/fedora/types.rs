#[derive(Debug, Clone)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub repo: String,
    pub size: u64,
    pub summary: String,
    pub description: String,
    pub url: String,
    pub license: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PackageSpec {
    pub name: String,
    pub version: String,
    pub arch: String,
    pub repo: String,
}

impl PackageSpec {
    pub fn new(name: String, version: String, arch: String, repo: String) -> Self {
        Self {
            name,
            version,
            arch,
            repo,
        }
    }
    
    pub fn to_nevra(&self) -> String {
        format!("{}-{}.{}", self.name, self.version, self.arch)
    }
}

#[derive(Debug, Clone)]
pub struct DiagnosticIssue {
    pub severity: String,
    pub message: String,
    pub suggestion: String,
}

#[derive(Debug, Clone)]
pub struct HistoryEntry {
    pub id: i64,
    pub timestamp: String,
    pub command: String,
    pub actions: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct RepositoryMetadata {
    pub name: String,
    pub baseurl: String,
    pub enabled: bool,
    pub gpgcheck: bool,
    pub gpgkey: Vec<String>,
    pub last_refresh: Option<i64>,
}

#[derive(Debug, Clone)]
pub struct PackageDownload {
    pub spec: PackageSpec,
    pub url: String,
    pub size: u64,
    pub checksum: String,
    pub checksum_type: String,
}
