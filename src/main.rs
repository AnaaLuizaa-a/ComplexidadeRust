pub fn imprimir_pares_e_soma(lista: &[i32], alvo: i32){
    for i in 0..lista.len(){
        print!("{}", lista[i]);
        for i in 0..lista.len(){
            for j in (i+1).. lista.len(){
                print!("({}, {}) ", lista[i], lista[j]);
            }
        }
    
    }
}

fn main(){
    let lista = vec![1000, 10000, 1000000];

    imprimir_pares_e_soma(&lista, 10000);
}