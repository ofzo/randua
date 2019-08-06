pub mod chrome;
pub mod all {}

#[cfg(test)]
mod tests {
    use super::chrome;
    #[test]
    fn it_works() {
        assert_eq!(chrome::chrome::create(), "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_14_6) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/75.0.3770.142 Safari/537.36");
    }
}
