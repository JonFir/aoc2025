#[derive(Clone, Debug)]
pub struct Matrix<T> {
    size: usize,
    storage: Vec<Vec<T>>
}

impl<T> Matrix<T> {

    pub fn get(&self, r: usize, c: usize) -> Option<&T> {
        if r < self.size && c < self.size {
            Some(&self.storage[r][c])
        } else {
            None
        }
    }

    pub fn iter(&self) -> MatrixIter<'_, T> {
        MatrixIter::new(self)
    }
    
}
#[derive(Clone, Copy, Debug)]
pub struct MatrixIter<'a, T> {
    row: usize,
    column: usize,
    matrix: &'a Matrix<T>
}

impl<'a, T> Iterator for  MatrixIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let r = self.matrix.get(self.row, self.column)?;
        if self.column + 1 < self.matrix.size {
            self.column += 1;
        } else if self.row + 1 < self.matrix.size {
            self.row += 1;
            self.column = 0;
        } else {
            self.row = self.matrix.size;
            self.column = self.matrix.size;
        }
        Some(r)
    }
}

impl<'a, T> MatrixIter<'a, T> {

    pub fn new(matrix: &'a Matrix<T>) -> Self {
        MatrixIter { row: 0, column: 0, matrix }
    }

    pub fn skip(mut self, limit: u64) -> Self {
        for _ in 0..limit {
            _ = self.next()
        }
        self
    }

    pub fn current(&mut self) -> Option<&T> {
        self.matrix.get(self.row, self.column)
    }

    pub fn index(&self) -> MatrixIndex {
        MatrixIndex { row: self.row, column: self.column}
    }

    pub fn n(&mut self) -> Option<&T> {
        self.row = self.row.checked_add_signed(-1)?;
        self.matrix.get(self.row, self.column)
    }

    pub fn ne(&mut self) -> Option<&T> {
        self.row = self.row.checked_add_signed(-1)?;
        self.column = self.column.checked_add_signed(1)?;
        self.matrix.get(self.row, self.column)
    }

    pub fn e(&mut self) -> Option<&T> {
        self.column = self.column.checked_add_signed(1)?;
        self.matrix.get(self.row, self.column)
    }

    pub fn se(&mut self) -> Option<&T> {
        self.row = self.row.checked_add_signed(1)?;
        self.column = self.column.checked_add_signed(1)?;
        self.matrix.get(self.row, self.column)
    }

    pub fn s(&mut self) -> Option<&T> {
        self.row = self.row.checked_add_signed(1)?;
        self.matrix.get(self.row, self.column)
    }

    pub fn sw(&mut self) -> Option<&T> {
        self.row = self.row.checked_add_signed(1)?;
        self.column = self.column.checked_add_signed(-1)?;
        self.matrix.get(self.row, self.column)
    }

    pub fn w(&mut self) -> Option<&T> {
        self.column = self.column.checked_add_signed(-1)?;
        self.matrix.get(self.row, self.column)
    }

    pub fn nw(&mut self) -> Option<&T> {
        self.row = self.row.checked_add_signed(-1)?;
        self.column = self.column.checked_add_signed(-1)?;
        self.matrix.get(self.row, self.column)
    }
}

pub trait MatrixFrom<T, I> {
    fn from_iter(iterator: I) -> Matrix<T>;
}

impl<T, I> MatrixFrom<T, I> for Matrix<T> where I: Iterator, I::Item: Iterator<Item = T> {
    fn from_iter(iterator: I) -> Matrix<T> {
        let storage = iterator.map(|line| line.collect::<Vec<T>>()).collect::<Vec<Vec<T>>>();
        let size = storage.len();
        Matrix { size, storage }
    }
}

impl From<&str> for Matrix<i64> {
    fn from(value: &str) -> Self {
        let numbers = value.lines().map(|line| line.chars().map(|char| char.to_digit(10).unwrap_or_default() as i64) );
        Matrix::<i64>::from_iter(numbers)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MatrixIndex {
    row: usize,
    column: usize
}