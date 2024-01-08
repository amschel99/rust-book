// The code below will use a hashmap to check how many times a word appears in a sentence
use std::collections::HashMap;
 
  fn main (){


  let text="hello world wonderful world";
let mut map=HashMap::new();
  
for word in text.split_whitespace(){
  //or_insert returns a mutable reference to the value with the key passed in entry
  let count=map.entry(word).or_insert(0);
  //count is a reference and to edit its value, we gotta dereference
  *count+=1;
}
  dbg!(map);

  
}
