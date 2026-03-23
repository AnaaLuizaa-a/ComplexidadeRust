mod ex01_verificar_primeiro;
mod ex02_somar_lista;
mod ex03_busca_binaria;
mod ex04_pares_com_soma;
mod ex05_imprimir_pares_e_soma;
mod ex06_potencias_de_dois;
mod ex07_fibonacci_recursivo;
mod ex08_ordenacao_bolha;
mod ex09_produto_de_matrizes;
mod ex10_merge_sort;

use ex01_verificar_primeiro::verificar_primeiro;
use ex02_somar_lista::somar_lista;
use ex03_busca_binaria::busca_binaria;
use ex04_pares_com_soma::pares_com_soma;
use ex05_imprimir_pares_e_soma::imprimir_pares_e_soma;
use ex06_potencias_de_dois::potencias_de_dois;
use ex07_fibonacci_recursivo::fibonacci_recursivo;
use ex08_ordenacao_bolha::ordenacao_bolha;
use ex09_produto_de_matrizes::produto_de_matrizes;
use ex10_merge_sort::merge_sort;

fn main() {
    println!("=== Ex01 ===");
    println!("{:?}", verificar_primeiro(&[1, 2, 3]));

    println!("\n=== Ex02 ===");
    println!("{:?}", somar_lista(&[1, 2, 3]));

    println!("\n=== Ex03 ===");
    println!("{:?}", busca_binaria(&[1, 2, 3, 4, 5], 3));

    println!("\n=== Ex04 ===");
    pares_com_soma(&[1, 2, 3, 4], 5);

    println!("\n=== Ex05 ===");
    imprimir_pares_e_soma(&[1, 2, 3],5);

    println!("\n=== Ex06 ===");
    potencias_de_dois(64);

    println!("\n=== Ex07 ===");
    println!("{}", fibonacci_recursivo(10));

    println!("\n=== Ex08 ===");
    let mut lista = vec![5, 3, 8, 1];
    ordenacao_bolha(&mut lista);
    println!("{:?}", lista);

    println!("\n=== Ex09 ===");
    let a = vec![vec![1, 2], vec![3, 4]];
    let b = vec![vec![5, 6], vec![7, 8]];
    println!("{:?}", produto_de_matrizes(&a, &b));

    println!("\n=== Ex10 ===");
    println!("{:?}", merge_sort(vec![64, 34, 25, 12]));
}