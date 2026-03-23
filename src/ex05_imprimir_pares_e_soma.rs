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
