pub fn ordenacao_bolha(lista: &mut [i32]) {
    let n = lista.len();
    for i in 0..n {
        for j in 0..n-i-1 {
            if lista[j] > lista[j+1] {
                lista.swap(j,j+1);
            }
        }
    }
}

fn main(){
    let mut lista = vec![1000, 10000, 1000000];
    ordenacao_bolha(&mut lista);
}