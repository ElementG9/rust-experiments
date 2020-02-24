pub mod client;
pub mod network;

#[cfg(test)]
mod tests {
    #[test]
    use super::*;
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
