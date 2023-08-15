use std::{cell::{Ref, RefCell}, rc::Rc};

fn main() {
    let var01 = Rc::new(RefCell::new(1));
    let var03 = Rc::new(RefCell::new(2));
    let mut var02 = (var01.borrow(),);

    {
        var02.0 = var03.borrow();
    }

    let a = var02.0.clone();


    println!("{a:?}");
}