
/* 
fn main(){
    let string1=String::from("xyzh");
    {
        let string2=String::from("abc");
        let result=longest(string1.as_str(), string2.as_str());
        println!("{}", result);
    }
    
   

}
fn longest<'a>(x:&'a str, y:&'a str)->&'a str{
    
if x.len()>y.len(){
    x
}
else{
    y
}
}

*/
//the return value of the function cn only be used where the shortest lifetime between x and y is valid
//so the code above works

/* 
fn main(){
    let string1=String::from("xyzh");
    {
        let string2=String::from("abc");
        let result=longest(string1.as_str(), string2.as_str());
       
    }
    
    println!("{}", result);// the shortest lifetime is string2 and it is not valid here
    //so this wont't work

}
fn longest<'a>(x:&'a str, y:&'a str)->&'a str{
    
if x.len()>y.len(){
    x
}
else{
    y
}
}
*/

/* 
fn main(){
    let string1=String::from("xyzh");
    {
        let string2=String::from("abc");
        let result=longest(string1.as_str(), string2.as_str());
        println!("{}", result);
    }
    
   

}
fn longest<'a, 'b>(x:&'a str, y:&'b str)->&'a str{
    
x
}
*/

//Lifetime Elision rules
/*
This are some rules that the compiler follows to try guess the lifetime of references if you have not specified
The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. 
In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32);
a function with two parameters gets two separate lifetime parameters:
fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.


The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters:
fn foo<'a>(x: &'a i32) -> &'a i32.

The third rule is that, if there are multiple input lifetime parameters, 
but one of them is &self or &mut self because this is a method, the lifetime of self is assigned 
to all output lifetime parameters. 
This third rule makes methods much nicer to read and write because fewer symbols are necessary.




*/
