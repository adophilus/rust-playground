#[derive(Debug)]
struct List {
    head: Link,
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug)]
struct Node {
    value: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        Self { head: Link::Empty }
    }

    fn push(&mut self, value: i32) {
        // let new_node = Box::new(Node::new(
        //     value,
        //     std::mem::replace(&mut self.head, Link::Empty),
        // ));
        // self.head = Link::More(new_node);

        let old_head = std::mem::replace(&mut self.head, Link::Empty);
        let new_node = Node::new(value, old_head);
        self.head = Link::More(Box::new(new_node));
    }

    fn pop(&mut self) -> Option<i32> {
        // // why does this work?
        match std::mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }

        // // but this doesn't
        // let node = match &self.head {
        //     Link::Empty => return None,
        //     Link::More(node) => node,
        // };
        // self.head = node.next;
        // Some(node.value)
    }
}

impl std::ops::Drop for List {
    fn drop(&mut self) {
        let mut cur_link = std::mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = std::mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

impl Node {
    pub fn new(value: i32, next: Link) -> Self {
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
        let mut list = List::new();

        // check empty list behaves right
        assert_eq!(list.pop(), None);

        // pupulate list
        list.push(1);
        list.push(2);
        list.push(3);

        // check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // push some more jsut to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
