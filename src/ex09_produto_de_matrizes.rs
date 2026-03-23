use std::vec;

pub fn produto_de_matrizes(a: &[Vec<i64>], b: &[Vec<i64>]) -> Vec<Vec<i64>> {
    let n: usize = a.len();
    let mut c = vec![vec![0i64;n]; n];
    for i in 0..n {
        for j in 0..n{
            for k in 0..n{
                c[i][j] += a[i][k] * b[k][j];
            }
        }
    }
    c
}

