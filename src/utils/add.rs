pub fn add(a: i32, b: i32) -> i32 {
    return a + b;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
