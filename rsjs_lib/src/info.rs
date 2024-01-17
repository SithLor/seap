pub struct Crate {
    pub name: String,
    pub version: String,
    pub authors: String,
    pub description: String,
    pub homepage: String,
    pub repository: String,
    pub manifest_dir: String,
    pub version_major: String,
    pub version_minor: String,
    pub version_patch: String,
    pub version_pre: String,
}
impl Crate {
    pub fn new() -> Crate {
        const CARGO_MANIFEST_DIR: &str = env!("CARGO_MANIFEST_DIR");
        const CARGO_PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
        const CARGO_PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
        const CARGO_PKG_HOMEPAGE: &str = env!("CARGO_PKG_HOMEPAGE");
        const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");
        const CARGO_PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");
        const CARGO_PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
        const CARGO_PKG_VERSION_MAJOR: &str = env!("CARGO_PKG_VERSION_MAJOR");
        const CARGO_PKG_VERSION_MINOR: &str = env!("CARGO_PKG_VERSION_MINOR");
        const CARGO_PKG_VERSION_PATCH: &str = env!("CARGO_PKG_VERSION_PATCH");
        const CARGO_PKG_VERSION_PRE: &str = env!("CARGO_PKG_VERSION_PRE");
        Crate {
            name: CARGO_PKG_NAME.to_string(),
            version: CARGO_PKG_VERSION.to_string(),
            authors: CARGO_PKG_AUTHORS.to_string(),
            description: CARGO_PKG_DESCRIPTION.to_string(),
            homepage: CARGO_PKG_HOMEPAGE.to_string(),
            repository: CARGO_PKG_REPOSITORY.to_string(),
            manifest_dir: CARGO_MANIFEST_DIR.to_string(),
            version_major: CARGO_PKG_VERSION_MAJOR.to_string(),
            version_minor: CARGO_PKG_VERSION_MINOR.to_string(),
            version_patch: CARGO_PKG_VERSION_PATCH.to_string(),
            version_pre: CARGO_PKG_VERSION_PRE.to_string(),
        }
    }
    pub fn get_name(&self) -> String {
        self.name.clone()
    }
    pub fn get_version(&self) -> String {
        self.version.clone()
    }
    pub fn get_authors(&self) -> String {
        self.authors.clone()
    }
    pub fn get_description(&self) -> String {
        self.description.clone()
    }
    pub fn get_homepage(&self) -> String {
        self.homepage.clone()
    }
    pub fn get_repository(&self) -> String {
        self.repository.clone()
    }
    pub fn get_manifest_dir(&self) -> String {
        self.manifest_dir.clone()
    }
    pub fn get_version_major(&self) -> String {
        self.version_major.clone()
    }
    pub fn get_version_minor(&self) -> String {
        self.version_minor.clone()
    }
    pub fn get_version_patch(&self) -> String {
        self.version_patch.clone()
    }
    pub fn get_version_pre(&self) -> String {
        self.version_pre.clone()
    }

}
#[macro_export]
macro_rules! CARGO_MANIFEST_DIR {
    () => {
        {
            let string: String = env!("CARGO_MANIFEST_DIR").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_AUTHORS {
    () => {
        {
            let string: String = env!("CARGO_PKG_AUTHORS").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_DESCRIPTION {
    () => {
        {
            let string: String = env!("CARGO_PKG_DESCRIPTION").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_HOMEPAGE {
    () => {
        {
            let string: String = env!("CARGO_PKG_HOMEPAGE").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_NAME {
    () => {
        {
            let string: String = env!("CARGO_PKG_NAME").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_REPOSITORY {
    () => {
        {
            let string: String = env!("CARGO_PKG_REPOSITORY").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_VERSION {
    () => {
        {
            let string: String = env!("CARGO_PKG_VERSION").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_VERSION_MAJOR {
    () => {
        {
            let string: String = env!("CARGO_PKG_VERSION_MAJOR").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_VERSION_MINOR {
    () => {
        {
            let string: String = env!("CARGO_PKG_VERSION_MINOR").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_VERSION_PATCH {
    () => {
        {
            let string: String = env!("CARGO_PKG_VERSION_PATCH").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! CARGO_PKG_VERSION_PRE {
    () => {
        {
            let string: String = env!("CARGO_PKG_VERSION_PRE").to_string();
            string
        }
    };
}
#[macro_export]
macro_rules! called_from {
    () => {
        {
            let string: String = format!("(/{}/{}:{}:{})",env!("CARGO_PKG_NAME"),file!(),line!(),column!());
            string
        }
    };
}
#[inline(always)]
pub fn call_from() -> String {
    let string: String = format!("(/{}/{}:{}:{})",env!("CARGO_PKG_NAME"),file!(),line!(),column!());
    string
}