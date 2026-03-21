pub fn verificar_primeiro(lista: &[i32]) -> Option<i32>{
    if lista.is_empty() {
        return None;
    }
    return Some(lista[0]);
}

fn main(){
    let lista = vec![1000, 10000, 1000000];
    let vazia: Vec<i32> = vec![];

    print!("{:?}", verificar_primeiro(&lista));
    print!("{:?}", verificar_primeiro(&vazia));
}