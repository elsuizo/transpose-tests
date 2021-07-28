pub fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    assert!(!v.is_empty());
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

use std::ops::{Index, IndexMut};

#[derive(Debug, Default)]
pub struct Matrix<T> {
    size: (usize, usize),
    data: Vec<T>,
}

impl<T: Default + Copy> Matrix<T> {
    fn zeros(x: usize, y: usize) -> Self {
        Self {
            size: (x, y),
            data: vec![T::default(); x * y],
        }
    }

    pub fn new(x: usize, y: usize, data: Vec<T>) -> Self {
        Self { size: (x, y), data }
    }

    fn rows(&self) -> usize {
        self.size.0
    }

    fn cols(&self) -> usize {
        self.size.1
    }

    pub fn transpose(&self) -> Self {
        let mut result = Self::zeros(self.rows(), self.cols());
        for i in 0..self.rows() {
            for j in 0..self.cols() {
                result[(i, j)] = self[(j, i)];
            }
        }
        result
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;
    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        let (w, _) = self.size;
        &self.data[y * w + x]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        let (w, _) = self.size;
        &mut self.data[y * w + x]
    }
}
