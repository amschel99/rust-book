// Lets simulate how we would handle errors while opening a file
use std::fs::File;
use std::io::ErrorKind;
fn main (){
    let file=File::open("hello.txt");
    let opened_file= match file{
        OK(file)=>file,
        Err(err)=>match err.kind(){
//err is of type io::Error, a struct provided by the standard library which has a method called kind which returns an io::ErrorKind type, an enum provided by the 
  //standard library
             ErrorKind::NotFound=>match File::create("hello.txt"){
                 OK(file)=>file,
                Err(e)=>panic!("The file could not be created, {:?}",e),
    },
      other_error=>{
        panic!("Error opening the file coz of another error");
      }

  
    }
  };
}

          //Below is  more coincise way of doing it

fn using_un_wrap(){
    let file=File::open("hello.txt").unwrap();
    //uwrap will return the file for us if it ws read succesfully or panic if there was an error
    
}
fn using_expect(){
    let file=File::open("hello.txt").expect("There was an error while trying to open the file");
    // works the same way as unwrap but here the vlue we pass to expect is the message that will be passed to panic
}

  
