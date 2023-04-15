pub fn factors(mut n: u64) -> Vec<u64> {
    let mut pf = Vec::new();
    let mut m = 2;
    while n > 1 {
        if n % m == 0 {
            pf.push(m);
            n /= m;
        } else {
            m += 1;
        }
    }
    pf
}
