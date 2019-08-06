struct Ua {
    browser_version: String,
    os_version: String,
    webkit_version: String,
}

impl Ua {
    fn to_string(&self) -> String {
        format!("Mozilla/5.0 (Macintosh; Intel Mac OS X {}) AppleWebKit/{} (KHTML, like Gecko) Chrome/{} Safari/537.36", self.os_version, self.webkit_version, self.browser_version)
    }
}

pub mod chrome {
    use super::Ua;
    pub fn create() -> String {
        let u = Ua {
            browser_version: String::from("75.0.3770.142"),
            os_version: String::from("10_14_6"),
            webkit_version: String::from("537.36"),
        };
        u.to_string()
    }
}
