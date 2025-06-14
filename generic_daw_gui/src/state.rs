use serde::{Deserialize, Serialize};
use std::{
    fs::{read_to_string, write},
    io,
    path::{Path, PathBuf},
    sync::{Arc, LazyLock},
};

pub static STATE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| dirs::state_dir().unwrap().join("generic_daw.toml"));

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
#[serde(default)]
pub struct State {
    pub last_project: Option<Arc<Path>>,
}

impl State {
    #[must_use]
    pub fn read() -> Self {
        let config = read_to_string(&*STATE_PATH);

        let read =
            toml::from_str::<Self>(config.as_deref().unwrap_or_default()).unwrap_or_default();

        if let Err(e) = config {
            if e.kind() == io::ErrorKind::NotFound {
                read.write();
            }
        }

        read
    }

    pub fn write(&self) {
        write(&*STATE_PATH, toml::to_string(self).unwrap()).unwrap();
    }
}
