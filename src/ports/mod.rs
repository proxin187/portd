mod store;
mod port;

use store::Store;
use port::Port;

use crate::error::Error;

use std::process::Command;
use std::path::{Path, PathBuf};
use std::env;


pub struct Ports {
    store: Store,
    path: PathBuf,
}

impl Ports {
    pub fn new() -> Ports {
        let path = env::var("PORTS")
            .map(|ports| PathBuf::from(ports))
            .unwrap_or_else(|_| PathBuf::from("/usr/s0-ports"));

        Ports {
            store: Store::new(),
            path,
        }
    }

    pub fn find(&self, specifier: &str) -> Result<Port, Error> {
        let name = specifier.split('@').next().ok_or_else(|| Error::NoSuchPort(specifier.to_string()))?;
        let path = self.path.join(name);

        if path.exists() {
            Port::new(&specifier, path)
        } else {
            Err(Error::NoSuchPort(specifier.to_string()))
        }
    }

    fn check(&self, port: &Port) -> Result<(), Error> {
        let missing = port.dependencies()?
            .into_iter()
            .filter(|dependency| !self.store.has(dependency))
            .collect::<Vec<String>>();

        match missing.as_slice() {
            [] => Ok(()),
            [..] => Err(Error::MissingDependencies(missing)),
        }
    }

    pub fn install(&self, port: Port) -> Result<(), Error> {
        let version = port.resolve()?;

        println!("version: {}", version);

        self.check(&port)?;

        Ok(())
    }
}

pub fn install(specifiers: Vec<String>) -> Result<(), Error> {
    let ports = Ports::new();

    for specifier in specifiers {
        let port = ports.find(&specifier)?;

        ports.install(port)?;
    }

    Ok(())
}

    // TODO: all of this needs a cleanup

    /*
    pub fn fetch(&self) -> Result<(), Error> {
        let dbuild = self.path.join(format!("{}.dbuild", self.spec));

        self.run_dbuild(&dbuild, "fetch")?;

        Ok(self.rename(dbuild, self.path.join(format!("local/{}.dbuild", self.spec)))?)
    }

    pub fn clean(&self) -> Result<(), Error> {
        let dbuild = self.path.join(format!("local/{}.dbuild", self.spec));

        self.run_dbuild(&dbuild, "clean")?;

        Ok(self.rename(dbuild, self.path.join(format!("{}.dbuild", self.spec)))?)
    }

    pub fn rename(&self, from: PathBuf, to: PathBuf) -> Result<(), Error> {
        let permissions = from.metadata()?.permissions();

        fs::rename(from, &to)?;

        Ok(fs::set_permissions(to, permissions)?)
    }

    pub fn run_dbuild(&self, dbuild: &PathBuf, arg: &str) -> Result<(), Error> {
        let status = Command::new(&dbuild)
            .current_dir(&self.path)
            .env("PORTD_PORTS", &self.path)
            .arg(arg)
            .status()?;

        if status.success() {
            println!("info: dbuild success: {}", dbuild.to_string_lossy());
        } else {
            return Err(Error::DbuildFailed(dbuild.clone()));
        }

        Ok(())
    }
    */

/*
#[inline]
pub fn fetch(ports: Vec<String>) -> Result<(), Error> {
    for spec in ports {
        Port::find(spec)
            .and_then(|port| port.fetch())?;
    }

    Ok(())
}

#[inline]
pub fn clean(ports: Vec<String>) -> Result<(), Error> {
    for spec in ports {
        Port::find(format!("local/{}", spec))
            .and_then(|port| port.clean())?;
    }

    Ok(())
}
*/


