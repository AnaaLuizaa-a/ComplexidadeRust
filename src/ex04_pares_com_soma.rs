pub fn pares_com_soma(lista: &[i32], alvo: i32){
    for i in 0..lista.len(){
        for j in (i+1).. lista.len(){
            if lista[i] + lista[j] == alvo {
                print!("({}, {}) ", lista[i], lista[j]);
            }
        }
    }

}

