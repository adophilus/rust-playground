use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
  value: T,
  next: Link<T>
}

#[derive(Debug)]
struct List<T> {
  head: Link<T>
}

#[derive(Debug)]
struct Iter<'a, T> {
  node: &'a Link<T>
}

impl<T> Node<T> {
  fn new(value: T, next: Link<T>) -> Self {
    Self {
      value,
      next
    }
  }
}

impl<T> List<T> {
  fn new(value: Option<T>) -> Self {
    Self { head: value.map(|value| Rc::new(Node::new(value, None))) }
  }

  fn new_with_node(node: Link<T>) -> Self {
    Self { head: node }
  }

  fn head(&self) -> Option<&T> {
    self.head.as_ref().map(|node| &node.value)
  }

  fn tail(&self) -> Self {
    Self::new_with_node(self.head.as_ref().and_then(|node| node.next.clone()))
  }

  fn prepend(&self, value: T) -> Self {
    Self {
      head: Some(Rc::new(Node::new(value, self.head.clone())))
    }
  }

  fn iter<'a>(&'a self) -> Iter<'a, T> {
    Iter {
      node: &self.head
    }
  }
}

impl<T> Drop for List<T> {
  fn drop(&mut self) {
    let mut node = self.head.take();
    while let Some(_node) = node {
      if let Ok(mut __node) = Rc::try_unwrap(_node) {
        node = __node.next;
      }
      else {
        break;
      }
    }
  }
}

impl<'a, T> Iterator for Iter<'a, T> {
  type Item = &'a T;
  
  fn next(&mut self) -> Option<Self::Item> {
    self.node.as_ref().map(|node| {
      self.node = &node.next;
      &node.value
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn basic() {
    let list: List<usize> = List::new(None);

    let list = list.prepend(3).prepend(2).prepend(1);

    assert_eq!(list.head(), Some(&1));
    let list = list.tail();
    assert_eq!(list.head(), Some(&2));
    let list = list.tail();
    assert_eq!(list.head(), Some(&3));
  }

  #[test]
  fn iter() {
    let list: List<usize> = List::new(None).prepend(3).prepend(2).prepend(1);
    let mut iter = list.iter();
    assert_eq!(iter.next(), Some(&1));
    assert_eq!(iter.next(), Some(&2));
    assert_eq!(iter.next(), Some(&3));
  }
}