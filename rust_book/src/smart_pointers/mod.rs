use std::ops::Deref;
pub mod  rc;
struct MyBox<T>(T);

impl<T> MyBox<T>{
    fn new (value:T)->MyBox<T>{
MyBox(value)
    }
}
impl<T> Deref for MyBox<T>{
    type Target = T;
   fn deref(&self) -> &Self::Target {
       &self.0
   } 
}
fn hello(name:&str){
    println!("{}",name);
}


fn main (){
    let name=MyBox::new(String::from("Amschel"));
    hello(&name);
    let x=5;
    let y= MyBox(x);
    assert_eq!(x,5);

    //below is what happens behind the scenes
    // *(y.deref())
    assert_eq!(*y,5);

}
//DEREF COERCION
//Basically what deref coercion does is it converts a reference to a type that implements a deref trait to a reference to another type