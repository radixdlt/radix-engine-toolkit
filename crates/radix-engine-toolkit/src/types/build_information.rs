#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BuildInformation {
    pub version: String,
    pub scrypto_dependency: DependencyInformation,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependencyInformation {
    // Crates.io
    Version(String),

    // Github
    Tag(String),
    Branch(String),
    Rev(String),
}

impl DependencyInformation {
    pub(crate) fn from_environment_variable() -> Self {
        let version = env!("SCRYPTO_DEPENDENCY");

        let mut splitted = version.split('=');
        let identifier = splitted.next().expect("Should never fail");
        let value = splitted.next().expect("Should never fail");

        match identifier {
            "version" => Self::Version(value.into()),
            "tag" => Self::Tag(value.into()),
            "branch" => Self::Branch(value.into()),
            "rev" => Self::Rev(value.into()),
            _ => panic!("Unknown identifier encountered: {}", identifier),
        }
    }
}
