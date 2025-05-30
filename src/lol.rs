#![allow(unused)]
use std::cell::RefCell;

fn main() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, RefCell::new("a"));
    map.insert(2, RefCell::new("b"));
    if let Some(x) = map.get(&1) {
        let mut x_ref = x.borrow_mut();
        *x_ref = "c";
        drop(x_ref);
        let mut x_ref = x.borrow_mut();
        *x_ref = "e";
        drop(x_ref);
        if let Some(y) = map.get(&2) {
            let mut y_ref = y.borrow_mut();
            *y_ref = "d";
        }
    }
    assert_eq!(*map[&1].borrow(), "e");
    assert_eq!(*map[&2].borrow(), "d");
}
