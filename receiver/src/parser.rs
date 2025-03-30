// parse blobs from twitch chat

// example blob

use winnow::ascii::{alpha1, alphanumeric1};
use winnow::error::EmptyError;
use winnow::Parser;
//blobacpageznmeauqi5wrg45i53tsqbt7336zjcpmrqv4mazl4u5hj6lyajdnb2hi4dthixs65ltmuys2mjoojswyylzf + more
// I want to match on things that start with 'blob' and are all lowercase alphanumeric

pub fn is_blob(s: &mut &str) -> bool {
    "blob"
        .and_then(alphanumeric1::<&str, EmptyError>)
        .verify(|c: &str| dbg!(c.len()) > 30)
        .parse_next(s)
        .is_ok()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid_blob() {
        let mut input = "blobacpageznmeauqi5wrg45i53tsqbt7336zjcpmrqv4mazl4u5hj6lyajdnb2hi4dthixs65ltmuys2mjoojswyylzf";
        assert!(is_blob(&mut input));
    }

    #[test]
    fn test_invalid_prefix() {
        let mut input = "blab123";
        assert!(!is_blob(&mut input));
    }

    #[test]
    fn test_blank() {
        let mut input = "";
        assert!(!is_blob(&mut input));
    }

    #[test]
    fn test_blob_then_space() {
        let mut input = "some blob and then words";
        assert!(!is_blob(&mut input));
    }

    #[test]
    fn test_just_blob() {
        let mut input = "blob";
        assert!(!is_blob(&mut input));
    }
}
