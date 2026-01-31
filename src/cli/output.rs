use std::io::{self, Write};
use crate::core::transaction::Transaction;
use crate::backend::fedora::types::{Package, PackageInfo, DiagnosticIssue, HistoryEntry};

const COLOR_RESET: &str = "\x1b[0m";
const COLOR_GREEN: &str = "\x1b[32m";
const COLOR_RED: &str = "\x1b[31m";
const COLOR_YELLOW: &str = "\x1b[33m";
const COLOR_BLUE: &str = "\x1b[34m";
const COLOR_BOLD: &str = "\x1b[1m";

fn format_size(bytes: u64) -> String {
    const KIB: u64 = 1024;
    const MIB: u64 = KIB * 1024;
    const GIB: u64 = MIB * 1024;

    if bytes >= GIB {
        format!("{:.1} GB", bytes as f64 / GIB as f64)
    } else if bytes >= MIB {
        format!("{:.1} MB", bytes as f64 / MIB as f64)
    } else if bytes >= KIB {
        format!("{:.1} KB", bytes as f64 / KIB as f64)
    } else {
        format!("{} B", bytes)
    }
}

pub fn info(message: &str) {
    println!("{}", message);
}

pub fn success(message: &str) {
    println!("{}", message);
}

pub fn error(message: &str) {
    eprintln!("Error: {}", message);
}

pub fn warning(message: &str) {
    eprintln!("Warning: {}", message);
}

pub fn print_transaction_summary(transaction: &Transaction, color: bool) {
    let mut stdout = io::stdout();
    
    if !transaction.install.is_empty() {
        if color {
            write!(stdout, "{}{}", COLOR_BOLD, COLOR_GREEN).unwrap();
        }
        writeln!(stdout, "Installing:").unwrap();
        if color {
            write!(stdout, "{}", COLOR_RESET).unwrap();
        }
        
        for pkg in &transaction.install {
            writeln!(stdout, "  {}-{}.{}", pkg.name, pkg.version, pkg.arch).unwrap();
        }
        writeln!(stdout).unwrap();
    }

    if !transaction.remove.is_empty() {
        if color {
            write!(stdout, "{}{}", COLOR_BOLD, COLOR_RED).unwrap();
        }
        writeln!(stdout, "Removing:").unwrap();
        if color {
            write!(stdout, "{}", COLOR_RESET).unwrap();
        }
        
        for pkg in &transaction.remove {
            writeln!(stdout, "  {}-{}.{}", pkg.name, pkg.version, pkg.arch).unwrap();
        }
        writeln!(stdout).unwrap();
    }

    if !transaction.upgrade.is_empty() {
        if color {
            write!(stdout, "{}{}", COLOR_BOLD, COLOR_BLUE).unwrap();
        }
        writeln!(stdout, "Upgrading:").unwrap();
        if color {
            write!(stdout, "{}", COLOR_RESET).unwrap();
        }
        
        for (old, new) in &transaction.upgrade {
            writeln!(
                stdout,
                "  {}: {}.{} -> {}.{}",
                old.name, old.version, old.arch, new.version, new.arch
            ).unwrap();
        }
        writeln!(stdout).unwrap();
    }

    writeln!(stdout, "Transaction Summary:").unwrap();
    writeln!(stdout, "  Install:  {} packages", transaction.install.len()).unwrap();
    writeln!(stdout, "  Remove:   {} packages", transaction.remove.len()).unwrap();
    writeln!(stdout, "  Upgrade:  {} packages", transaction.upgrade.len()).unwrap();
    writeln!(stdout, "  Download: {}", format_size(transaction.download_size)).unwrap();
    
    if transaction.install_size > 0 {
        writeln!(stdout, "  Disk space required: {}", format_size(transaction.install_size as u64)).unwrap();
    } else if transaction.install_size < 0 {
        writeln!(stdout, "  Disk space freed: {}", format_size(transaction.install_size.unsigned_abs())).unwrap();
    }
    
    writeln!(stdout).unwrap();
    
    stdout.flush().unwrap();
}

