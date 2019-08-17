mod browser;
mod engin;
mod platform;
mod semver;
mod utils;

/// user agent base struct
pub struct UserAgent {
    version: String,
    browser: Option<browser::Browser>,
    platform: Option<platform::Platform>,
}
impl UserAgent {
    /// export UserAgent
    pub fn to_string(&self) -> String {
        let b = self.browser.as_ref().unwrap();
        let p = &self.platform.as_ref().unwrap();

        if b.name.contains("irefox") {
            return format!(
                "{} ({}; rv:{}) {}",
                self.version,
                p.to_string().replace("_", "."),
                b.semver.to_string('.'),
                b.to_string()
            );
        }
        format!("{} ({}) {}", self.version, p.to_string(), b.to_string())
    }
}
/// create a random UserAgent
/// # Expleam
/// ```rust
/// let ua = randua::new();
/// println!("{}", ua);
/// ```
pub fn new() -> UserAgent {
    let mozilla_with_version = "Mozilla/5.0";
    let mut ua = UserAgent {
        version: String::from(mozilla_with_version),
        browser: None,
        platform: None,
    };
    if let None = ua.browser {
        ua.browser = Some(browser::Browser::random())
    }
    if let None = ua.platform {
        ua.platform = Some(platform::Platform::random(true));
    }
    return ua;
}

#[test]
fn test_user_agent() {
    println!("{}", new().firefox().phone().to_string());
}
