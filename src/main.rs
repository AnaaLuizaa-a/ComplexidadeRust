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

fn main(){
    let lista = vec![1000, 10000, 1000000];
    let vazia: Vec<i32> = vec![];

    print!("{:?}", somar_lista(&lista));
    print!("{:?}", somar_lista(&vazia));
}