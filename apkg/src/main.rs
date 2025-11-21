// apkg - ALinux Universal Package Manager
// Pulls packages from AUR, Debian, Flatpak, Snap, etc. and converts to apkg format

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApkgPackage {
    pub name: String,
    pub version: String,
    pub description: String,
    pub source: PackageSource,
    pub dependencies: Vec<String>,
    pub build_type: BuildType,
    pub homepage: Option<String>,
    pub license: Vec<String>,
    pub maintainer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PackageSource {
    Aur { pkgbase: String, url: String },
    Debian { suite: String, component: String, arch: String },
    Flatpak { remote: String, ref_name: String },
    Snap { channel: String },
    Nixpkgs { attr: String },
    Source { url: String, vcs_type: Option<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BuildType {
    SourceBuild { build_cmd: Vec<String> },
    Binary { extract_cmd: Option<Vec<String>> },
    Container { runtime: String },
}

// AUR RPC v5 structures
#[derive(Debug, Deserialize)]
struct AurResponse {
    version: u32,
    #[serde(rename = "type")]
    response_type: String,
    resultcount: u32,
    results: Vec<AurPackage>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
struct AurPackage {
    #[serde(rename = "ID")]
    id: u64,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "PackageBase")]
    package_base: String,
    #[serde(rename = "Version")]
    version: String,
    #[serde(rename = "Description")]
    description: Option<String>,
    #[serde(rename = "URL")]
    url: Option<String>,
    #[serde(rename = "URLPath")]
    url_path: String,
    #[serde(rename = "Depends")]
    depends: Option<Vec<String>>,
    #[serde(rename = "MakeDepends")]
    makedepends: Option<Vec<String>>,
    #[serde(rename = "License")]
    license: Option<Vec<String>>,
    #[serde(rename = "Maintainer")]
    maintainer: Option<String>,
}

// Debian package structures
#[derive(Debug, Deserialize)]
struct DebianPackage {
    package: String,
    version: String,
    description: Option<String>,
    homepage: Option<String>,
    depends: Option<String>,
    maintainer: Option<String>,
}

// Flatpak structures
#[derive(Debug)]
struct FlatpakPackage {
    app_id: String,
    version: String,
    branch: String,
    remote: String,
    description: String,
}

pub struct PackageDatabase {
    packages: HashMap<String, ApkgPackage>,
    cache_dir: PathBuf,
}

impl PackageDatabase {
    pub fn new(cache_dir: PathBuf) -> Self {
        Self {
            packages: HashMap::new(),
            cache_dir,
        }
    }

    // AUR Integration
    pub async fn sync_aur(&mut self) -> Result<usize, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut count = 0;

        // Search for popular packages (we can't download all at once due to rate limits)
        let search_terms = vec!["rust", "gui", "terminal", "system"];
        
        for term in search_terms {
            let url = format!(
                "https://aur.archlinux.org/rpc/v5/search/{}",
                urlencoding::encode(term)
            );
            
            let response: AurResponse = client.get(&url).send().await?.json().await?;
            
            for pkg in response.results {
                let apkg = self.convert_aur_package(pkg);
                self.packages.insert(apkg.name.clone(), apkg);
                count += 1;
            }

            // Respect rate limits
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(count)
    }

    fn convert_aur_package(&self, pkg: AurPackage) -> ApkgPackage {
        let mut deps = pkg.depends.unwrap_or_default();
        deps.extend(pkg.makedepends.unwrap_or_default());

        ApkgPackage {
            name: pkg.name.clone(),
            version: pkg.version,
            description: pkg.description.unwrap_or_else(|| "No description".to_string()),
            source: PackageSource::Aur {
                pkgbase: pkg.package_base,
                url: format!("https://aur.archlinux.org{}", pkg.url_path),
            },
            dependencies: deps,
            build_type: BuildType::SourceBuild {
                build_cmd: vec![
                    "makepkg".to_string(),
                    "--noconfirm".to_string(),
                    "-si".to_string(),
                ],
            },
            homepage: pkg.url,
            license: pkg.license.unwrap_or_default(),
            maintainer: pkg.maintainer,
        }
    }

    // Debian Integration
    pub async fn sync_debian(&mut self, suite: &str) -> Result<usize, Box<dyn std::error::Error>> {
        let client = reqwest::Client::new();
        let mut count = 0;

        // Download package list from Debian
        let components = vec!["main", "contrib", "non-free"];
        
        for component in components {
            let url = format!(
                "https://deb.debian.org/debian/dists/{}/{}/binary-amd64/Packages.gz",
                suite, component
            );

            // In real implementation, would decompress and parse Packages file
            // For now, showing structure
            println!("Would sync from: {}", url);
            count += 1;
        }

        Ok(count)
    }

    fn parse_debian_packages(&mut self, content: &str, suite: &str, component: &str) {
        // Parse debian control format
        let mut current_pkg = HashMap::new();

        for line in content.lines() {
            if line.is_empty() {
                if let Some(name) = current_pkg.get("Package") {
                    let apkg = self.convert_debian_package(&current_pkg, suite, component);
                    self.packages.insert(name.to_string(), apkg);
                }
                current_pkg.clear();
            } else if let Some((key, value)) = line.split_once(": ") {
                current_pkg.insert(key.to_string(), value.to_string());
            }
        }
    }

    fn convert_debian_package(
        &self,
        pkg: &HashMap<String, String>,
        suite: &str,
        component: &str,
    ) -> ApkgPackage {
        let name = pkg.get("Package").unwrap().clone();
        let version = pkg.get("Version").unwrap_or(&"unknown".to_string()).clone();
        let description = pkg.get("Description").unwrap_or(&"".to_string()).clone();
        let depends = pkg.get("Depends")
            .map(|d| d.split(',').map(|s| s.trim().to_string()).collect())
            .unwrap_or_default();

        ApkgPackage {
            name: name.clone(),
            version,
            description,
            source: PackageSource::Debian {
                suite: suite.to_string(),
                component: component.to_string(),
                arch: "amd64".to_string(),
            },
            dependencies: depends,
            build_type: BuildType::Binary {
                extract_cmd: Some(vec!["dpkg-deb".to_string(), "-x".to_string()]),
            },
            homepage: pkg.get("Homepage").cloned(),
            license: vec![],
            maintainer: pkg.get("Maintainer").cloned(),
        }
    }

    // Flatpak Integration
    pub async fn sync_flatpak(&mut self, remote: &str) -> Result<usize, Box<dyn std::error::Error>> {
        // Execute flatpak remote-ls to get package list
        let output = std::process::Command::new("flatpak")
            .args(&["remote-ls", remote, "--app", "--columns=application,version,branch,description"])
            .output()?;

        if !output.status.success() {
            return Err("Flatpak command failed".into());
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut count = 0;

        for line in stdout.lines().skip(1) {
            let parts: Vec<&str> = line.split('\t').collect();
            if parts.len() >= 4 {
                let pkg = FlatpakPackage {
                    app_id: parts[0].to_string(),
                    version: parts[1].to_string(),
                    branch: parts[2].to_string(),
                    remote: remote.to_string(),
                    description: parts[3].to_string(),
                };

                let apkg = self.convert_flatpak_package(pkg);
                self.packages.insert(apkg.name.clone(), apkg);
                count += 1;
            }
        }

        Ok(count)
    }

    fn convert_flatpak_package(&self, pkg: FlatpakPackage) -> ApkgPackage {
        ApkgPackage {
            name: pkg.app_id.clone(),
            version: pkg.version,
            description: pkg.description,
            source: PackageSource::Flatpak {
                remote: pkg.remote,
                ref_name: format!("app/{}/{}", pkg.app_id, pkg.branch),
            },
            dependencies: vec![],
            build_type: BuildType::Container {
                runtime: "org.freedesktop.Platform".to_string(),
            },
            homepage: None,
            license: vec![],
            maintainer: None,
        }
    }

    // Search functionality
    pub fn search(&self, query: &str) -> Vec<&ApkgPackage> {
        let query_lower = query.to_lowercase();
        self.packages
            .values()
            .filter(|pkg| {
                pkg.name.to_lowercase().contains(&query_lower)
                    || pkg.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    // Save/Load database
    pub fn save(&self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(&self.packages)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(&mut self, path: &PathBuf) -> Result<(), Box<dyn std::error::Error>> {
        let json = std::fs::read_to_string(path)?;
        self.packages = serde_json::from_str(&json)?;
        Ok(())
    }

    pub fn stats(&self) -> DatabaseStats {
        let mut stats = DatabaseStats::default();

        for pkg in self.packages.values() {
            match &pkg.source {
                PackageSource::Aur { .. } => stats.aur_count += 1,
                PackageSource::Debian { .. } => stats.debian_count += 1,
                PackageSource::Flatpak { .. } => stats.flatpak_count += 1,
                PackageSource::Snap { .. } => stats.snap_count += 1,
                PackageSource::Nixpkgs { .. } => stats.nixpkgs_count += 1,
                PackageSource::Source { .. } => stats.source_count += 1,
            }
        }

        stats.total_count = self.packages.len();
        stats
    }
}

#[derive(Debug, Default)]
pub struct DatabaseStats {
    pub total_count: usize,
    pub aur_count: usize,
    pub debian_count: usize,
    pub flatpak_count: usize,
    pub snap_count: usize,
    pub nixpkgs_count: usize,
    pub source_count: usize,
}

// CLI Interface
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cache_dir = PathBuf::from("/var/cache/apkg");
    std::fs::create_dir_all(&cache_dir)?;

    let db_path = cache_dir.join("packages.json");
    let mut db = PackageDatabase::new(cache_dir);

    // Try to load existing database
    if db_path.exists() {
        println!("Loading existing package database...");
        db.load(&db_path)?;
    }

    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return Ok(());
    }

    match args[1].as_str() {
        "sync" => {
            println!("Syncing package databases...");
            
            println!("Syncing AUR...");
            let aur_count = db.sync_aur().await?;
            println!("Synced {} AUR packages", aur_count);

            // Flatpak sync (if available)
            if std::process::Command::new("flatpak").output().is_ok() {
                println!("Syncing Flatpak (flathub)...");
                match db.sync_flatpak("flathub").await {
                    Ok(count) => println!("Synced {} Flatpak packages", count),
                    Err(e) => eprintln!("Flatpak sync failed: {}", e),
                }
            }

            println!("Saving database...");
            db.save(&db_path)?;
            
            let stats = db.stats();
            println!("\nDatabase Statistics:");
            println!("  Total packages: {}", stats.total_count);
            println!("  AUR: {}", stats.aur_count);
            println!("  Debian: {}", stats.debian_count);
            println!("  Flatpak: {}", stats.flatpak_count);
        }

        "search" => {
            if args.len() < 3 {
                eprintln!("Usage: apkg search <query>");
                return Ok(());
            }

            let results = db.search(&args[2]);
            println!("Found {} packages:\n", results.len());

            for pkg in results.iter().take(20) {
                println!("{} {} - {}", pkg.name, pkg.version, pkg.description);
            }
        }

        "info" => {
            if args.len() < 3 {
                eprintln!("Usage: apkg info <package>");
                return Ok(());
            }

            if let Some(pkg) = db.packages.get(&args[2]) {
                println!("Name: {}", pkg.name);
                println!("Version: {}", pkg.version);
                println!("Description: {}", pkg.description);
                println!("Source: {:?}", pkg.source);
                println!("Dependencies: {:?}", pkg.dependencies);
                println!("Build Type: {:?}", pkg.build_type);
                if let Some(url) = &pkg.homepage {
                    println!("Homepage: {}", url);
                }
                println!("License: {:?}", pkg.license);
            } else {
                eprintln!("Package not found: {}", args[2]);
            }
        }

        "stats" => {
            let stats = db.stats();
            println!("Database Statistics:");
            println!("  Total: {}", stats.total_count);
            println!("  AUR: {}", stats.aur_count);
            println!("  Debian: {}", stats.debian_count);
            println!("  Flatpak: {}", stats.flatpak_count);
            println!("  Snap: {}", stats.snap_count);
            println!("  Nixpkgs: {}", stats.nixpkgs_count);
            println!("  Source: {}", stats.source_count);
        }

        _ => {
            print_usage();
        }
    }

    Ok(())
}

fn print_usage() {
    println!("apkg - ALinux Universal Package Manager\n");
    println!("Usage:");
    println!("  apkg sync              Sync all package databases");
    println!("  apkg search <query>    Search for packages");
    println!("  apkg info <package>    Show package information");
    println!("  apkg stats             Show database statistics");
}
