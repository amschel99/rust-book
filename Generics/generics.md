# Generics in Rust

Generics in Rust share similarities with generics in TypeScript, allowing constraints using traits, much like TypeScript interfaces. Below are examples of generic functions, structs, enums, and implementation blocks.

## Generic Functions

```rust
// Generic function example
fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

Generics are constrained with `T: std::cmp::PartialOrd` to ensure types can be compared.

## Generic Structs

```rust
// Generic struct
struct Point<T> {
    x: T,
    y: T,
}

// Instances with different types
let integer_point = Point { x: 0, y: 1 };
let float_point = Point { x: 0.1, y: 1.3 };
```

A generic `Point` struct allows instances with various types.

```rust
struct Point2<T, U> {
    x: T,
    y: U,
}
```

A more flexible `Point2` struct allows different types for `x` and `y`.

## Generic Enums

```rust
// Generic enum Option
enum Option<T> {
    Some(T),
    None,
}

// Generic enum Result
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

Enums like `Option` and `Result` can hold values of generic types.

## Generics in Implementation Blocks

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

fn main() {
    let my_point = Point { x: 1, y: 2 };
    print!("{:?}", my_point.x())
}
```

Implementation blocks allow defining methods on generic structs. In this example, a method `x` is implemented for `Point<T>`. The `impl<T>` syntax denotes a generic implementation.

