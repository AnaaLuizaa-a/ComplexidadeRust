pub fn fibonacci_recursivo(n: u64) -> u64 {
    if n <= 1{
        return n;
    } else {
        return fibonacci_recursivo(n - 1) + fibonacci_recursivo(n - 2);
    }
}

fn main(){
    let _lista = vec![1000, 10000, 1000000];

    println!("Fibonacci de 1000: {}", fibonacci_recursivo(1000));
    println!("Fibonacci de 10000: {}", fibonacci_recursivo(10000));
    println!("Fibonacci de 1000000: {}", fibonacci_recursivo(1000000));
}