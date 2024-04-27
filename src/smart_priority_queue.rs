use std::cmp::Ordering;
use std::collections::HashMap;

pub trait PriorityUsize {
    fn to_usize(self: &Self) -> usize;
}

#[derive(Debug)]
pub struct Element<T: PriorityUsize, E> {
    priority: T,
    object: Box<E>
}
impl<T: PriorityUsize, E> Element<T, E> {

    pub fn new(priority: T, object: Box<E>) -> Self {
        Self {priority, object}
    }
    pub fn object_retrieve(self: Self) -> E {
        *self.object
    }
}

#[derive(Debug)]
struct QueueElement {
    priority: usize,
    entry_order: usize,
}

impl PartialEq for QueueElement {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority && self.entry_order == other.entry_order
    }
}

impl PartialOrd for QueueElement {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {

        if self.priority < other.priority || (self.priority == other.priority && self.entry_order > other.entry_order) {
            Some(Ordering::Less)
        } else if self.priority > other.priority || (self.priority == other.priority && self.entry_order < other.entry_order) {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}

pub struct SmartPriorityQueue<T: PriorityUsize, E> {
    map: HashMap<usize, Element<T, E>>,
    array: Vec<QueueElement>,
    count: usize
}
impl<T: PriorityUsize, E> SmartPriorityQueue<T, E> {

    pub fn entry_orders_vec(self: &Self) -> Vec<usize> {
        let mut v = vec![];
        for queue_element in &self.array {
            v.push(queue_element.entry_order)
        }
        v
    }

    pub fn len(self: &Self) -> usize {
        self.array.len()
    }

    pub fn insert(self: &mut Self, element: Element<T, E>) {

        self.count += 1;

        let queue_element = QueueElement {priority: element.priority.to_usize(), entry_order: self.count};

        self.map.insert(self.count, element);

        self.array.push(queue_element);

        let mut pos = self.array.len();

        while pos > 1 {

            let parent = self.array.get(pos/2 - 1).unwrap();

            let children = self.array.get(pos - 1).unwrap();

            if parent >= children {
                return;
            } else {
                self.array.swap(pos/2 -1, pos - 1);
                pos = pos/2;
            }
        }
    }

    pub fn delete(self: &mut Self, entry_order: usize) {

        match self.map.remove(&entry_order) {
            None => {
                println!("Elemento com ordem de entrada {entry_order} não consta no map");
                return;
            }
            _ => {},
        }

        let last = self.array.len() - 1;

        let mut delete_position= last + 1;

        for (position, queue_element) in self.array.iter().enumerate() {
            if queue_element.entry_order == entry_order {
                delete_position = position;
                break;
            }
        }

        if delete_position == last + 1 {
            panic!("Elemento foi encontrado no map, mas não constava no vetor... bug!!!");
        }

        self.array.swap(delete_position, last);

        self.array.pop().unwrap_or_else(|| panic!("Como não exsite último elemento pra dropar??? bug!!!"));

        if last > 0 {
            Self::max_heapify(delete_position + 1, &mut self.array);
        }

    }

    pub fn extract_max_priority(self: &mut Self) -> Option<Element<T, E>> {

        if self.array.is_empty() {
            return None;
        }

        let last = self.array.len() - 1;

        self.array.swap(0, last);

        let max_priority = match self.array.pop() {

            Some(queue_element) => self.map.remove(&queue_element.entry_order),

            None => None
        };

        if last > 0 {
            Self::max_heapify(1, &mut self.array);
        }

        max_priority
    }

    pub fn change_priority(self: &mut Self, entry_order: usize, new_priority: T) {

        let old_priority_usize;

        let new_priority_usize;

        match self.map.get_mut(&entry_order) {

            Some(element) => {
                element.priority = new_priority;
                new_priority_usize = element.priority.to_usize();
            },

            None => {
                println!("Elemento com ordem de entrada {entry_order} não consta no map");
                return;
            }
        }

        let mut change_position = self.array.len();

        for (position, queue_element) in self.array.iter().enumerate() {
            if queue_element.entry_order == entry_order {
                change_position = position;
                break;
            }
        }

        if change_position == self.array.len() {
            panic!("Elemento foi encontrado no map, mas não constava no vetor... bug!!!");
        }

        old_priority_usize = self.array.get(change_position).unwrap().priority;

        self.array.get_mut(change_position).unwrap().priority = new_priority_usize;

        if new_priority_usize < old_priority_usize {

            Self::max_heapify(change_position + 1, &mut self.array);

        } else if new_priority_usize > old_priority_usize {

            let mut pos = change_position + 1;

            while pos > 1 {

                let parent = self.array.get(pos/2 - 1).unwrap();

                let children = self.array.get(pos - 1).unwrap();

                if parent >= children {
                    return;
                } else {
                    self.array.swap(pos/2 -1, pos - 1);
                    pos = pos/2;
                }
            }

        }

    }


    pub fn new(elements: Vec<Element<T, E>>) -> Self {

        let mut count = 0;

        let mut queue_elements = vec![];

        let mut map = HashMap::new();

        for element in elements {
            count += 1;
            queue_elements.push(QueueElement {priority: element.priority.to_usize(), entry_order: count});
            map.insert(count, element);
        }

        Self::build_max_heap(&mut queue_elements);

        Self {map, array: queue_elements, count}
    }

    fn build_max_heap(queue_elements: &mut [QueueElement]) {

        let mut indices = 1..=(queue_elements.len()/2);

        for i in indices.rev() {

            Self::max_heapify(i, queue_elements)
        }
    }

    fn max_heapify(i: usize, queue_elements: &mut [QueueElement]) {

        let parent = queue_elements.get(i - 1).unwrap();

        let left_children = queue_elements.get(2*i - 1);

        let rigth_children = queue_elements.get(2*i);

        match (left_children, rigth_children) {
            (None, None) => {
                return;
            }
            (Some(left), None) => {
                if parent >= left {
                    return;
                } else {
                    queue_elements.swap(i - 1, 2*i - 1);
                    Self::max_heapify(2*i, queue_elements);
                }
            }
            (Some(left), Some(right)) => {
                if parent >= left && parent >= right {
                    return;
                } else {
                    if left >= right {
                        queue_elements.swap(i - 1, 2*i - 1);
                        Self::max_heapify(2*i, queue_elements);
                    } else {
                        queue_elements.swap(i - 1, 2*i);
                        Self::max_heapify(2*i + 1, queue_elements)
                    }
                }
            }
            _ => {panic!("Isso nunca vai acontecer!!!");}
        }
    }

    pub fn is_max_heap(self: &Self) -> bool {

        let mut indices = 1..=(self.len()/2);

        for i in indices.rev() {

            let parent = self.array.get(i - 1).unwrap();

            let left_children = self.array.get(2*i - 1);

            let rigth_children = self.array.get(2*i);

            match (left_children, rigth_children) {

                (None, None) => {
                    continue;
                }
                (Some(left), None) => {
                    if parent >= left {
                        continue;
                    } else {
                        return false;
                    }
                }
                (Some(left), Some(right)) => {
                    if parent >= left && parent >= right {
                        continue;
                    } else {
                        return false;
                    }
                }
                _ => {panic!("Isso nunca vai acontecer!!!");}
            }
        }

        true
    }

}
