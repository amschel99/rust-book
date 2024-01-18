# Rust Traits Explained

## Summary Trait

### Defining the Trait
The `Summary` trait is created with a single method called `summarize` that returns a `String`.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### Implementing for NewsLetter
The `NewsLetter` struct implements the `Summary` trait by providing a custom `summarize` method.

```rust
impl Summary for NewsLetter {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

### Implementing for Tweet
Similarly, the `Tweet` struct implements the `Summary` trait with its own implementation of `summarize`.

```rust
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

## Default Implementation

### Providing a Default Implementation
You can provide a default implementation for the `Summary` trait.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### Using Default Implementation
Structs like `NewsArticle` can utilize the default implementation.

```rust
impl Summary for NewsArticle {}
```

## Traits as Parameters

### Function with Trait as Parameter
The `notify` function accepts any type that implements the `Summary` trait.

```rust
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

## Specifying Multiple Trait Bounds

### Using + Syntax
Trait bounds with multiple traits can be specified using the `+` syntax.

```rust
fn notify<T: Summary + Display>(item: &T) {
    // do something
}
```

## Clearer Trait Bounds with Where Clause

### Using Where Clause
A clearer syntax for specifying trait bounds using the `where` clause.

```rust
fn notify<T, U>(item1: &T, item2: &U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    // do something
    String::from("hello world")
}
```

## Returning Types that Implement Traits

### Using `impl Trait`
The `returns_summarizable` function returns a type that implements the `Summary` trait without specifying the concrete type.

```rust
fn returns_summarizable() -> impl Summary {
    // ...
}
```

### Limitations of `impl Trait`
Note that `impl Trait` can only be used for returning a single type.

## Conditional Implementation of Methods

### Conditional Method Implementation
Methods can be conditionally implemented using trait bounds.

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        // ...
    }
}
```

### Example Usage
An example usage of the `Pair` struct with an `Ip` enum.

```rust
let pair2: Pair<Ip> = Pair::new(Ip::V4(String::from("192...")), Ip::V6(String::from("10.20..")));
pair2.cmp_display();  // This won't work for Ip because it does not implement Display and PartialOrd
```

Feel free to use and modify this documentation to better suit your project. Happy coding!# Rust Traits Explained

## Summary Trait

### Defining the Trait
The `Summary` trait is created with a single method called `summarize` that returns a `String`.

```rust
pub trait Summary {
    fn summarize(&self) -> String;
}
```

### Implementing for NewsLetter
The `NewsLetter` struct implements the `Summary` trait by providing a custom `summarize` method.

```rust
impl Summary for NewsLetter {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}
```

### Implementing for Tweet
Similarly, the `Tweet` struct implements the `Summary` trait with its own implementation of `summarize`.

```rust
impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

## Default Implementation

### Providing a Default Implementation
You can provide a default implementation for the `Summary` trait.

```rust
pub trait Summary {
    fn summarize(&self) -> String {
        String::from("(Read more...)")
    }
}
```

### Using Default Implementation
Structs like `NewsArticle` can utilize the default implementation.

```rust
impl Summary for NewsArticle {}
```

## Traits as Parameters

### Function with Trait as Parameter
The `notify` function accepts any type that implements the `Summary` trait.

```rust
fn notify<T: Summary>(item: &T) {
    println!("{}", item.summarize());
}
```

## Specifying Multiple Trait Bounds

### Using + Syntax
Trait bounds with multiple traits can be specified using the `+` syntax.

```rust
fn notify<T: Summary + Display>(item: &T) {
    // do something
}
```

## Clearer Trait Bounds with Where Clause

### Using Where Clause
A clearer syntax for specifying trait bounds using the `where` clause.

```rust
fn notify<T, U>(item1: &T, item2: &U) -> String
where
    T: Summary + Display,
    U: Clone + Debug,
{
    // do something
    String::from("hello world")
}
```

## Returning Types that Implement Traits

### Using `impl Trait`
The `returns_summarizable` function returns a type that implements the `Summary` trait without specifying the concrete type.

```rust
fn returns_summarizable() -> impl Summary {
    // ...
}
```

### Limitations of `impl Trait`
Note that `impl Trait` can only be used for returning a single type.

## Conditional Implementation of Methods

### Conditional Method Implementation
Methods can be conditionally implemented using trait bounds.

```rust
struct Pair<T> {
    x: T,
    y: T,
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        // ...
    }
}
```

### Example Usage
An example usage of the `Pair` struct with an `Ip` enum.

```rust
let pair2: Pair<Ip> = Pair::new(Ip::V4(String::from("192...")), Ip::V6(String::from("10.20..")));
pair2.cmp_display();  // This won't work for Ip because it does not implement Display and PartialOrd
```

Feel free to use and modify this documentation to better suit your project. Happy coding!
