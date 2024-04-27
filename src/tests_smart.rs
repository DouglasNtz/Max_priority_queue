use super::smart_priority_queue::{SmartPriorityQueue, PriorityUsize, Element};
use rand;

#[derive(Debug)]
struct AnyObject {
    id: usize
}

#[derive(Debug, Copy, Clone)]
enum Urgency {
    Mininum,
    Medium,
    Maximum
}
impl PriorityUsize for Urgency {
    fn to_usize(self: &Self) -> usize {
        match self {
            Self::Mininum => 1,
            Self::Medium => 2,
            Self::Maximum => 3
        }
    }
}

#[test]
fn insertion_keep_max_heap() {

    let mut elements = vec![];

    let priorities = [Urgency::Maximum, Urgency::Maximum, Urgency::Medium, Urgency::Medium, Urgency::Medium, Urgency::Mininum,
        Urgency::Mininum, Urgency::Maximum, Urgency::Mininum, Urgency::Mininum, Urgency::Mininum, Urgency::Medium,
        Urgency::Medium, Urgency::Mininum, Urgency::Mininum];

    for (i, priority) in priorities.into_iter().enumerate() {

        elements.push(Element::new(priority, Box::new(AnyObject {id: 1001 + i})));
    }

    let mut priority_queue = SmartPriorityQueue::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    assert!(priority_queue.is_max_heap());

    priority_queue.insert(Element::new(Urgency::Maximum, Box::new(AnyObject {id: 1001 + priority_queue.len()})));

    assert!(priority_queue.is_max_heap());

    priority_queue.insert(Element::new(Urgency::Medium, Box::new(AnyObject {id: 1001 + priority_queue.len()})));

    assert!(priority_queue.is_max_heap());

    priority_queue.insert(Element::new(Urgency::Mininum, Box::new(AnyObject {id: 1001 + priority_queue.len()})));

    assert!(priority_queue.is_max_heap());

    priority_queue.insert(Element::new(Urgency::Maximum, Box::new(AnyObject {id: 1001 + priority_queue.len()})));

    assert!(priority_queue.is_max_heap());

}

#[test]
fn delete_keep_max_heap() {

    let mut elements = vec![];

    let priorities = [Urgency::Maximum, Urgency::Maximum, Urgency::Medium, Urgency::Medium, Urgency::Medium, Urgency::Mininum,
        Urgency::Mininum, Urgency::Maximum, Urgency::Mininum, Urgency::Mininum, Urgency::Mininum, Urgency::Medium,
        Urgency::Medium, Urgency::Mininum, Urgency::Mininum];

    for (i, priority) in priorities.into_iter().enumerate() {

        elements.push(Element::new(priority, Box::new(AnyObject {id: 1001 + i})));
    }

    let mut priority_queue = SmartPriorityQueue::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    assert!(priority_queue.is_max_heap());

    priority_queue.delete(1);

    assert!(priority_queue.is_max_heap());

    priority_queue.delete(15);

    assert!(priority_queue.is_max_heap());

    priority_queue.delete(7);

    assert!(priority_queue.is_max_heap());

    priority_queue.delete(9);

    assert!(priority_queue.is_max_heap());

    priority_queue.delete(13);

    assert!(priority_queue.is_max_heap());

    priority_queue.delete(3);

    assert!(priority_queue.is_max_heap());

}

