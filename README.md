# Max_priority_queue

Implementação de uma max-priority-queue em Rust, baseadas em  max-heap.

A implementação principal, a SmartPriorityQueue conta com diversas propriedades, entre elas a inserção

de elementos e a extração do elemento de prioridade máxima. Além disso ainda podemos deletar elementos e 

podemos mudar a prioridade de um elemento que está na fila.

O exemplo contido em main.rs mostra o funcionamento de um modelos onde os objetos são jobs a serem executados.

A prioridade é um enum chamado Urgency que possui três variantes: Maximum, Medium e Minimum.

O critério de desempate para execução dos jobs, depois de verificada a prioridade, é a ordem de entrada na fila.

Ou seja, primeiro serão exectados os jobs com prioridade Maximum, depois os com Medium e por último os com Minimum.

Dados dois jobs com mesma prioridade, primeiro será executado o que entrou antes na fila.

Para deletar objetos e mudar sua prioridade, optamos por usar a ordem de entrada como chave. Para em vez disso

usarmos o id do objeto, precisaríamos garantir que toda aplicação usará objeto que possuem um id.