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

fn main(){
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    let c = produto_de_matrizes(&a, &b);
    println!("{:?}", c);
}