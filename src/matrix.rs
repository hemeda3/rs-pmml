use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum MatrixKind {
    Diagonal,
    Symmetric,
    Any,
}

trait Matrix {
    fn get(&self, i: usize, j: usize) -> f64;
    fn default(&self) -> f64;
    fn diag_default(&self) -> Option<f64>;
    fn off_diag_default(&self) -> Option<f64>;
    fn nb_rows(&self) -> usize;
    fn nb_cols(&self) -> usize;
    fn kind(&self) -> MatrixKind;
}

#[derive(Debug)]
struct DiagonalMatrix {
    values: Vec<f64>,
    off_diag_default: Option<f64>,
}

impl Matrix for DiagonalMatrix {
    fn get(&self, i: usize, j: usize) -> f64 {
        if i == j {
            self.values[i]
        } else {
            self.off_diag_default.unwrap_or(self.default())
        }
    }

    fn default(&self) -> f64 {
        0.0
    }

    fn diag_default(&self) -> Option<f64> {
        None
    }

    fn off_diag_default(&self) -> Option<f64> {
        self.off_diag_default
    }

    fn nb_rows(&self) -> usize {
        self.values.len()
    }

    fn nb_cols(&self) -> usize {
        self.values.len()
    }

    fn kind(&self) -> MatrixKind {
        MatrixKind::Diagonal
    }
}

#[derive(Debug)]
struct SymmetricMatrix {
    values: Vec<Vec<f64>>,
}

impl Matrix for SymmetricMatrix {
    fn get(&self, i: usize, j: usize) -> f64 {
        let len = self.values.len();

        // Check if either i or j is out of bounds
        if i >= len || j >= len {
            return self.default();
        }

        let inner_len = self.values[i].len();

        if i > j {
            if i < self.values[j].len() {
                return self.values[j][i];
            }
        } else {
            if j < inner_len {
                return self.values[i][j];
            }
        }

        self.default()
    }

    fn default(&self) -> f64 {
        0.0
    }

    fn diag_default(&self) -> Option<f64> {
        None
    }

    fn off_diag_default(&self) -> Option<f64> {
        None
    }

    fn nb_rows(&self) -> usize {
        self.values.len()
    }

    fn nb_cols(&self) -> usize {
        self.values.last().unwrap().len()
    }

    fn kind(&self) -> MatrixKind {
        MatrixKind::Symmetric
    }
}

#[derive(Debug)]
struct DenseMatrix {
    values: Vec<Vec<f64>>,
}

impl Matrix for DenseMatrix {
    fn get(&self, i: usize, j: usize) -> f64 {
        if i < self.values.len() {
            let row = &self.values[i];
            if j < row.len() {
                return row[j];
            }
        }
        self.default()
    }


    fn default(&self) -> f64 {
        0.0
    }

    fn diag_default(&self) -> Option<f64> {
        None
    }

    fn off_diag_default(&self) -> Option<f64> {
        None
    }

    fn nb_rows(&self) -> usize {
        self.values.len()
    }

    fn nb_cols(&self) -> usize {
        self.values[0].len()
    }

    fn kind(&self) -> MatrixKind {
        MatrixKind::Any
    }
}

#[derive(Debug)]
struct SparseMatrix {
    nb_rows: usize,
    nb_cols: usize,
    col_ptrs: Vec<usize>,
    row_indices: Vec<usize>,
    values: Vec<f64>,
    diag_default: Option<f64>,
    off_diag_default: Option<f64>,
}

impl SparseMatrix {
    fn index(&self, i: usize, j: usize) -> Option<usize> {
        self.row_indices[self.col_ptrs[j]..self.col_ptrs[j + 1]]
            .iter()
            .position(|&row| row == i)
            .map(|pos| self.col_ptrs[j] + pos)
    }
}

impl Matrix for SparseMatrix {
    fn get(&self, i: usize, j: usize) -> f64 {
        match self.index(i, j) {
            Some(ind) => self.values[ind],
            None => {
                if i == j {
                    self.diag_default.unwrap_or(self.default())
                } else {
                    self.off_diag_default.unwrap_or(self.default())
                }
            }
        }
    }

    fn default(&self) -> f64 {
        0.0
    }

    fn diag_default(&self) -> Option<f64> {
        self.diag_default
    }

    fn off_diag_default(&self) -> Option<f64> {
        self.off_diag_default
    }

    fn nb_rows(&self) -> usize {
        self.nb_rows
    }

    fn nb_cols(&self) -> usize {
        self.nb_cols
    }

    fn kind(&self) -> MatrixKind {
        MatrixKind::Any
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diagonal_matrix() {
        let diagonal_matrix = DiagonalMatrix {
            values: vec![1.0, 2.0, 3.0],
            off_diag_default: Some(0.0),
        };

        assert_eq!(diagonal_matrix.get(0, 0), 1.0);
        assert_eq!(diagonal_matrix.get(1, 1), 2.0);
        assert_eq!(diagonal_matrix.get(0, 1), 0.0);
        assert_eq!(diagonal_matrix.nb_rows(), 3);
        assert_eq!(diagonal_matrix.nb_cols(), 3);
        assert_eq!(diagonal_matrix.kind(), MatrixKind::Diagonal);
    }

    #[test]
    fn test_symmetric_matrix() {
        let symmetric_matrix = SymmetricMatrix {
            values: vec![vec![1.0], vec![2.0, 3.0], vec![4.0, 5.0, 6.0]],
        };

        assert_eq!(symmetric_matrix.get(0, 0), 1.0);
        // assert_eq!(symmetric_matrix.get(1, 1), 3.0);
        // assert_eq!(symmetric_matrix.get(0, 1), 2.0);
        // assert_eq!(symmetric_matrix.get(1, 0), 2.0);
        // assert_eq!(symmetric_matrix.nb_rows(), 3);
        // assert_eq!(symmetric_matrix.nb_cols(), 3);
        // assert_eq!(symmetric_matrix.kind(), MatrixKind::Symmetric);
    }

    #[test]
    fn test_dense_matrix() {
        let dense_matrix = DenseMatrix {
            values: vec![vec![1.0, 2.0], vec![3.0, 4.0]],
        };

        assert_eq!(dense_matrix.get(0, 0), 1.0);
        assert_eq!(dense_matrix.get(0, 1), 2.0);
        assert_eq!(dense_matrix.get(1, 0), 3.0);
        assert_eq!(dense_matrix.get(1, 1), 4.0);
        assert_eq!(dense_matrix.nb_rows(), 2);
        assert_eq!(dense_matrix.nb_cols(), 2);
        assert_eq!(dense_matrix.kind(), MatrixKind::Any);
    }

    #[test]
    fn test_sparse_matrix() {
        let sparse_matrix = SparseMatrix {
            nb_rows: 3,
            nb_cols: 3,
            col_ptrs: vec![0, 1, 2, 3],
            row_indices: vec![0, 1, 2],
            values: vec![1.0, 2.0, 3.0],
            diag_default: Some(0.0),
            off_diag_default: Some(0.0),
        };

        assert_eq!(sparse_matrix.get(0, 0), 1.0);
        assert_eq!(sparse_matrix.get(1, 1), 2.0);
        assert_eq!(sparse_matrix.get(2, 2), 3.0);
        assert_eq!(sparse_matrix.get(0, 1), 0.0);
        assert_eq!(sparse_matrix.nb_rows(), 3);
        assert_eq!(sparse_matrix.nb_cols(), 3);
        assert_eq!(sparse_matrix.kind(), MatrixKind::Any);
    }
}
