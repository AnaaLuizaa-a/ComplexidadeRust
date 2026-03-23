pub fn verificar_primeiro(lista: &[i32]) -> Option<i32>{
    if lista.is_empty() {
        return None;
    }
    return Some(lista[0]);
}

