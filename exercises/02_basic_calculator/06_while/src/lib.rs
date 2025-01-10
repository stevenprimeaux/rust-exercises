pub fn factorial(n: u32) -> u32 {
    let mut fact: u32 = 1;
    let mut fact_current: u32 = 1;
    while fact_current < n {
        fact_current += 1;
        fact *= fact_current;
    }

    fact
}

#[cfg(test)]
mod tests {
    use crate::factorial;

    #[test]
    fn first() {
        assert_eq!(factorial(0), 1);
    }

    #[test]
    fn second() {
        assert_eq!(factorial(1), 1);
    }

    #[test]
    fn third() {
        assert_eq!(factorial(2), 2);
    }

    #[test]
    fn fifth() {
        assert_eq!(factorial(5), 120);
    }
}
