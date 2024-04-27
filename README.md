# Max_priority_queue

Implementação de uma max-priority-queue em Rust, baseadas em  max-heap.

Fiz duas implementaçõe:


Uma em que a max-priority-queue não trata desempates. Nesse caso, dois elementos com

mesma prioridade inseridos em uma certa ordem, podem ser extraídos numa ordem contrária.


A outra gera uma chave secundária automática, por debaixo dos panos. Ela garante que dois elementos

com a mesma prioridade, serão extraídos na mesma ordem que foram inseridos. Ou seja, para os elementos

empatados em prioridade, é extraído primeiro aquele que entrou primeiro na fila.


Os módulos de testes provam que a extração é sempre correta. Ou seja, é de fato o que o algoritmo deve fazer.


Outro ponto demonstrado nos testes, é que dado um conjunto de elementos, inserí-los um a um gera uma max-heap que pode

ser diferente da max-heap gerada inserindo o array de elementos inteiro e então aplicando build_max_heap para transformar

o array numa max-heap. 

Porém, o mais incrível é que a ordem de extração de cada uma dessas max-heap é idêntica!!! 

Até mesmo para a max-priority-queue que não trata desempates, as extrações dessas max-heap distintas tem extrações iguais. 

Mas é claro que a extração da max-priority-queue que não trata desempates pode ser diferente da que trata desempates.


