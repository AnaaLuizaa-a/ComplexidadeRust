pub fn somar_lista(lista: &[i32]) -> Option<i32>{
    if lista.is_empty() {
        None
    
    } else {
        let mut soma = 0;
        for numero in lista {
            soma += numero;
        }
        Some (soma)
    }
}

