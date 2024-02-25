use std::collections::{HashMap, HashSet};


type Graph<T> = HashMap<T,HashSet<T>>;

pub struct State<T>{
    depends_on: Graph<T>,
    dependents: Graph<T>,
    no_deps: Vec<T>,
}

impl<T> State<T>
where 
    T: Eq + std::hash::Hash {
        pub fn get_dependents(){}

        pub fn is_resolved(self: &Self) -> bool {
            self.depends_on.is_empty()
        }

}


fn main() {
    println!("Hello, world!");
}
