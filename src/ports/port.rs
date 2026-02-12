use crate::error::Error;

use std::path::PathBuf;
use std::fs;

use semver::{Version, VersionReq};


pub struct Port {
    name: String,
    version: VersionReq,
    path: PathBuf,
}

impl Port {
    pub fn new(specifier: &str, path: PathBuf) -> Result<Port, Error> {
        if let Some((name, version)) = specifier.split_once('@') {
            Ok(Port {
                name: name.to_string(),
                version: VersionReq::parse(version)?,
                path,
            })
        } else {
            Ok(Port {
                name: specifier.to_string(),
                version: VersionReq::STAR,
                path,
            })
        }
    }

    pub fn dependencies(&self) -> Result<Vec<String>, Error> {
        let content = fs::read_to_string(&self.path)?;

        Ok(content.lines().map(|line| line.to_string()).collect::<Vec<String>>())
    }

    pub fn resolve(&self) -> Result<Version, Error> {
        fs::read_dir(self.path.join("versions"))?
            .filter_map(|result| result.ok().and_then(|entry| Version::parse(entry.file_name().to_string_lossy().as_ref()).ok()))
            .filter(|version| self.version.matches(version))
            .max_by(|a, b| a.cmp_precedence(b))
            .ok_or_else(|| Error::ResolveFailed(self.name.clone(), self.version.clone()))
    }

    /*
    pub fn command(&self, command: &str) -> Result<Vec<String>, Error> {
        let output = Command::new(&self.path)
            .env("PORTS", &self.path)
            .arg(command)
            .output()?;

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);

            Ok(stdout.split(' ').map(|line| line.to_string()).collect::<Vec<String>>())
        } else {
            Err(Error::InvalidPort(self.path.clone()))
        }
    }
    */
}


