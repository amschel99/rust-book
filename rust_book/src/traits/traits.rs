/*
Lets say we have have newsletter struct and tweet struct and we want them to have a common method called summarize. So we can create a Trait and write the summarize signature
then the 2 structs will implement the trait.


*/
/*
pub trait Summary {
  fn summarize(&self)->String;
}
pub struct NewsLetter{
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}
impl Summary for NewsLetter{
  fn summarize(&self)->String{
    format!("{}, by {} ({})", self.headline, self.author, self.location)

  }
}
pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

fn main (){
  let tweet=Tweet{
    username:String::from("Amschel"),
    content:String::from("I write Rust btw"),
    reply:false,
    retweet:false
  };
  println!("{}", tweet.summarize())

}
//Default implementation
//so you can have a default implementation eg
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}

//then we can just do as below
impl Summary for NewsArticle {}


//Traits as parameters
fn notify(item:&impl Summary){
  println!("{}",item.summarize());

}
/*
This parameter accepts any type that implements the specified trait.
In the body of notify, we can call any methods on item that come from the Summary trait, such as summarize.
We can call notify and pass in any instance of NewsArticle or Tweet.
Code that calls the function with any other type, such as a String or an i32, won’t compile because those types don’t implement Summary.
*/
//THE FUNCTION ABOVE CAN BE WRITTEN AS BELOW WHICH I FIND TO BE MORE CLEAR
fn notify <T : Summary>(item:&T){
  println!("{}", item.summarize());

}
// SPECIFYING MULTIPLE TRAIT BOUNDS with + syntax
fn notify<T : Summary + Display>(item:&T){
  // do sth

}
//ABOVE IS MY PREFERRED WAY but we can still do it as below
fn notify(item:&(impl Summary + Display)){
  //do sth
}
//Clearer trait bounds with where clause
fn notify<T: Summary +Display, U:Clone +Debug>(item1:&T, item2:&U)->String{
  // do sth
  String::from("hello world")

}
// instead of doing sth like the above, we can write it as below
fn notify<T,U>(item1:&T, item2:&U)->String
  where
  T:Summary +Display,
  U:Clone+Debug,
  {
  String::from("hello world")

}
//Returning types that implement traits
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
//
//By using impl Summary for the return type,
//we specify that the returns_summarizable function returns some type that implements the Summary trait without naming the concrete type.
//In this case, returns_summarizable returns a Tweet, but the code calling this function doesn’t need to know that.
//However, you can only use impl Trait if you’re returning a single type.
//  For example, this code that returns either a NewsArticle or a Tweet with the return type specified as impl Summary wouldn’t work:

//This code does not compile!
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        Tweet {
            username: String::from("horse_ebooks"),
            content: String::from(
                "of course, as you probably already know, people",
            ),
            reply: false,
            retweet: false,
        }
    }
}

// We can use trait bounds to conditionally implement methods
use std::fmt::Display;

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

fn main() {
  enum Ip{
      V4(String),
      V6(String)
  }

    let pair = Pair::new(1, 2);
    pair.cmp_display();

    let pair2: Pair<Ip> = Pair::new(Ip::V4(String::from("192...")), Ip::V6(String::from("10.20..")));
   // pair2.cmp_display();// this won't work becuse enums do not implement Disply and partial order
  }


*/
