extern crate rand;
use rand::prelude::*;
pub struct Semver {
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub pre_release: String,
    pub mate_data: String,
}

impl Semver {
    pub fn to_string(&self, sep: char) -> String {
        if self.patch == 0 {
            return format!("{}{}{}", self.major, sep, self.minor);
        }
        format!("{}{}{}{}{}", self.major, sep, self.minor, sep, self.patch)
    }
}

pub struct VersionInfo {
    pub major: EndPoint,
    pub minor: EndPoint,
    pub patch: EndPoint,
}

pub struct EndPoint {
    pub start: i32,
    pub end: i32,
}
impl VersionInfo {
    pub fn create(&self) -> Semver {
        Semver {
            major: random_range(self.major.start, self.major.end),
            minor: random_range(self.minor.start, self.minor.end),
            patch: random_range(self.patch.start, self.patch.end),
            pre_release: String::from(""),
            mate_data: String::from(""),
        }
    }
}

fn random_range(start: i32, end: i32) -> i32 {
    if start == end {
        return start;
    }
    let mut rng = rand::thread_rng();
    let x: i32 = rng.gen_range(start, end);
    x
}
