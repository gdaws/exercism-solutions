#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    InvalidInputBase,
    InvalidOutputBase,
    InvalidDigit(u32),
}

pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, Error> {
    match decode(number, from_base) {
        Ok(value) => encode(value, to_base),
        Err(error) => Err(error),
    }
}

fn decode(number: &[u32], base: u32) -> Result<u32, Error> {
    if base < 2 {
        return Err(Error::InvalidInputBase);
    }
    let num_digits = number.len();
    if num_digits == 0 {
        return Ok(0);
    }
    let mut value = 0;
    for (index, digit) in number.iter().enumerate() {
        if *digit >= base {
            return Err(Error::InvalidDigit(*digit));
        }
        value += digit * base.pow((num_digits - 1 - index) as u32);
    }
    Ok(value)
}

fn encode(mut value: u32, base: u32) -> Result<Vec<u32>, Error> {
    if base < 2 {
        return Err(Error::InvalidOutputBase);
    }
    if value == 0 {
        return Ok(vec![0]);
    }
    let mut result = Vec::new();
    let m = value.ilog(base);
    for p in 0..(m + 1) {
        let d = base.pow(m - p);
        let u = value / d;
        result.push(u);
        value -= u * d;
    }
    Ok(result)
}
