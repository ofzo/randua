use std::fmt;
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

impl std::fmt::Display for UserAgent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}

/// create a random UserAgent
///
/// ## create a random user agent
/// ```rust
/// let ua = randua::new();
///
/// // Mozilla/5.0 (iPhone; CPU iPhone OS 7.0.2 like Mac OS X; rv:41.40) Gecko/20100101 Firefox/41.40
/// println!("{}", ua);
/// ```
///
/// ## create a chrome user agent for desktop platform
/// ```rust
/// let mut ua = randua::new();
/// ua.chrome().desktop();
/// assert!(ua.to_string().contains("Chrome"));
/// ```
///
/// ## create a chrome user agent for mobile platform
/// ```rust
/// let mut ua = randua::new();
/// ua.chrome().phone();
/// assert!(ua.to_string().contains("Safari"));
/// assert!(ua.to_string().contains("iPhone") || ua.to_string().contains("Android"));
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
fn test_user_agent_for_firefox() {
    let ua = new().firefox().phone().to_string();
    assert!(ua.contains("Firefox"));
    assert!(ua.to_string().contains("iPhone") || ua.to_string().contains("Android"));
}
