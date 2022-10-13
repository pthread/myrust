
pub trait Summary{
    fn summary(&self) -> String; 
}

struct Node {
    header: String,
    content: String,
}

impl Node {
    fn new(h:String, c:String) -> Self {
        Node { header:h, content:c }
    }
}

impl Summary for Node {
    fn summary(&self) -> String {
        format!("{} -> {}", self.header, self.content)
    }
}

// Traits as Parameters: use traits to define functions that accept many different types
// This parameter accepts any type that implements the specified trait.
// sugar for  pub fn notify<T: Summary>(item: &T), Trait Bound Syntax
pub fn notify(item: &impl Summary) -> String{
    item.summary()
}

// Specifying Multiple Trait Bounds with the + Syntax
// pub fn notify(item: &(impl Summary + Display))
// =
// pub fn notify<T: Summary + Display>(item: &T)

// Clearer Trait Bounds with where Clauses
// fn some_function<T, U>(t: &T, u: &U) -> i32
// where
//    T: Display + Clone,
//    U: Clone + Debug,

// Returning Types that Implement Traits
// fn returns_summary() -> impl Summary { Node {x, y} }
// Returning either a NewsArticle or a Tweet isn’t allowed due to restrictions around how the impl Trait syntax is implemented in the compiler.

//Using Trait Bounds to Conditionally Implement Methods
// impl<T> Pair<T> {fn fx() }
// impl<T: Display + PartialOrd> Pair<T> { fn fy() }
//Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. 
//impl<T: Display> ToString for T {
    // --snip--
//}


fn find_largest<T: std::cmp::PartialOrd> (list: &Vec<T>) -> &T {
    let mut largest = &list[0];
    let iter = list.iter();
    for val in iter {
        if val > largest {
            largest = val;
        }
    }
    largest
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_notify() {
        let node = Node::new(String::from("rust"), String::from("safe language"));
        let sum = notify(&node);
        assert_eq!(sum, "rust -> safe language");
    }
    #[test]
    fn test_trait_node_summary() {
        let node = Node::new(String::from("rust"), String::from("safe language"));
        let sum = node.summary();
        assert_eq!(sum, "rust -> safe language");
    }

    #[test]
    fn test_int_largest() {
        let list: Vec<i32> = vec![9, 8, 12, 56, 78, 89, 2];
        let result = find_largest(&list);
        assert_eq!(*result, 89);
    }

    #[test]
    fn test_char_largest() {
        let list: Vec<char> = vec!['y', 'm', 'a', 'q'];
        let result = find_largest(&list);
        assert_eq!(*result, 'y');
    }
}
