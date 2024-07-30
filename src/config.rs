use crate::link::{Link, LinkType};
use configparser::ini::Ini;
use std::{
    path::{Path, PathBuf},
    sync::{Mutex, OnceLock},
};

struct ConfigurationData {
    ini: Ini,
    path: Option<PathBuf>,
}

pub struct Configuration {
    data: Mutex<ConfigurationData>,
}

impl Configuration {
    pub fn get() -> &'static Self {
        static CONFIG: OnceLock<Configuration> = OnceLock::new();

        CONFIG.get_or_init(Configuration::new)
    }

    pub fn new() -> Self {
        Self {
            data: Mutex::new(ConfigurationData {
                ini: Ini::new(),
                path: None,
            }),
        }
    }

    pub fn load_from_file(&self, p: &Path) {
        let mut data = self.data.lock().unwrap();
        let _ = data.ini.load(p);
        data.path = Some(p.to_path_buf());
    }

    pub fn write(&self, link: &Link) {
        let mut data = self.data.lock().unwrap();
        data.ini
            .setstr(&link.id.to_string(), "name", Some(&link.name));

        match link.r#type.clone() {
            LinkType::Account(api_key) => {
                data.ini
                    .setstr(&link.id.to_string(), "type", Some("account"));
                data.ini
                    .setstr(&link.id.to_string(), "api_key", Some(&api_key));
            }
            LinkType::None => (),
        }

        if let Some(p) = data.path.as_ref() {
            let _ = data.ini.write(p);
        }
    }
}
