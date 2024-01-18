
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