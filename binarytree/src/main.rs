// High Performance Binary Tree
// Support type: &str, i32, f32

struct Node<'a> {
    val: &'a str,
    left: Option<Box<Node<'a>>>,
    right: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {
    fn new(v: &'a str) -> Self {
        Self {
            val: v, 
            left: None,
            right: None,
        }
    }

    pub fn insert(&mut self, new_val: &'a str) {
        println!("insert: {}", new_val);
        if self.val == new_val {
            return
        }   
        let target_node = if new_val < self.val { &mut self.left } else { &mut self.right };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Some(Box::new(Node::new(new_val)));
                *target_node = new_node;
            }   
        }   
    }   

    // Why is the first param &?
    // ^^^^^^^^^ move occurs because `self.left` has type `Option<Box<Node<'_>>>`, which does not implement the `Copy` trait
    fn dfs (root: &Option<Box<Node<'a>>>, list: &mut Vec<String>) {
        if let Some(x) = root {
            Self::dfs(&x.left, list);
            list.push(x.val.into());
            Self::dfs(&x.right, list);
        }
    }

    pub fn dump(&self) {
        println!("====== Dump Binary Tree ======");
        println!("root node: {}", self.val);
        let mut list: Vec<String> = vec![];
        Self::dfs (&self.left, &mut list);
        Self::dfs (&self.right, &mut list);
        println!("{:?}", list);
        println!("==============================");
    }

    pub fn search(&self, val: &'a str) -> Option<&'a str> {
        Some(val)
    }   
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_btree() -> Result<(), String> {
        let result = 200;
        if result == 100 {
            Ok(())
        } else {
            Err(String::from("should be 100"))
        }
        //assert_eq!(result, 100, "should be 100");
    }
}


fn main() {
    println!("Hello, BinaryTree!");
    let mut btree: Node = Node::new("hello");
    btree.insert ("world");
    btree.insert ("rust");
    btree.insert ("book");
    btree.dump();

    let result = btree.search("hello");
    println!("{:?}", result);
    let result2 = btree.search("rust");
    println!("{:?}", result2);
}

