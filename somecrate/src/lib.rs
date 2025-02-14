pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    // click "Run Test" in the gutter to show the issue
    fn it_works() {
    
        let result = add(2, 2);
        assert_eq!(result, 4);
        todo!();
    }
}
