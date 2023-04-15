pub struct PascalsTriangle(usize);

impl PascalsTriangle {
    pub fn new(row_count: usize) -> Self {
        Self(row_count)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let row_count = self.0;
        if row_count == 0 {
            return vec![];
        }
        let mut result: Vec<Vec<u32>> = vec![];
        let mut prev: Vec<u32> = vec![1];
        for size in 2..(row_count + 1) {
            let row: Vec<u32> = (0..size)
                .map(|i| match i {
                    0 => 1,
                    i if i == size - 1 => 1,
                    i => prev[i - 1] + prev[i],
                })
                .collect();
            result.push(prev);
            prev = row;
        }
        result.push(prev);
        result
    }
}
