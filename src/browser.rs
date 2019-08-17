use crate::engin;
use crate::semver;
use crate::UserAgent;
use rand::Rng;
pub struct Browser {
    pub name: String,
    pub engin: engin::Engin,
    pub semver: semver::Semver,
}

impl Browser {
    pub fn to_string(&self) -> String {
        if self.name.contains("hrome") {
            return format!(
                "{} {}/{} Safari/537.36",
                self.engin.to_string(),
                self.name,
                self.semver.to_string('.')
            );
        }
        format!(
            "{} {}/{}",
            self.engin.to_string(),
            self.name,
            self.semver.to_string('.')
        )
    }
    pub fn random() -> Browser {
        let mut _browser_db: Vec<BrowserInfo> = vec![
            BrowserInfo {
                name: String::from("Firefox"),
                engin_info: engin::EnginInfo {
                    name: "Gecko".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint { start: 0, end: 0 },
                        minor: semver::EndPoint { start: 0, end: 0 },
                        patch: semver::EndPoint { start: 0, end: 0 },
                    },
                },
                version_info: semver::VersionInfo {
                    major: semver::EndPoint { start: 35, end: 56 },
                    minor: semver::EndPoint { start: 35, end: 56 },
                    patch: semver::EndPoint { start: 0, end: 3 },
                },
            },
            BrowserInfo {
                name: String::from("Chrome"),
                engin_info: engin::EnginInfo {
                    name: "AppleWebKit".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint {
                            start: 534,
                            end: 603,
                        },
                        minor: semver::EndPoint { start: 35, end: 56 },
                        patch: semver::EndPoint { start: 0, end: 0 },
                    },
                },
                version_info: semver::VersionInfo {
                    major: semver::EndPoint { start: 39, end: 64 },
                    minor: semver::EndPoint { start: 0, end: 0 },
                    patch: semver::EndPoint {
                        start: 0,
                        end: 3000,
                    },
                },
            },
            BrowserInfo {
                name: String::from("Safari"),
                engin_info: engin::EnginInfo {
                    name: "WebKit".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint {
                            start: 534,
                            end: 603,
                        },
                        minor: semver::EndPoint { start: 0, end: 21 },
                        patch: semver::EndPoint { start: 0, end: 10 },
                    },
                },
                version_info: semver::VersionInfo {
                    major: semver::EndPoint { start: 5, end: 11 },
                    minor: semver::EndPoint { start: 0, end: 2 },
                    patch: semver::EndPoint { start: 0, end: 10 },
                },
            },
        ];
        let idx = rand::thread_rng().gen::<usize>() % _browser_db.len();
        let browser_info = &_browser_db[idx];
        browser_info.create()
    }
}

struct BrowserInfo {
    name: String,
    engin_info: engin::EnginInfo,
    version_info: semver::VersionInfo,
}

impl BrowserInfo {
    pub fn create(&self) -> Browser {
        Browser {
            name: self.name.clone(),
            engin: self.engin_info.create(),
            semver: self.version_info.create(),
        }
    }
}

impl UserAgent {
    /// create a chrome browser
    pub fn chrome(&mut self) -> &mut UserAgent {
        self.browser = Some(
            BrowserInfo {
                name: String::from("Chrome"),
                engin_info: engin::EnginInfo {
                    name: "AppleWebKit".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint {
                            start: 534,
                            end: 603,
                        },
                        minor: semver::EndPoint { start: 35, end: 56 },
                        patch: semver::EndPoint { start: 0, end: 0 },
                    },
                },
                version_info: semver::VersionInfo {
                    major: semver::EndPoint { start: 39, end: 64 },
                    minor: semver::EndPoint { start: 0, end: 0 },
                    patch: semver::EndPoint {
                        start: 0,
                        end: 3000,
                    },
                },
            }
            .create(),
        );

        self
    }
    /// create a safari browser
    pub fn safari(&mut self) -> &mut UserAgent {
        self.browser = Some(
            BrowserInfo {
                name: String::from("Safari"),
                engin_info: engin::EnginInfo {
                    name: "WebKit".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint {
                            start: 534,
                            end: 603,
                        },
                        minor: semver::EndPoint { start: 0, end: 21 },
                        patch: semver::EndPoint { start: 0, end: 10 },
                    },
                },
                version_info: semver::VersionInfo {
                    major: semver::EndPoint { start: 5, end: 11 },
                    minor: semver::EndPoint { start: 0, end: 2 },
                    patch: semver::EndPoint { start: 0, end: 10 },
                },
            }
            .create(),
        );

        self
    }
    /// create a firefox browser
    pub fn firefox(&mut self) -> &mut UserAgent {
        self.browser = Some(
            BrowserInfo {
                name: String::from("Firefox"),
                engin_info: engin::EnginInfo {
                    name: "Gecko".to_string(),
                    version_info: semver::VersionInfo {
                        major: semver::EndPoint { start: 0, end: 0 },
                        minor: semver::EndPoint { start: 0, end: 0 },
                        patch: semver::EndPoint { start: 0, end: 0 },
                    },
                },
                version_info: semver::VersionInfo {
                    major: semver::EndPoint { start: 35, end: 56 },
                    minor: semver::EndPoint { start: 35, end: 56 },
                    patch: semver::EndPoint { start: 0, end: 3 },
                },
            }
            .create(),
        );

        self
    }
}
