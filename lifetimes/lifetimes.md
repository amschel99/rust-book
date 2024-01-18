lifetimes are named regions of code that a reference must be valid for
references are the ones that have lifetimes
```rust


fn main() {
    let r;

    {
        let x = 5;
        r = &x;                     <!--- lifetime of r starts here

    }

    println!("r: {}", r);            <!--- lifetime of r ends here
} 

```
The above code is not valid because r crosses the curly braces to where x is not valid

                                  
``` rust

fn main() {
    let r;

    {
        let x = 5;
        r = &x;                     <!--- lifetime of r starts here and ends here since

    }

} 
                                  
```
 The above code is valid , look at the lifetime of r

 ```rust 
// this code sample does *not* compile
fn f(s: &str, t: &str) -> &str {
    if s.len() > 5 { s } else { t }
}
```
In the above, We know that the returned reference must be one of the references we received as an input argument, but we don’t know which one.

```rust
fn f<'a>(s: &'a str, t: &'a str) -> &'a str {
    if s.len() > 5 { s } else { t }
}

```
The way to achieve this is to give both input parameters the same lifetime annotation. It’s how we tell the compiler that as long as both of these input parameters are valid, so is the returned value.

```rust
fn f<'a, 'b>(s: &'a str, _t: &'b str) -> &'a str {
    s
}

```
If you know what you are doing, you can do the above.

