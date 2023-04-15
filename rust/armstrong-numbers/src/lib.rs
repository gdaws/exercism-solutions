pub fn is_armstrong_number(num: u64) -> bool {
    let num_digits = num.max(1).ilog10() + 1;
    num == (1..(num_digits + 1))
        .map(|p| (num % 10u64.pow(p) / 10u64.pow(p - 1)).pow(num_digits))
        .sum()
}
