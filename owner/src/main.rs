use std::cell::RefCell;
use std::fmt::Display;
use std::rc::Rc;
use std::sync::Arc;
use std::sync::RwLock;
use std::{thread, time};

#[derive(Debug)]
struct Node {
    id: usize,
    // 使用 Rc<RefCell<Node>> 让节点可以被修改
    downstream: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    fn update_downstream(&mut self, downstream: Rc<RefCell<Node>>) {
        self.downstream = Some(downstream);
    }

    fn get_downstream(&self) -> Option<Rc<RefCell<Node>>> {
        self.downstream.as_ref().map(|v| v.clone())
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node<id={}>", self.id)
    }
}

fn main() {
    let mut node1 = Node::new(1);
    let mut node2 = Node::new(2);
    let mut node3 = Node::new(3);
    let node4 = Node::new(4);
    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap().clone());
    println!("node1: {:?}, node2: {:?}", node1, node2);

    let node5 = Node::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3.borrow_mut().downstream = Some(Rc::new(RefCell::new(node5)));
    println!("node1: {:?}, node2: {:?}", node1, node2);

    let s = Arc::new(RwLock::new("Hello".to_owned()));
    let ws = Arc::downgrade(&s);
    let arr = vec![1];
    let s1 = s.clone();
    let t = thread::spawn(move || {
        println!("{:?} from {:?}", arr, thread::current().id());
        thread::yield_now();
        println!("{} from {:?}", s1.read().unwrap(), thread::current().id());
        let ss = ws.upgrade().unwrap();
        let mut s2 = ss.write().unwrap();
        *s2 = "Hi".to_string();
    });
    thread::sleep(time::Duration::from_millis(10));
    println!("{} from {:?}", s.read().unwrap(), thread::current().id());

    t.join().unwrap();
}
