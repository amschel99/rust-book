use std::cell::RefCell;

fn main (){
    let x= RefCell::new(4);
    *x.borrow_mut()+=1;
}