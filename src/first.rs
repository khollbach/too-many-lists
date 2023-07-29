use std::mem;

pub struct List {
    head: Link,
}

enum Link {
    Empty,
    More(Box<Node>),
}

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        Self { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let old_head = mem::replace(&mut self.head, Link::Empty);

        let new_node = Box::new(Node {
            elem,
            next: old_head,
        });

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match self.pop_node() {
            Link::Empty => None,
            Link::More(node) => {
                debug_assert!(matches!(node.next, Link::Empty));
                Some(node.elem)
            }
        }
    }

    fn pop_node(&mut self) -> Link {
        let old_head = mem::replace(&mut self.head, Link::Empty);
        match old_head {
            Link::Empty => Link::Empty,
            Link::More(mut node) => {
                let new_head = mem::replace(&mut node.next, Link::Empty);
                self.head = new_head;
                Link::More(node)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        while let Link::More(_) = self.pop_node() {}
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}