#[test]
fn change_priority_keep_max_heap() {

    let mut elements = vec![];

    let priorities = [Urgency::Maximum, Urgency::Maximum, Urgency::Medium, Urgency::Medium, Urgency::Medium, Urgency::Mininum,
        Urgency::Mininum, Urgency::Maximum, Urgency::Mininum, Urgency::Mininum, Urgency::Mininum, Urgency::Medium,
        Urgency::Medium, Urgency::Mininum, Urgency::Mininum];

    for (i, priority) in priorities.into_iter().enumerate() {

        elements.push(Element::new(priority, Box::new(AnyObject {id: 1001 + i})));
    }

    let mut priority_queue = SmartPriorityQueue::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    assert!(priority_queue.is_max_heap());

    priority_queue.change_priority(2, Urgency::Mininum);

    assert!(priority_queue.is_max_heap());

    priority_queue.change_priority(5, Urgency::Maximum);

    assert!(priority_queue.is_max_heap());

    priority_queue.change_priority(15, Urgency::Medium);

    assert!(priority_queue.is_max_heap());

    priority_queue.change_priority(10, Urgency::Maximum);

    assert!(priority_queue.is_max_heap());

    priority_queue.change_priority(8, Urgency::Medium);

    assert!(priority_queue.is_max_heap());

    priority_queue.change_priority(14, Urgency::Medium);

    assert!(priority_queue.is_max_heap());

}

#[test]
fn test_extract_insert_one_a_one() {

    let mut elements = vec![];

    let priorities = [Urgency::Maximum, Urgency::Maximum, Urgency::Medium, Urgency::Medium, Urgency::Medium, Urgency::Mininum,
                    Urgency::Mininum, Urgency::Maximum, Urgency::Mininum, Urgency::Mininum, Urgency::Mininum, Urgency::Medium,
                    Urgency::Medium, Urgency::Mininum, Urgency::Mininum];

    for (i, priority) in priorities.into_iter().enumerate() {

        elements.push(Element::new(priority, Box::new(AnyObject {id: 1001 + i})));
    }

    let mut priority_queue = SmartPriorityQueue::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1001);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1002);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1008);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1003);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1004);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1005);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1012);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1013);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1006);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1007);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1009);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1010);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1011);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1014);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1015);
}

#[test]
fn test_extract_insert_full() {

    let mut elements = vec![];

    let priorities = [Urgency::Maximum, Urgency::Maximum, Urgency::Medium, Urgency::Medium, Urgency::Medium, Urgency::Mininum,
        Urgency::Mininum, Urgency::Maximum, Urgency::Mininum, Urgency::Mininum, Urgency::Mininum, Urgency::Medium,
        Urgency::Medium, Urgency::Mininum, Urgency::Mininum];

    for (i, priority) in priorities.into_iter().enumerate() {

        elements.push(Element::new(priority, Box::new(AnyObject {id: 1001 + i})));
    }

    let mut priority_queue = SmartPriorityQueue::new(vec![]);

    for element in elements {
        priority_queue.insert(element);
    }

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1001);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1002);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1008);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1003);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1004);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1005);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1012);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1013);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1006);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1007);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1009);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1010);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1011);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1014);

    assert_eq!(priority_queue.extract_max_priority().unwrap().object_retrieve().id, 1015);

}

#[test]
fn test_same_extract() {

    let mut elements1 = vec![];

    let mut elements2 = vec![];

    let priorities = [Urgency::Maximum, Urgency::Maximum, Urgency::Medium, Urgency::Medium, Urgency::Medium, Urgency::Mininum,
        Urgency::Mininum, Urgency::Maximum, Urgency::Mininum, Urgency::Mininum, Urgency::Mininum, Urgency::Medium,
        Urgency::Medium, Urgency::Mininum, Urgency::Mininum];

    for (i, priority) in priorities.into_iter().enumerate() {

        elements1.push(Element::new(priority, Box::new(AnyObject {id: 1001 + i})));
        elements2.push(Element::new(priority, Box::new(AnyObject {id: 1001 + i})));
    }

    let mut priority_queue_one_a_one = SmartPriorityQueue::new(vec![]);

    for element in elements1 {
        priority_queue_one_a_one.insert(element);
    }

    let mut priority_queue_full = SmartPriorityQueue::new(elements2);


    //--------

    let n = priority_queue_one_a_one.len();

    for i in 0..n {

        assert_eq!(
            priority_queue_full.extract_max_priority().unwrap().object_retrieve().id,
            priority_queue_one_a_one.extract_max_priority().unwrap().object_retrieve().id);

    }
}

