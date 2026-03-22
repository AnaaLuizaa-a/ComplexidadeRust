pub fn busca_binaria(lista: &[i32], alvo: i32) -> Option<usize> {
    let mut esquerda: isize = 0;
    let mut direita: isize = (lista.len() as isize) - 1;

    while esquerda <= direita {
        let meio = esquerda + direita / 2;
        if lista[meio as usize] == alvo {
            return Some(meio as usize);
        } else if lista[meio as usize] < alvo {
            esquerda = meio + 1;
        } else {
            direita = meio - 1;
        }
    }
    None
}

fn main(){
    let lista = vec![1000, 10000, 1000000];

    print!("{:?}", busca_binaria(&lista,10000));
}