use crate::link::{Link, LinkType};
use configparser::ini::Ini;
use std::{
    path::{Path, PathBuf},
    str::FromStr,
    sync::{Mutex, OnceLock},
};
use uuid::Uuid;

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

    pub fn get_link(&self, name: &str) -> Option<Link> {
        let data = self.data.lock().unwrap();

        data.ini
            .sections()
            .iter()
            .find(|id| data.ini.get(id, "name") == Some(name.to_string()))
            .and_then(|id| match data.ini.get(id, "type").as_deref() {
                Some("account") => {
                    if let Some(api_key) = data.ini.get(id, "api_key") {
                        Some(Link {
                            name: name.to_string(),
                            r#type: LinkType::Account(api_key),
                            id: Uuid::from_str(id).unwrap(),
                        })
                    } else {
                        None
                    }
                }
                _ => None,
            })
    }

    pub fn get_links(&self) -> Vec<Link> {
        let data = self.data.lock().unwrap();

        let names = data
            .ini
            .sections()
            .iter()
            .filter_map(|id| data.ini.get(id, "name"))
            .collect::<Vec<_>>();

        drop(data);

        names
            .iter()
            .filter_map(|n| self.get_link(&n))
            .collect::<Vec<_>>()
    }
}
