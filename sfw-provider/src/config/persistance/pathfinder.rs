use crate::config::Config;
use pemstore::pathfinder::PathFinder;
use std::path::PathBuf;

#[derive(Debug)]
pub struct ProviderPathfinder {
    pub config_dir: PathBuf,
    pub private_sphinx_key: PathBuf,
    pub public_sphinx_key: PathBuf,
}

impl ProviderPathfinder {
    pub fn new(id: String) -> Self {
        let os_config_dir = dirs::config_dir().unwrap(); // grabs the OS default config dir
        let config_dir = os_config_dir.join("nym").join("mixnodes").join(id);
        let private_sphinx_key = config_dir.join("private.pem");
        let public_sphinx_key = config_dir.join("public.pem");
        ProviderPathfinder {
            config_dir,
            private_sphinx_key,
            public_sphinx_key,
        }
    }

    pub fn new_from_config(config: &Config) -> Self {
        ProviderPathfinder {
            config_dir: config.get_config_file_save_location(),
            private_sphinx_key: config.get_private_sphinx_key_file(),
            public_sphinx_key: config.get_public_sphinx_key_file(),
        }
    }
}

impl PathFinder for ProviderPathfinder {
    fn config_dir(&self) -> PathBuf {
        self.config_dir.clone()
    }

    fn private_identity_key(&self) -> PathBuf {
        // TEMPORARILY USE SAME KEYS AS ENCRYPTION
        self.private_sphinx_key.clone()
    }

    fn public_identity_key(&self) -> PathBuf {
        // TEMPORARILY USE SAME KEYS AS ENCRYPTION
        self.public_sphinx_key.clone()
    }

    fn private_encryption_key(&self) -> Option<PathBuf> {
        Some(self.private_sphinx_key.clone())
    }

    fn public_encryption_key(&self) -> Option<PathBuf> {
        Some(self.public_sphinx_key.clone())
    }
}
