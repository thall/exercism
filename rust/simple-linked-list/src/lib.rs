use std::fmt::Debug;

#[derive(Debug)]
pub struct SimpleLinkedList<T> {
    head: Option<Box<Node<T>>>
}

#[derive(Debug)]
struct Node<T> {
    node: T,
    next: Option<Box<Node<T>>>,
}


impl<T> SimpleLinkedList<T>
{
    pub fn new() -> Self {
        SimpleLinkedList {
            head: None
        }
    }

    pub fn len(&self) -> usize {
        let mut e = &self.head;
        let mut len: usize = 0;
        while e.is_some() {
            len = len + 1;
            e = &e.as_ref().unwrap().next;
        }

        len
    }

    pub fn push(&mut self, _element: T) {
        let node = Node {
            node: _element,
            next: self.head.take(),
        };
        self.head = Some(Box::new(node))
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        let x = self.head.take().unwrap();
        self.head = x.next;
        Some(x.node)
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|f| &f.node)
    }
}

impl<T: Clone> SimpleLinkedList<T> {
    pub fn rev(&self) -> SimpleLinkedList<T> {
        let mut result = SimpleLinkedList::new();
        let mut current = self.head.as_ref();

        while let Some(node_ref) = current {
            result.push(node_ref.node.clone());
            current = node_ref.next.as_ref();
        }

        result
    }
}

impl<'a, T: Clone> From<&'a [T]> for SimpleLinkedList<T> {
    fn from(_item: &[T]) -> Self {
        let mut l: SimpleLinkedList<T> = SimpleLinkedList::new();
        for item in _item {
            l.push(item.clone())
        }

        l
    }
}

impl<T> Into<Vec<T>> for SimpleLinkedList<T> {
    fn into(mut self) -> Vec<T> {
        let mut result: Vec<T> = vec![];

        while let Some(data) = self.pop() {
            result.push(data);
        }

        result.reverse();
        result
    }
}
