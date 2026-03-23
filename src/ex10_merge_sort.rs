
pub fn merge_sort(lista: Vec<i32>) -> Vec<i32> {
    if lista.len() <= 1 {
        return lista;
    }

    let meio = lista.len() /2;
    let esquerda = merge_sort(lista[..meio].to_vec());
    let direita = merge_sort(lista[meio..].to_vec());

    let mut resultado = Vec::new();
    let (mut i, mut j) = (0, 0);
    while i < esquerda.len() && j < direita.len() {
        if esquerda[i] <= direita[j] {
            resultado.push(esquerda[i]);
            i+=1;
        } else { 
            resultado.push(direita[j]);
            j+=1
        }
    }
    resultado.extend_from_slice(&esquerda[i..]);
    resultado.extend_from_slice(&direita[j..]);
    return resultado;
}
