// Source: https://github.com/BamPeers/rust-ci-github-actions-workflow
// Dummy stuff for pipeline setup

/// Multiplies two integers
pub fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

/// ```
/// assert_eq!(multiply(2,2), 4);
/// ```
#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(multiply(2, 2), 4);
    }
}
