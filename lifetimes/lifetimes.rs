
/*
The code below won't work because of the following reason,
 r's value is a reference to x but x went outta scope 
 In other words, r has a longer lifetime than x
 //The borrow checker compares lifetimes or scopes to determine where a borrow is valid
fn main() {
    let r;

    {
        let x = 5;
        r = &x;
    }

    println!("r: {}", r);
}
*/


