struct Node<'a> {
    val: &'a str,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    pub fn insert(&mut self, new_val: &'a str) {
        println!("insert: {}", new_val);
        if self.val == new_val {
            return
        }   
        let target_node = if new_val < self.val { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Some(Box::new(Node{val: new_val, left: None, right: None, }));
                *target_node = new_node;
            }   
        }   
    }   

    pub fn dump(&self) {
        println!("====== Dump Binary Tree ======");
        println!("node: {}, left:", self.val);
        println!("==============================");
    }   
}

fn main() {
    println!("Hello, BinaryTree!");
    let mut node: Node = Node{val: "hello", left: None, right: None, };
    node.insert ("world");
    node.insert ("rust");
    node.insert ("book");
    node.dump();
}

