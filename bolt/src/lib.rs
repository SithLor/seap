pub fn add(left: usize, right: usize) -> usize {
    left + right
}

pub struct _2D_Array {
    pub data: Vec<Vec<usize>>,
}
impl _2D_Array {
    pub fn new(rows: usize, cols: usize) -> Self {
        let mut data: Vec<Vec<usize>> = Vec::with_capacity(rows);
        for _ in 0..rows {
            let mut row: Vec<usize> = Vec::with_capacity(cols);
            for _ in 0..cols {
                row.push(0);
            }
            data.push(row);
        }
        Self { data }
    }
    pub fn get(&self, row: usize, col: usize) -> usize {
        self.data[row][col]
    }
    pub fn set(&mut self, row: usize, col: usize, value: usize) {
        self.data[row][col] = value;
    }
    
    pub fn rows(&self) -> usize {
        self.data.len()
    }
    pub fn cols(&self) -> usize {
        self.data[0].len()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
