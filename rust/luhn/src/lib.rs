enum LuhnChar {
    Digit(u8),
    Whitespace,
    Invalid,
}

// Check a Luhn checksum.
pub fn is_valid(code: &str) -> bool {
    match code
        .chars()
        .rev()
        .map(|char| {
            if char.is_ascii_digit() {
                LuhnChar::Digit(char.to_digit(10).unwrap() as u8)
            } else if char.is_ascii_whitespace() {
                LuhnChar::Whitespace
            } else {
                LuhnChar::Invalid
            }
        })
        .scan(-1, |digit_index, item| {
            if let LuhnChar::Digit(_) = item {
                *digit_index = *digit_index + 1;
            }
            Some((*digit_index, item))
        })
        .try_fold((-1, 0), |(_, sum), (index, item)| match item {
            LuhnChar::Digit(value) => Ok((
                index,
                sum + if index % 2 == 0 {
                    value
                } else {
                    value * 2 % 10 + value / 5
                },
            )),
            LuhnChar::Whitespace => Ok((index, sum)),
            LuhnChar::Invalid => Err(()),
        }) {
        Ok((last_index, sum)) => last_index > 0 && sum % 10 == 0,
        Err(()) => false,
    }
}
