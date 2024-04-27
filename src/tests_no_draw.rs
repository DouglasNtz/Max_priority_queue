use std::thread::park_timeout_ms;
use super::priority_queue_no_draw::PriorityQueueNoDraw;

#[derive(Clone, Debug, PartialEq)]
struct AnyObject {
    id: usize
}

#[test]
fn test_insert_one_a_one() {

    let keys = vec![10, 5, 2, 9, 0, 3, 3, 0, 9, 1, 5, 0, 10, 5, 6];

    let mut elements = vec![];

    for (pos, &key) in keys.iter().enumerate() {
        elements.push((key, AnyObject {id: pos}));
    }

    let mut priority_queue = PriorityQueueNoDraw::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    let max_heap = vec![((10, -1), AnyObject { id: 0 }), ((9, -4), AnyObject { id: 3 }),
                        ((10, -13), AnyObject { id: 12 }), ((9, -9), AnyObject { id: 8 }), ((5, -11), AnyObject { id: 10 }),
                        ((3, -6), AnyObject { id: 5 }), ((6, -15), AnyObject { id: 14 }), ((0, -8), AnyObject { id: 7 }),
                        ((5, -2), AnyObject { id: 1 }), ((0, -5), AnyObject { id: 4 }), ((1, -10), AnyObject { id: 9 }),
                        ((0, -12), AnyObject { id: 11 }), ((2, -3), AnyObject { id: 2 }), ((3, -7), AnyObject { id: 6 }),
                        ((5, -14), AnyObject { id: 13 })];

    assert_eq!(priority_queue.array(), &max_heap);

}

#[test]
fn test_insert_full() {

    let keys = vec![10, 5, 2, 9, 0, 3, 3, 0, 9, 1, 5, 0, 10, 5, 6];

    let mut elements = vec![];

    for (pos, &key) in keys.iter().enumerate() {
        elements.push((key, AnyObject {id: pos}));
    }

    let mut priority_queue = PriorityQueueNoDraw::new(elements);

    let max_heap = vec![((10, -1), AnyObject { id: 0 }), ((9, -4), AnyObject { id: 3 }),
                        ((10, -13), AnyObject { id: 12 }), ((9, -9), AnyObject { id: 8 }), ((5, -11), AnyObject { id: 10 }),
                        ((3, -6), AnyObject { id: 5 }), ((6, -15), AnyObject { id: 14 }), ((0, -8), AnyObject { id: 7 }),
                        ((5, -2), AnyObject { id: 1 }), ((1, -10), AnyObject { id: 9 }), ((0, -5), AnyObject { id: 4 }),
                        ((0, -12), AnyObject { id: 11 }), ((2, -3), AnyObject { id: 2 }), ((5, -14), AnyObject { id: 13 }),
                        ((3, -7), AnyObject { id: 6 })];

    assert_eq!(priority_queue.array(), &max_heap);

}

#[test]
fn test_extract_insert_one_a_one() {

    let keys = vec![10, 5, 2, 9, 0, 3, 3, 0, 9, 1, 5, 0, 10, 5, 6];

    let mut elements = vec![];

    for (pos, &key) in keys.iter().enumerate() {
        elements.push((key, AnyObject {id: pos}));
    }

    let clone = elements.clone();  // cópia, para nosso controle

    let mut priority_queue = PriorityQueueNoDraw::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[0].1);  // primeiro 10

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[12].1);  // segundo 10

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[3].1);  // primeiro 9

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[8].1);  // segundo 9

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[14].1); // 6

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[1].1); // primeiro 5

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[10].1); // segundo 5

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[13].1); // terceiro 5

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[5].1); // primeiro 3

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[6].1); // segundo 3

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[2].1); // 2

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[9].1); // 1

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[4].1); // primeiro 0

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[7].1); // segundo 0

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[11].1); // terceiro 0

}

#[test]
fn test_extract_insert_full() {

    let keys = vec![10, 5, 2, 9, 0, 3, 3, 0, 9, 1, 5, 0, 10, 5, 6];

    let mut elements = vec![];

    for (pos, &key) in keys.iter().enumerate() {
        elements.push((key, AnyObject {id: pos}));
    }

    let clone = elements.clone();  // cópia, para nosso controle

    let mut priority_queue = PriorityQueueNoDraw::new(elements);

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[0].1);  // primeiro 10

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[12].1);  // segundo 10

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[3].1);  // primeiro 9

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[8].1);  // segundo 9

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[14].1); // 6

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[1].1); // primeiro 5

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[10].1); // segundo 5

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[13].1); // terceiro 5

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[5].1); // primeiro 3

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[6].1); // segundo 3

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[2].1); // 2

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[9].1); // 1

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[4].1); // primeiro 0

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[7].1); // segundo 0

    assert_eq!(priority_queue.extract_max_priority().unwrap(), clone[11].1); // terceiro 0

}

#[test]
fn test_same_extract() {

    let keys = vec![10, 5, 2, 9, 0, 3, 3, 0, 9, 1, 5, 0, 10, 5, 6];

    // Primeiro vamos criar uma max priority queue inserindo elemento por elemento

    let mut elements = vec![];

    for (pos, &key) in keys.iter().enumerate() {
        elements.push((key, AnyObject {id: pos}));
    }

    let mut priority_queue_one_a_one = PriorityQueueNoDraw::new(vec![]);

    for element in elements {
        priority_queue_one_a_one.insert(element);
    }

    // Agora vamos criar uma max priority queue inseindo o vetor e depois aplicando build_max_heap

    let mut elements = vec![];

    for (pos, &key) in keys.iter().enumerate() {
        elements.push((key, AnyObject {id: pos}));
    }

    let mut priority_queue_full = PriorityQueueNoDraw::new(elements);

    //--------

    let n = priority_queue_one_a_one.array().len();

    for i in 0..n {

       assert_eq!(priority_queue_full.extract_max_priority(), priority_queue_one_a_one.extract_max_priority());  // primeiro 10

    }
}

// Isert 1 a 1 e insert full geram arvores max heap diferentes, mas tem o mesmo extract.

// Para problema em que cada elemento tem uma prioridade diferente ok. Mas quando existem elementos
// com a mesma prioridade, nada é garantido sobre quem será extraído primeiro.



