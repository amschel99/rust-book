# Error Handling with `Result` and `Option` Types in Rust

## 1. For `Result` Type (`Result<T, E>`):

- If the result is `Ok(value)`, the `value` is returned.
- If the result is `Err(error)`, the function exits early, and the `Err(error)` is returned from the function.

In the context of file I/O operations, like reading from a file, `File::open` and other related functions return a `Result` with `io::Error` as the error type (`Result<File, io::Error>`). Therefore, when you use `?` with these functions, it will return the content of the `Ok` variant (`File`) on success and automatically propagate the `Err` variant (`io::Error`) on failure.

## 2. For `Option` Type (`Option<T>`):

- If the option is `Some(value)`, the `value` is returned.
- If the option is `None`, the function exits early, and `None` is returned from the function.

The `?` operator can also be used with functions that return `Option`. If the function returns `Some(value)`, `value` is returned, and if it returns `None`, the function exits early, and `None` is returned from the function.

In summary, when dealing with `Result`, it will return the value on success or propagate the error (of type `E`), and when dealing with `Option`, it will return the value on `Some` or propagate `None`. In the case of file I/O and the `?` operator, you would typically see `io::Error` for error cases when dealing with `Result`.
