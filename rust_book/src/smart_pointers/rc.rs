use std::rc::Rc;

struct Node{
    value:i32,
    points_to:Option<Rc<Node>>
}



fn main (){
    let a= Rc::new(Node{
        value:1,
        points_to:None
    });
    let b=Node{
        value:2,
        points_to:Some(Rc::clone(&a))
    };
    let c=Node{
        value:3,
        points_to:Some(Rc::clone(&a))
    };


}