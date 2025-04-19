#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
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
            next: self.head.take().map(Box::new)
        };

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) {
        match self.head.take() {
            Some(node) => {
                self.head = node.next.map(|f| *f);
            },
            None => {},
        }
    }

    pub fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
    
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref().map(|boxed_node| &**boxed_node);
        }
        count
    }
}