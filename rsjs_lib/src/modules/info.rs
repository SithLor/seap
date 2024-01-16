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
    fn new() -> Crate {
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