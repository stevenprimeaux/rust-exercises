pub trait Power<Exp> {
    fn power(self, n: Exp) -> Self;
}

impl Power<u16> for u32 {
    fn power(self, n: u16) -> u32 {
        let mut mult: u32 = 1;
        for _ in 0..n {
            mult *= self;
        }

        mult
    }
}

impl Power<u32> for u32 {
    fn power(self, n: u32) -> u32 {
        let mut mult: u32 = 1;
        for _ in 0..n {
            mult *= self;
        }

        mult
    }
}

impl Power<&u32> for u32 {
    fn power(self, n: &u32) -> u32 {
        let mut mult: u32 = 1;
        for _ in 0..*n {
            mult *= self;
        }

        mult
    }
}

#[cfg(test)]
mod tests {
    use super::Power;

    #[test]
    fn test_power_u16() {
        let x: u32 = 2_u32.power(3u16);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_u32() {
        let x: u32 = 2_u32.power(3u32);
        assert_eq!(x, 8);
    }

    #[test]
    fn test_power_ref_u32() {
        let x: u32 = 2_u32.power(&3u32);
        assert_eq!(x, 8);
    }
}
