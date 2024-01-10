// generics in Rust are very similar to generics in Typescript since we can also constraints them using Traits as we constraints generics in Typescript using interfaces
//Below is a generic function example
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}
//since not all types can be used with >, we had to add the constraints so that T are only types that implement the trait
fn main() {
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
//Generics with structs
struct Point <T>{
  x:T,
  y:T,
}
let integer_point= Point {
    x:0,
    y:1
}
let float_point=Point {
    x:0.1,
    y:1.3

}

    struct Point2<T, U>{
    x:T,
    y:U
    }

//Generics with enums
enum Option<T> {
    Some(T),
    None,
}
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// Generics in implementation blocks
#[derive(Debug)]
struct Point <T>{
  x:T,
  y:T
}
//the T after impl says that the T after Point is a generic not a core type. If we left it out 
//we would get an error saying T is not defined in the scope unless we replaced with a core type
impl<T> Point<T>{
fn x(&self)->&T{
  &self.x


}
}
fn main (){
  let  my_point=Point{
    x:1,
    y:2
  };
  print!("{:?}", my_point.x())
}


