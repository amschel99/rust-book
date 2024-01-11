
/*
Lets say we have have newsletter struct and tweet struct and we want them to have a common method called summarize. So we can create a Trait and write the summarize signature
then the 2 structs will implement the trait.


*/
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
