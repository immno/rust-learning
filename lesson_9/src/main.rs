use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let mut node1 = Node2::new(1);
    let mut node2 = Node2::new(2);
    let mut node3 = Node2::new(3);
    let node4 = Node2::new(4);
    node3.update_downstream(Rc::new(RefCell::new(node4)));
    node1.update_downstream(Rc::new(RefCell::new(node3)));
    node2.update_downstream(node1.get_downstream().unwrap());
    println!("node1: {:?}, node2: {:?}", node1, node2);

    let node5 = Node2::new(5);
    let node3 = node1.get_downstream().unwrap();
    node3.borrow_mut().update_downstream(Rc::new(RefCell::new(node5)));

    println!("node1: {:?}, node2: {:?}", node1, node2);
}

#[derive(Debug)]
struct Node {
    id: usize,
    downstream: Option<Rc<Node>>,
}

impl Node {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<Node>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<Node>> {
        self.downstream.as_ref().cloned()
    }
}


#[derive(Debug)]
struct Node2 {
    id: usize,
    downstream: Option<Rc<RefCell<Node2>>>,
}

impl Node2 {
    pub fn new(id: usize) -> Self {
        Self {
            id,
            downstream: None,
        }
    }

    pub fn update_downstream(&mut self, downstream: Rc<RefCell<Node2>>) {
        self.downstream = Some(downstream);
    }

    pub fn get_downstream(&self) -> Option<Rc<RefCell<Node2>>> {
        self.downstream.as_ref().cloned()
    }

    // for code that is not used but we don't want compiler to emit a warning
    // we could use `#[allow(dead_code)]` macro.
    #[allow(dead_code)]
    pub fn chain_downstream(&mut self, downstream: Rc<RefCell<Node2>>) -> Result<(), String> {
        match self.downstream.as_ref() {
            None => Err("No downstream".into()),
            Some(v) => {
                v.borrow_mut().downstream = Some(downstream);
                Ok(())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::Rc;
    use std::sync::Arc;

    use super::*;

    #[test]
    fn example_1() {
        let a = Rc::new(1);
    }

    #[test]
    fn example_2() {
        let a = Rc::new(1);
        let b = a.clone();
        let c = a.clone();
    }

    #[test]
    fn example_3() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);
        node3.update_downstream(Rc::new(node4));
        node1.update_downstream(Rc::new(node3));
        node2.update_downstream(node1.get_downstream().unwrap());
        println!("node1: {:?}, node2: {:?}", node1, node2);
    }

    #[test]
    fn example_4() {
        let mut node1 = Node::new(1);
        let mut node2 = Node::new(2);
        let mut node3 = Node::new(3);
        let node4 = Node::new(4);
        node3.update_downstream(Rc::new(node4));
        node1.update_downstream(Rc::new(node3));
        node2.update_downstream(node1.get_downstream().unwrap());
        let node5 = Node::new(5);
        let node3 = node1.get_downstream().unwrap();
        // node3 cannot borrow as mutable
        // node3.update_downstream(Rc::new(node5));
        println!("node1: {:?}, node2: {:?}", node1, node2);
    }

    // 内部可变性
    #[test]
    fn example_5() {
        let data = RefCell::new(1);
        // 在同一个作用域下，我们不能同时有活跃的可变借用和不可变借用。
        {
            let mut v = data.borrow_mut();
            *v += 1;
        }
        println!("data: {:?}", data.borrow());
    }


    #[test]
    fn example_6() {
        let mut node1 = Node2::new(1);
        let mut node2 = Node2::new(2);
        let mut node3 = Node2::new(3);
        let node4 = Node2::new(4);
        node3.update_downstream(Rc::new(RefCell::new(node4)));
        node1.update_downstream(Rc::new(RefCell::new(node3)));
        node2.update_downstream(node1.get_downstream().unwrap());
        println!("node1: {:?}, node2: {:?}", node1, node2);

        let node5 = Node2::new(5);
        // 能够直接对node4进行修改
        let node3 = node2.get_downstream().unwrap();
        let node4 = node3.borrow_mut().get_downstream().unwrap();
        node4.borrow_mut().update_downstream(Rc::new(RefCell::new(node5)));

        println!("node1: {:?}, node2: {:?}", node1, node2)
    }

    #[test]
    fn homework_1() {
        let arr = vec![1];
        std::thread::spawn(move || { println!("{:?}", arr); });
    }

    #[test]
    fn homework_2() {
        let s = Arc::new("Hello Mno!");
        let s1 = s.clone();
        let handler = std::thread::spawn(move || { println!("{:?}", s); });
        println!("{:?}", s1);
        handler.join().unwrap();
    }
}