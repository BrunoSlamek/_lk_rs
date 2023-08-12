
#[derive(Debug)]
struct LinkedListNode<T> {
    data: Box<T>,
    next: Option<Box<LinkedListNode<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<LinkedListNode<T>>>,
}

impl<T: std::fmt::Debug> LinkedList<T> {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn push(&mut self, data: T) {
        let new_node: Box<LinkedListNode<T>> = Box::new(LinkedListNode {
            data: Box::new(data),
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    fn append(&mut self, data: T) {
        let new_node: Box<LinkedListNode<T>> = Box::new(LinkedListNode {
            data: Box::new(data),
            next: None,
        });

        if self.head.is_none() {
            self.head = Some(new_node);
            return;
        }

        let mut current_node: &mut Option<Box<LinkedListNode<T>>> = &mut self.head;

        while let Some(node) = current_node {
            if node.next.is_none() {
                node.next = Some(new_node);
                return;
            }

            current_node = &mut node.next;
        }
    }

    fn push_after_index(&mut self, index: usize, data: Box<T>) {
        if index == 0 {
            self.push(*data);
            return;
        }

        let mut current_node: &mut Option<Box<LinkedListNode<T>>> = &mut self.head;
        let mut remaining_index = index;

        loop {
            
            println!("CURRENT NODE -> {:?}", current_node);

            if let Some(node) = current_node {
                if remaining_index == 1 {
                    let next_node: Option<Box<LinkedListNode<T>>> = node.next.take();
                    node.next = Some(Box::new(LinkedListNode {
                        data,
                        next: next_node,
                    }));
                    return;
                }
                remaining_index -= 1;
                current_node = &mut node.next;
            } else {
                self.append(*data);
                return;
            }
        }
    }

    fn print(&self) {
        let mut current_node: &Option<Box<LinkedListNode<T>>> = &self.head;

        while let Some(node) = current_node {
            println!("{:?}", *node.data);
            current_node = &node.next;
        }
    }
}

fn main() {
    let mut list: LinkedList<i32> = LinkedList::new();

    // println!("HEAD {:?}", list.head);

    list.push(1);
    list.push(2);
    list.append(3);
    list.append(4);
    list.append(5);
    list.append(6);
    list.append(7);

    list.push_after_index(5, Box::new(20));

    // list.push_after_index(1, Box::new(30));
    
    /*list.push_after_index(3, Box::new(5));

    list.push_after_index(4, Box::new(17));
    list.push_after_index(4, Box::new(15));

    list.push(10); */

    list.print();
}