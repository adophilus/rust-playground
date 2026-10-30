struct List {
    head: Link
}

enum Link {
    Empty,
    More(Box<Node>)
}

struct Node {
    value: i32,
    next: Link
}

impl List {
    fn new() -> Self {
        Self { head: Link::Empty }
    }
}

fn main() {
    let mut list = List::new();
    list.push(1);
}
