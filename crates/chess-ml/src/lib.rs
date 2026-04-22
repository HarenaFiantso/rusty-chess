pub fn modulo(left: u64, right: u64) -> u64 {
    left % right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = modulo(2, 2);
        assert_eq!(result, 0);
    }
}
