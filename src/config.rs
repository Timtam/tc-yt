use configparser::ini::{Ini, WriteOptions};
use std::{
    path::Path,
    sync::{Mutex, OnceLock},
};

struct ConfigurationData {
    ini: Ini,
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
            data: Mutex::new(ConfigurationData { ini: Ini::new() }),
        }
    }

    pub fn load_from_file(&self, p: &Path) {
        let mut data = self.data.lock().unwrap();
        let _ = data.ini.load(p);
    }
}
