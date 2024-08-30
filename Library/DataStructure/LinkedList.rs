pub mod dll {
    use std::{rc::Rc, cell::RefCell};

    #[derive(Clone, Debug)]
    pub struct DoublyLinkedListNode<T> {
        data: T,
        prev: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
        next: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>
    }

    #[derive(Clone, Debug)]
    pub struct DoublyLinkedList<T> {
        current: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
        HEAD: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>,
        TAIL: Option<Rc<RefCell<DoublyLinkedListNode<T>>>>
    }

    impl<T: Clone + Copy> DoublyLinkedList<T> {
        pub fn new() -> Self {
            DoublyLinkedList { current: None, HEAD: None, TAIL: None }
        }
        pub fn from(a: &[T]) -> Self {
            let refs: Vec<_> = a.iter().map(
                |&a| Rc::new(RefCell::new(DoublyLinkedListNode { data: a, prev: None, next: None }))
            ).collect();
            if a.len() > 1 {
                refs[0].borrow_mut().next = Some(Rc::clone(&refs[1]));
                for i in 1..a.len() {
                    refs[i - 1].borrow_mut().next = Some(Rc::clone(&refs[i]));
                    refs[i].borrow_mut().prev = Some(Rc::clone(&refs[i - 1]));
                }
            }
            DoublyLinkedList {
                current: Some(Rc::clone(&refs[0])),
                HEAD: Some(Rc::clone(&refs[0])),
                TAIL: Some(Rc::clone(&refs[refs.len() - 1]))
            }
        }
        pub fn add_prev(&mut self, x: T) {
            let new_node = Rc::new(RefCell::new(DoublyLinkedListNode { data: x, prev: None, next: None }));
            if let Some(n) = &self.current {
                n.borrow_mut().prev = Some(Rc::clone(&new_node));
                if let Some(prv) = &n.borrow().prev {
                    prv.borrow_mut().prev = Some(Rc::clone(&new_node));
                }
            } else {
                self.current = Some(Rc::clone(&new_node));
            }
        }
        pub fn add_next(&mut self, x: T) {
            let new_node = Rc::new(RefCell::new(DoublyLinkedListNode { data: x, prev: None, next: None }));
            if let Some(n) = &self.current {
                n.borrow_mut().prev = Some(Rc::clone(&new_node));
                if let Some(nxt) = &n.borrow().next {
                    nxt.borrow_mut().prev = Some(Rc::clone(&new_node));
                }
            } else {
                self.current = Some(Rc::clone(&new_node));
            }
        }
        pub fn remove(&mut self) {
            let mut next_node = None;
            if let Some(n) = &self.current {
                if let Some(prv) = &n.borrow().prev {
                    if let Some(nxt) = &n.borrow().next {
                        prv.borrow_mut().next = Some(Rc::clone(nxt));
                        nxt.borrow_mut().prev = Some(Rc::clone(prv));
                    } else {
                        prv.borrow_mut().next = None;
                        self.TAIL = Some(Rc::clone(&prv));
                    }
                    next_node = Some(Rc::clone(prv));
                } else if let Some(nxt) = &n.borrow().next {
                    nxt.borrow_mut().prev = None;
                    self.HEAD = Some(Rc::clone(nxt));
                    next_node = Some(Rc::clone(nxt));
                }
            }
            self.current = next_node;
        }
        pub fn get(&self) -> Option<T> {
            if let Some(n) = &self.current {
                Some(n.borrow().data)
            } else {
                None
            }
        }
        pub fn get_prev(&mut self) -> Option<T> {
            if let Some(n) = &self.current {
                if let Some(m) = &n.borrow().prev {
                    return Some(m.borrow().data);
                }
            }
            None
        }
        pub fn get_next(&mut self) -> Option<T> {
            if let Some(n) = &self.current {
                if let Some(m) = &n.borrow().next {
                    return Some(m.borrow().data);
                }
            }
            None
        }
        pub fn go_prev(&mut self) {
            let mut prv = None;
            if let Some(n) = &self.current {
                if let Some(m) = &n.borrow().prev {
                    prv = Some(Rc::clone(m));
                }
            }
            match prv {
                Some(_) => { self.current = prv; },
                _ => {}
            }
        }
        pub fn go_next(&mut self) {
            let mut nxt = None;
            if let Some(n) = &self.current {
                if let Some(m) = &n.borrow().next {
                    nxt = Some(Rc::clone(m));
                }
            }
            match nxt {
                Some(_) => { self.current = nxt; },
                _ => {}
            }
        }
        pub fn go_head(&mut self) {
            if let Some(n) = &self.HEAD {
                self.current = Some(Rc::clone(n));
            } else {
                self.current = None;
            }
        }
        pub fn go_tail(&mut self) {
            if let Some(n) = &self.TAIL {
                self.current = Some(Rc::clone(n));
            } else {
                self.current = None;
            }
        }
    }
}
