pub struct PriorityQueue<T: PartialOrd + Copy, E> {
    array: Vec<(T, E)>
}
impl<T: PartialOrd + Copy, E> PriorityQueue<T, E> {

    pub fn array<'a>(self: &'a Self) -> &'a Vec<(T,E)> {
        &self.array
    }

    pub fn insert(self: &mut Self, element: (T,E)) {

        self.array.push(element);

        let mut pos = self.array.len();

        while pos > 1 {

            let parent = self.array.get(pos/2 -1).unwrap();

            let children = self.array.get(pos - 1).unwrap();

            if parent.0 >= children.0 {
                return;
            } else {
                self.array.swap(pos/2 -1, pos - 1);
                pos = pos/2;
            }
        }
    }

    pub fn extract_max_priority(self: &mut Self) -> Option<E> {

        let last = self.array.len() - 1;

        self.array.swap(0, last);

        let max_priority = match self.array.pop() {
            Some(element) => Some(element.1),
            None => None
        };

        if last > 0 {
            Self::max_heapify(1, &mut self.array);
        }

        max_priority
    }


    pub fn new(mut elements: Vec<(T, E)>) -> Self {

        Self::build_max_heap(&mut elements);

        Self {array: elements}
    }

    fn build_max_heap(elements: &mut [(T, E)]) {

        let mut indices = 1..=(elements.len()/2);

        for i in indices.rev() {

            Self::max_heapify(i, elements)
        }
    }

    fn max_heapify(i: usize, elements: &mut [(T, E)]) {

        let parent = elements.get(i - 1).unwrap();

        let left_children = elements.get(2*i - 1);

        let rigth_children = elements.get(2*i);

        match (left_children, rigth_children) {
            (None, None) => {
                return;
            }
            (Some(left), None) => {
                if parent.0 >= left.0 {
                    return;
                } else {
                    elements.swap(i - 1, 2*i - 1);
                    Self::max_heapify(2*i, elements);
                }
            }
            (Some(left), Some(right)) => {
                if parent.0 >= left.0 && parent.0 >= right.0 {
                    return;
                } else {
                    if left.0 >= right.0 {
                        elements.swap(i - 1, 2*i - 1);
                        Self::max_heapify(2*i, elements);
                    } else {
                        elements.swap(i - 1, 2*i);
                        Self::max_heapify(2*i + 1, elements)
                    }
                }
            }
            _ => {panic!("Isso nunca vai acontecer!!!");}
        }
    }

}
