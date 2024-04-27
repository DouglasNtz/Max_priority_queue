use Max_priority_queue::{PriorityQueue, PriorityQueueNoDraw};

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct AnyObject {
    senha: usize
}

fn main() {

    // Exemplo de elementos que serão inseridos numa fila e depois serão extraídos.

    let keys = vec![10, 5, 2, 9, 0, 3, 3, 0, 9, 1, 5, 0, 10, 5, 6];

    let mut elements = vec![];

    for (pos, &key) in keys.iter().enumerate() {
        elements.push((key, AnyObject {senha: pos + 1}));
    }

    println!("Elementos por ordem de inserção: par prioridade-elemento");
    for element in &elements {
        println!("{:?}", element);
    }

    let mut h = std::collections::HashMap::new();
    for (key, object) in &elements {
        h.insert(object.clone(), *key);
    }

    let mut priority_queue = PriorityQueueNoDraw::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    println!("Elementos por ordem de extração:");
    while let Some(element) = priority_queue.extract_max_priority() {
        println!("{:?}", (h.get(&element).unwrap(), element));
    }

}
