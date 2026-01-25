use anyhow::Result;
use crate::core::transaction::Transaction;
use crate::backend::fedora::types::PackageSpec;

pub trait Resolver {
    fn resolve_install(&self, packages: &[String]) -> Result<Transaction>;
    fn resolve_remove(&self, packages: &[String]) -> Result<Transaction>;
    fn resolve_upgrade(&self) -> Result<Transaction>;
    fn resolve_upgrade_packages(&self, packages: &[String]) -> Result<Transaction>;
}

pub struct DependencyGraph {
    nodes: Vec<GraphNode>,
}

#[derive(Debug, Clone)]
struct GraphNode {
    package: PackageSpec,
    dependencies: Vec<usize>,
    dependents: Vec<usize>,
}

impl DependencyGraph {
    pub fn new() -> Self {
        Self { nodes: Vec::new() }
    }
    
    pub fn add_package(&mut self, package: PackageSpec) -> usize {
        let index = self.nodes.len();
        self.nodes.push(GraphNode {
            package,
            dependencies: Vec::new(),
            dependents: Vec::new(),
        });
        index
    }
    
    pub fn add_dependency(&mut self, dependent: usize, dependency: usize) {
        if dependent < self.nodes.len() && dependency < self.nodes.len() {
            self.nodes[dependent].dependencies.push(dependency);
            self.nodes[dependency].dependents.push(dependent);
        }
    }
    
    pub fn topological_sort(&self) -> Result<Vec<PackageSpec>> {
        let mut in_degree = vec![0usize; self.nodes.len()];
        
        for node in &self.nodes {
            for &dep in &node.dependencies {
                in_degree[dep] += 1;
            }
        }
        
        let mut queue: Vec<usize> = in_degree
            .iter()
            .enumerate()
            .filter_map(|(i, &deg)| if deg == 0 { Some(i) } else { None })
            .collect();
        
        let mut sorted = Vec::new();
        
        while let Some(idx) = queue.pop() {
            sorted.push(self.nodes[idx].package.clone());
            
            for &dependent in &self.nodes[idx].dependents {
                in_degree[dependent] -= 1;
                if in_degree[dependent] == 0 {
                    queue.push(dependent);
                }
            }
        }
        
        if sorted.len() != self.nodes.len() {
            anyhow::bail!("Circular dependency detected");
        }
        
        Ok(sorted)
    }
    
    pub fn find_conflicts(&self) -> Vec<String> {
        let mut conflicts = Vec::new();
        
        for (i, node) in self.nodes.iter().enumerate() {
            for (j, other) in self.nodes.iter().enumerate().skip(i + 1) {
                if node.package.name == other.package.name 
                    && node.package.version != other.package.version {
                    conflicts.push(format!(
                        "Version conflict: {} ({} vs {})",
                        node.package.name,
                        node.package.version,
                        other.package.version
                    ));
                }
            }
        }
        
        conflicts
    }
}

impl Default for DependencyGraph {
    fn default() -> Self {
        Self::new()
    }
}
