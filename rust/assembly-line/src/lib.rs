pub fn production_rate_per_hour(speed: u8) -> f64 {
    if speed > 10 {
        return 0.0;
    }
    let success_rate: [f64; 11] = [0.0, 1.0, 1.0, 1.0, 1.0, 0.9, 0.9, 0.9, 0.9, 0.77, 0.77];
    success_rate[usize::from(speed)] * f64::from(speed) * 221.0
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}
