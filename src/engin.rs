use crate::semver;

pub struct Engin {
    pub name: String,
    pub semver: semver::Semver,
}

impl Engin {
    pub fn to_string(&self) -> String {
        if self.name.contains("ecko") {
            return format!("{}/20100101", self.name);
        }
        format!(
            "{}/{} (KHTML, like Gecko)",
            self.name,
            self.semver.to_string('.')
        )
    }
}

pub struct EnginInfo {
    pub name: String,
    pub version_info: semver::VersionInfo,
}

impl EnginInfo {
    pub fn create(&self) -> Engin {
        Engin {
            name: self.name.clone(),
            semver: self.version_info.create(),
        }
    }
}
