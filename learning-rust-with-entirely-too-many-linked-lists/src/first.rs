#[derive(Debug)]
struct List {
    head: Link,
}

impl List {
    fn new() -> Self {
        Self { head: Link::Empty }
    }

    fn push(&mut self, elem: i32) {
        let mut link = &mut self.head;
        let new_link = Link::More(Box::new(Node::new(elem, Link::Empty)));
        loop {
            match link {
                Link::Empty => {
                    let _ = std::mem::replace(link, new_link);
                    break;
                }
                Link::More(node) => {
                    link = &mut node.next;
                }
            }
        }
    }

    fn pop(&mut self) -> Option<i32> {
        let mut link = &mut self.head;

        if let Link::Empty = link {
            return None;
        }

        let mut prev_link = link;

        while let Link::More(node) = link {
            if let Link::Empty = node.next {
                match prev_link {
                    Link::Empty => unreachable!(),
                    Link::More(node) => {
                        let _link = std::mem::replace(&mut node.next, Link::Empty);
                        match _link {
                            Link::Empty => unreachable!(),
                            Link::More(_node) => return Some(_node.value),
                        }
                    }
                };
            }
            prev_link = link;
            link = &mut node.next;
        }

        return None;
    }
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

impl Node {
    pub fn new(value: i32, next: Link) -> Self {
        Self { value, next }
    }
}

pub fn first() {
    let mut list = List::new();
    list.push(1);
    list.push(2);
    list.push(3);
    dbg!(&list);

    list.pop();
    dbg!(&list);
}
