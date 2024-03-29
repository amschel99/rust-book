// Lets simulate how we would handle errors while opening a file

/*
use std::fs::File;
use std::io::ErrorKind;
fn main() {
    let file = File::open("hello.txt");
    let opened_file = match file {
        OK(file) => file,
        Err(err) => match err.kind() {
            //err is of type io::Error, a struct provided by the standard library which has a method called kind which returns an io::ErrorKind type, an enum provided by the
            //standard library
            ErrorKind::NotFound => match File::create("hello.txt") {
                OK(file) => file,
                Err(e) => panic!("The file could not be created, {:?}", e),
            },
            other_error => {
                panic!("Error opening the file coz of another error");
            }
        },
    };
}

//Below is  more coincise way of doing it

fn using_un_wrap() {
    let file = File::open("hello.txt").unwrap();
    //uwrap will return the file for us if it ws read succesfully or panic if there was an error
}
fn using_expect() {
    let file = File::open("hello.txt").expect("There was an error while trying to open the file");
    // works the same way as unwrap but here the value we pass to expect is the message that will be passed to panic
}

//PROPAGATING ERRORS TO THE CALLING CODE

fn read_username_from_file() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt");

    let read_file = match file_result {
        OK(file) => file,
        Err(err) => return Err(err),
    };

    let mut username = String::new();

    match read_file.read_to_string(&mut username) {
        OK(_) => OK(username),
        Err(e) => Err(e),
    }
}
// The code bove can be written in a more coincise way
fn read_username_from_file_easily() -> Result<String, io::Error> {
    let file_result = File::open("hello.txt")?;
    let mut username = String::new();
    file_result.read_to_string(&mut username)?;
    OK(username)
}

fn read_username_from_file_more_easily() -> Result<String, io::Error> {
    let mut username = String::new();
    let file_result = File::open("hello.txt")?.read_to_string(&mut username)?;
    OK(username)
}

// reading contents of files is fairly common so the standard library provides, fs::read_to_string
fn read_file_using_std() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
//The ? operator can only be used in functions whose return type is compatible with the value the ? is used on
// The below function wll fail because return type is not Result< String, io::Error>
use std::fs::File;

fn main() {
    let greeting_file = File::open("hello.txt")?;
}
//? can be used to return Option<T>
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last()
}
//Nb main function always returns () but can also return Result<T, E>
fn our_main() -> Result<(), io::Error> {
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
*/
