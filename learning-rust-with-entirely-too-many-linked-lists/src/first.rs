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

        match &link {
            Link::Empty => return None,
            Link::More(node) => match node.next {
                Link::Empty => {
                    match link {
                        Link::Empty => unreachable!(),
                        Link::More(_node) => {
                            node.next = _node.next;
                        }
                    };
                    return Some(node.value);
                }
                _ => (),
            },
        };

        loop {
            match link {
                Link::Empty => unreachable!(),
                Link::More(node) => match node.next {
                    Link::Empty => {
                        match link {
                            Link::Empty => unreachable!(),
                            Link::More(_node) => {
                                node.next = _node.next;
                            }
                        };
                        return Some(node.value);
                    }
                    Link::More(_node) => {
                        link = &mut node.next;
                    }
                },
            }
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
