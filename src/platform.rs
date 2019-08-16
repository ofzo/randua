use crate::semver;
use crate::utils;
use crate::UserAgent;
pub struct Platform {
    pub name: String,
    pub semver: semver::Semver,
    pub comment: String,
}

impl Platform {
    pub fn to_string(&self) -> String {
        match self.name.as_ref() {
            "iOS" => format!(
                "iPhone; CPU iPhone OS {} like Mac OS X",
                self.semver.to_string('_')
            ),
            "Android" => format!(
                "Linux; Android {}; Build/{}",
                self.semver.to_string('_'),
                utils::rand_string_bytes_mask_impr(5)
            ),
            "Windows" => format!("Windows NT {}; Win64; x64", self.semver.to_string('_')),
            "Linux" => format!("X11; Linux x86_64"),
            "MacOS" => format!("Macintosh; Intel Mac OS X {}", self.semver.to_string('_')),
            _ => format!("{}; {}{}", self.name, self.name, self.semver.to_string('_')),
        }
    }
    pub fn random(is_desktop: bool) -> Platform {
        let platform_db = vec![
            vec![
                PlatformInfo {
                    name: "Linux".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint { start: 3, end: 5 },
                        minor: semver::EndPoint { start: 0, end: 3 },
                        patch: semver::EndPoint { start: 0, end: 3 },
                    },
                    comment: "".to_string(),
                },
                PlatformInfo {
                    name: "MacOS".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint { start: 8, end: 10 },
                        minor: semver::EndPoint { start: 0, end: 3 },
                        patch: semver::EndPoint { start: 0, end: 3 },
                    },
                    comment: "".to_string(),
                },
                PlatformInfo {
                    name: "Windows".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint { start: 7, end: 10 },
                        minor: semver::EndPoint { start: 0, end: 3 },
                        patch: semver::EndPoint { start: 0, end: 3 },
                    },
                    comment: "".to_string(),
                },
            ],
            vec![
                PlatformInfo {
                    name: "iOS".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint { start: 6, end: 11 },
                        minor: semver::EndPoint { start: 0, end: 4 },
                        patch: semver::EndPoint { start: 0, end: 4 },
                    },
                    comment: "".to_string(),
                },
                PlatformInfo {
                    name: "Android".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint { start: 4, end: 8 },
                        minor: semver::EndPoint { start: 0, end: 4 },
                        patch: semver::EndPoint { start: 0, end: 4 },
                    },
                    comment: "".to_string(),
                },
            ],
        ];
        let index: usize = if is_desktop {
            rand::random::<usize>() % 3
        } else {
            rand::random::<usize>() % 2
        };
        let info = if is_desktop {
            &platform_db[0][index]
        } else {
            &platform_db[1][index]
        };

        Platform {
            name: info.name.clone(),
            semver: info.version_info.create(),
            comment: info.comment.clone(),
        }
    }
}

struct PlatformInfo {
    name: String,
    version_info: semver::VersionInfo,
    comment: String,
}

impl UserAgent {
    pub fn desktop(&mut self) -> &mut UserAgent {
        self.platform = Some(Platform::random(true));
        self
    }
    pub fn phone(&mut self) -> &mut UserAgent {
        self.platform = Some(Platform::random(false));
        self
    }
}
