#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Box<Node<T>>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T> List<T> {
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn push(&mut self, value: T) {
        let new_node: Node<T> = Node{
            value,
            next: self.head.take()
        };

        self.head = Some(Box::new(new_node));
    }

    pub fn pop(&mut self) {
        match self.head.take() {
            Some(node) => {
                self.head = node.next;
            },
            None => {},
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;

        while let Some(Node) = current{
            count += 1;
            current = &Node.next;
        }
        count
    }
}