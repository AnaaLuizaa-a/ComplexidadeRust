# Atividade - Reescrita de Algoritmos em Rust

**Disciplina:** Estruturas de Dados e Análise de Algoritmos  
**Professor:** Alexandre de Oliveira  
**Aluna:** Ana Luiza Mattos de Carvalho  
**RA:** 124114111  
**Repositório BigO em python:** https://github.com/AnaaLuizaa-a/Analise_BigO

---

## Exercício 1 - Verificar Primeiro

**Complexidade:** O(1)

**Lógica do algoritmo:**  
Verifica se a lista está vazia e, caso não esteja, retorna o primeiro elemento diretamente pelo índice. Nenhum laço é executado.

**Justificativa da complexidade:**  
São realizadas apenas operações de tempo constante: verificar se a lista está vazia e acessar o elemento no índice 0. O tempo de execução não depende do tamanho da entrada.

---

## Exercício 2 - Somar Lista

**Complexidade:** O(n)

**Lógica do algoritmo:**  
Percorre todos os elementos da lista um a um, acumulando a soma em uma variável, e retorna o total ao final.

**Justificativa da complexidade:**  
Há um único laço que itera sobre todos os n elementos da lista. O número de operações cresce linearmente com o tamanho da entrada.

---

## Exercício 3 - Busca Binária

**Complexidade:** O(log n)

**Lógica do algoritmo:**  
Divide o intervalo de busca ao meio a cada iteração, descartando metade dos elementos restantes até encontrar o alvo ou esgotar a lista.

**Justificativa da complexidade:**  
A cada passo o espaço de busca é cortado pela metade: n → n/2 → n/4 → ... → 1. São necessários log₂(n) passos para ir de n até 1. Para n = 1.000.000, isso representa apenas ~20 iterações.

---

## Exercício 4 - Pares com Soma

**Complexidade:** O(n²)

**Lógica do algoritmo:**  
Usa dois laços aninhados para comparar todos os pares possíveis de elementos da lista, imprimindo os pares cuja soma é igual ao alvo.

**Justificativa da complexidade:**  
O primeiro laço percorre os n elementos da lista. Para cada elemento, o segundo laço percorre os elementos restantes. O número total de comparações é proporcional a n², pois cada par (i, j) é verificado uma vez.

---

## Exercício 5 - Imprimir Pares e Pares

**Complexidade:** O(n²)

**Lógica do algoritmo:**  
Possui dois blocos sequenciais: o primeiro percorre a lista uma vez imprimindo cada elemento (O(n)); o segundo usa dois laços aninhados para imprimir todos os pares possíveis (O(n²)).

**Justificativa da complexidade:**  
Pela regra da soma, O(n) + O(n²) = O(n²), pois o termo dominante é o quadrático. O bloco com laços aninhados determina a complexidade final do algoritmo.

---

## Exercício 6 - Potências de Dois

**Complexidade:** O(log n)

**Lógica do algoritmo:**  
Começa com i = 1 e multiplica por 2 a cada iteração, imprimindo o valor até que i alcance ou ultrapasse n.

**Justificativa da complexidade:**  
A cada iteração o valor de i é multiplicado por 2, o que significa que o número de iterações necessárias para alcançar n é proporcional ao logaritmo de n na base 2. O tempo de execução cresce de forma logarítmica em relação à entrada.

---

## Exercício 7 - Fibonacci Recursivo

**Complexidade:** O(2ⁿ)

**Lógica do algoritmo:**  
Calcula o n-ésimo número de Fibonacci por meio de recursão, fazendo duas chamadas recursivas a cada passo (para n-1 e n-2), até atingir o caso base (n ≤ 1).

**Justificativa da complexidade:**  
A cada chamada recursiva são feitas duas novas chamadas, formando uma árvore de recursão com profundidade n. O número total de chamadas é aproximadamente 2ⁿ, tornando o algoritmo exponencial. Para n > 40, o tempo de execução se torna perceptível.

---

## Exercício 8 - Ordenação Bolha (Bubble Sort)

**Complexidade:** O(n²)

**Lógica do algoritmo:**  
Percorre a lista repetidamente comparando elementos adjacentes e trocando-os quando estão fora de ordem, até que a lista esteja completamente ordenada.

**Justificativa da complexidade:**  
O algoritmo possui dois laços aninhados: o externo executa n vezes e o interno executa até n-i-1 vezes. O número total de comparações é proporcional a n², tornando o algoritmo quadrático.

---

## Exercício 9 - Produto de Matrizes

**Complexidade:** O(n³)

**Lógica do algoritmo:**  
Calcula o produto de duas matrizes n×n usando três laços aninhados: para cada célula (i, j) da matriz resultado, acumula o produto dos elementos correspondentes das linhas de A e colunas de B.

**Justificativa da complexidade:**  
Há três laços aninhados, cada um iterando n vezes. O número total de operações é n × n × n = n³, resultando em complexidade cúbica.

---

## Exercício 10 - Merge Sort

**Complexidade:** O(n log n)

**Lógica do algoritmo:**  
Divide recursivamente a lista ao meio até obter sublistas de tamanho 1, depois combina (merge) as sublistas de forma ordenada até reconstruir a lista completa.

**Justificativa da complexidade:**  
A divisão da lista ao meio ocorre log n vezes (profundidade da recursão). A etapa de combinação (merge) percorre todos os n elementos a cada nível. Multiplicando: n × log n operações no total, resultando em O(n log n).

---

## Tabela de Complexidades Big-O

| Complexidade | Descrição          | Algoritmo desta atividade     |
|:---:|:---:|:---:|
| O(1)         | Tempo constante    | Exercício 1 — Verificar Primeiro |
| O(log n)     | Tempo logarítmico  | Exercícios 3 e 6              |
| O(n)         | Tempo linear       | Exercício 2 — Somar Lista     |
| O(n log n)   | Tempo linearítmico | Exercício 10 — Merge Sort     |
| O(n²)        | Tempo quadrático   | Exercícios 4, 5 e 8           |
| O(n³)        | Tempo cúbico       | Exercício 9 — Produto de Matrizes |
| O(2ⁿ)        | Tempo exponencial  | Exercício 7 — Fibonacci Recursivo |
