pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    (1..limit)
        .map(
            |m| match factors.iter().any(|factor| *factor != 0 && m % factor == 0) {
                true => m,
                false => 0,
            },
        )
        .sum()
}
