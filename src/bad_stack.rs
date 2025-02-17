use std::mem;

#[derive(Debug)]
pub struct List {
    head: Link
}

#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>)
}

#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link
}


pub fn hello() {
    println!("hello");
}


impl List {

    pub fn new() -> Self {
        List { head: Link::Empty }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }

}




impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);}
    }
}




#[cfg(test)]
mod test {   
    use super::List;
    #[test]
    fn testing() {
        let mut l = List::new();
        assert_eq!(l.pop(), None);
        l.push(123);
        assert_eq!(l.pop(), Some(123));
        assert_eq!(l.pop(), None);
        assert_ne!(l.pop(), Some(0));
    }
}