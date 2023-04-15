struct MinefieldCount {
    values: Vec<u8>,
    width: usize,
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let num_rows = minefield.len();

    if num_rows == 0 {
        return Vec::new();
    }

    let num_columns = minefield[0].len();

    let mut count = MinefieldCount::new(num_columns, num_rows);

    for (y, row) in minefield.iter().enumerate() {
        for (x, value) in row.as_bytes().iter().enumerate() {
            if *value == 42 {
                count.flag(x, y);
            }
        }
    }

    (0..num_rows)
        .map(|y| {
            (0..num_columns)
                .map(|x| match count.get(x, y) {
                    0 => ' ',
                    9 => '*',
                    value => (('0' as u8) + value) as char,
                })
                .collect::<String>()
        })
        .collect()
}

impl MinefieldCount {
    fn new(width: usize, height: usize) -> MinefieldCount {
        MinefieldCount {
            values: vec![0; (width + 2) * (height + 2)],
            width,
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        (y + 1) * (self.width + 2) + x + 1
    }

    fn flag(&mut self, x: usize, y: usize) {
        let i = self.index(x, y);
        let w = self.width + 2;
        self.values[i] = 9;
        self.values[i + 1] += 1;
        self.values[i - 1] += 1;
        self.values[i + w] += 1;
        self.values[i + w + 1] += 1;
        self.values[i + w - 1] += 1;
        self.values[i - w] += 1;
        self.values[i - w + 1] += 1;
        self.values[i - w - 1] += 1;
    }

    fn get(&self, x: usize, y: usize) -> u8 {
        let i = self.index(x, y);
        self.values[i].min(9)
    }
}
