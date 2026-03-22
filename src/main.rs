pub fn pares_com_soma(lista: &[i32], alvo: i32){
    for i in 0..lista.len(){
        for j in (i+1).. lista.len(){
            if lista[i] + lista[j] == alvo {
                print!("({}, {}) ", lista[i], lista[j]);
            }
        }
    }

}

fn main(){
    let lista = vec![1000, 10000, 1000000];

    pares_com_soma(&lista, 10000);
}