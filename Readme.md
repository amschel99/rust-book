#### Currently struggling with lifetimes 😢


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