//METHOD 


// 

struct _2DArray<T> {
    arr: Vec<Vec<T>>,
    rows: usize,
    cols: usize,
    heap_size: usize,
}
impl _2DArray<T> {
    pub fn new(rows: usize, cols: usize) -> _2DArray<T> {
        let mut arr: Vec<Vec<_>> = Vec::with_capacity(rows);
        for _ in 0..rows {
            arr.push(Vec::with_capacity(cols));
        }
        _2DArray {
            arr,
            rows,
            cols,
            heap_size: 0,
        }
    }
    pub fn get(&self, i: usize, j: usize) -> Option<&T> {
        if i < self.rows && j < self.cols {
            Some(&self.arr[i][j])
        } else {
            None
        }
    }
    pub fn set(&mut self, i: usize, j: usize, val: T) -> bool {
        if i < self.rows && j < self.cols {
            self.arr[i][j] = val;
            true
        } else {
            false
        }
    }
    //get the size in bytes of the array
    pub fn heap_size(&self) -> usize {
        self.rows * self.cols * std::mem::size_of::<T>()
    }
    //clear the array 
    pub fn clear(&mut self) {
        for i in 0..self.rows {
            for j in 0..self.cols {
                self.arr[i][j] = T::default();
            }
        }
    }
    //clear_row clears the row at index i
    pub fn clear_row(&mut self, i: usize) {
        for j in 0..self.cols {
            self.arr[i][j] = T::default();
        }
    }
    //clear_col clears the column at index j
    pub fn clear_col(&mut self, j: usize) {
        for i in 0..self.rows {
            self.arr[i][j] = T::default();
        }
    }
    //clear_range clears the range of rows and columns
    pub fn clear_range(&mut self, i: usize, j: usize, rows: usize, cols: usize) {
        for x in i..i + rows {
            for y in j..j + cols {
                self.arr[x][y] = T::default();
            }
        }
    }
    pub fn drop(&mut self) {
        self.arr = Vec::new();
        self.rows = 0;
        self.cols = 0;
        self.heap_size = 0;
    }
}