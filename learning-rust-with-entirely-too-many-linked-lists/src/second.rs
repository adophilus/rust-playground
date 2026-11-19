#[derive(Debug)]
struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Link<T>,
}

impl<T> List<T> {
    fn new() -> Self {
        Self { head: None }
    }

    fn push(&mut self, value: T) {
        // let new_node = Box::new(Node::new(value, self.head.take()));
        // self.head = Some(new_node);

        let old_head = self.head.take();
        let new_node = Node::new(value, old_head);
        self.head = Some(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.value
        })
    }

    fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.value)
    }
}

impl<T> std::ops::Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

impl<T> Node<T> {
    pub fn new(value: T, next: Link<T>) -> Self {
        Self { value, next }
    }
}

pub fn main() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    dbg!(&list);

    list.pop();
    dbg!(&list);
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basic() {
        let mut list1 = List::new();

        // check empty list behaves right
        assert_eq!(list1.pop(), None);

        // pupulate list
        list1.push(1);
        list1.push(2);
        list1.push(3);

        // check normal removal
        assert_eq!(list1.pop(), Some(3));
        assert_eq!(list1.pop(), Some(2));

        // push some more jsut to make sure nothing's corrupted
        list1.push(4);
        list1.push(5);

        // check normal removal
        assert_eq!(list1.pop(), Some(5));
        assert_eq!(list1.pop(), Some(4));

        // check exhaustion
        assert_eq!(list1.pop(), Some(1));
        assert_eq!(list1.pop(), None);
    }

    #[test]
    fn peek() {
        let mut list = List::new();

        assert_eq!(list.peek(), None);
        assert_eq!(list.peek_mut(), None);
        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.peek_mut(), Some(&mut 3));
    }
}
