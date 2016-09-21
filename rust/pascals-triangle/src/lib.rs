pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}

impl PascalsTriangle {
    fn next_row(current_row: &Vec<u32>) -> Vec<u32> {
        let mut row1 = current_row.clone();
        let mut row2 = current_row.clone();
        row1.push(0);
        row1.reverse();
        row2.push(0);
        row1.iter().zip(row2).map(|(x, y)| x+y).collect()
    }

    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<u32>> = Vec::new();
        let mut current = vec![1];
        for i in 0..row_count {
            rows.push(current.clone());
            current = PascalsTriangle::next_row(&current);
        };
        PascalsTriangle{rows: rows}
    }

    pub fn rows(self) -> Vec<Vec<u32>> {
        self.rows
    }
}
