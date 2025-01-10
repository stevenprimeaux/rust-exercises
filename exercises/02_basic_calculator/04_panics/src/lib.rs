/// Given the start and end points of a journey, and the time it took to complete the journey,
/// calculate the average speed of the journey.
fn speed(start: u32, end: u32, time_elapsed: u32) -> u32 {
    if time_elapsed == 0 {
        panic!("The journey took no time at all. That's impossible!")
    }

    (end - start) / time_elapsed
}

#[cfg(test)]
mod tests {
    use crate::speed;

    #[test]
    fn case1() {
        assert_eq!(speed(0, 10, 10), 1);
    }

    #[test]
    #[should_panic(expected = "The journey took no time at all. That's impossible!")]
    fn by_zero() {
        speed(0, 10, 0);
    }
}