pub fn print_package_list(packages: &[Package], color: bool) {
    let mut stdout = io::stdout();
    
    for pkg in packages {
        if color {
            write!(stdout, "{}", COLOR_BOLD).unwrap();
        }
        write!(stdout, "{}", pkg.name).unwrap();
        if color {
            write!(stdout, "{}", COLOR_RESET).unwrap();
        }
        
        writeln!(stdout, "-{}.{}", pkg.version, pkg.arch).unwrap();
        
        if !pkg.summary.is_empty() {
            writeln!(stdout, "  {}", pkg.summary).unwrap();
        }
    }
    
    stdout.flush().unwrap();
}

pub fn print_package_info(info: &PackageInfo, color: bool) {
    let mut stdout = io::stdout();
    
    let print_field = |label: &str, value: &str| {
        if color {
            write!(stdout, "{}", COLOR_BOLD).unwrap();
        }
        write!(stdout, "{:12}: ", label).unwrap();
        if color {
            write!(stdout, "{}", COLOR_RESET).unwrap();
        }
        writeln!(stdout, "{}", value).unwrap();
    };

    print_field("Name", &info.name);
    print_field("Version", &info.version);
    print_field("Arch", &info.arch);
    print_field("Repository", &info.repo);
    print_field("Size", &format_size(info.size));
    print_field("Summary", &info.summary);
    
    if !info.description.is_empty() {
        writeln!(stdout).unwrap();
        writeln!(stdout, "Description:").unwrap();
        writeln!(stdout, "{}", info.description).unwrap();
    }
    
    if !info.url.is_empty() {
        writeln!(stdout).unwrap();
        print_field("URL", &info.url);
    }
    
    if !info.license.is_empty() {
        print_field("License", &info.license);
    }
    
    stdout.flush().unwrap();
}

pub fn print_diagnostic_issues(issues: &[DiagnosticIssue], color: bool) {
    let mut stdout = io::stdout();
    
    for issue in issues {
        if color {
            let color_code = match issue.severity.as_str() {
                "error" => COLOR_RED,
                "warning" => COLOR_YELLOW,
                _ => COLOR_BLUE,
            };
            write!(stdout, "{}{}", COLOR_BOLD, color_code).unwrap();
        }
        
        write!(stdout, "[{}]", issue.severity.to_uppercase()).unwrap();
        
        if color {
            write!(stdout, "{}", COLOR_RESET).unwrap();
        }
        
        writeln!(stdout, " {}", issue.message).unwrap();
        
        if !issue.suggestion.is_empty() {
            writeln!(stdout, "  Suggestion: {}", issue.suggestion).unwrap();
        }
        
        writeln!(stdout).unwrap();
    }
    
    stdout.flush().unwrap();
}

pub fn print_history(entries: &[HistoryEntry], color: bool) {
    let mut stdout = io::stdout();
    
    for entry in entries {
        if color {
            write!(stdout, "{}", COLOR_BOLD).unwrap();
        }
        write!(stdout, "ID {}", entry.id).unwrap();
        if color {
            write!(stdout, "{}", COLOR_RESET).unwrap();
        }
        
        writeln!(stdout, " | {} | {}", entry.timestamp, entry.command).unwrap();
        
        for action in &entry.actions {
            writeln!(stdout, "  {}", action).unwrap();
        }
        
        writeln!(stdout).unwrap();
    }
    
    stdout.flush().unwrap();
}

pub fn prompt_confirmation(message: &str) -> io::Result<bool> {
    print!("{} [y/N] ", message);
    io::stdout().flush()?;
    
    let mut response = String::new();
    io::stdin().read_line(&mut response)?;
    
    Ok(response.trim().eq_ignore_ascii_case("y"))
}
