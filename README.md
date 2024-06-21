# Pike - PipeLike

Pike is a macro collection to pipe your functions calls, like in functional languages such as F#, Elixir and OCamel.

## Examples

The pipe operator |> allows you to establish "pipelines" of functions in a flexible manner.

### TL;DR

```rust
// takes a string's length, doubles it and converts it back into a string
let len = pike! {
  "abcd"
  |> str::len
  |> (as u32)
  |> (times(2))
  |> &
  |> u32::to_string
};

// same as

let len = times("abcd".len() as u32, 2).to_string();
```

```rust
fn times(a: u32, b: u32) -> u32{
    a * b
}

fn times2(n: u32) -> u32 {
    times(n, 2)
}

// Passes the preceding expression as the only argument of proceding function.
let num = pike! {
  2
  |> times2
  |> times2
};
assert_eq!(num, 2 * 2 * 2);

// Passes the preceding expression as the first argument of proceding function.
// by wrapping the function in parentheses we can pass the remanining arguments by partially
// calling the `times` as `times(?, 2)` and passing 2 as its first argument via the pipeline.
let num = pike! {
  1
  |> (times(2))
  |> (times(3))
};
assert_eq!(num, 1 * 2 * 3);

// call a method using pipelines
let len = pike!("abcd" |> str::len);
assert_eq!(len, "abcd".len());

// Closures can also be pipelined similar to partial functions.
let c = pike! {
  ['a', 'b', 'c', 'd']
  |> (|it: [char; 4]| it[2])
};
assert_eq!(c, 'c');

// Piping through `&` symbol would get a reference to the preceding expression.
let it = "it";
let is_it = |r: &&str| it == *r;

let is_it = pike! {
  it
  |> &
  |> is_it
};
assert_eq!(is_it, true);


// There are also special macros for options and results but those already have an ergonomic API for chaining.

let data = pike_opt!(id |> get_cached |> fetch_local |> fetch_remote);
// same as get_cached(id).or_else(|| fetch_local(id)).or_else(|| fetch_remote(id));

let result = pike_res!("http://rust-lang.org" |> download |> parse |> get_links);
// same as download("http://rust-lang.org").map(parse).map(get_links);
```


## Macros

- `pike!` is the "standard" pipe macro
- `pike_res!` works like `pike!` but takes only functions that return a `Result` (of the
  same type) and returns early if that result is an Err. Useful for combining multiple IO
  transformations like opening a file, reading the contents and making an HTTP request.
- `pike_opt!` works like `pike!` but takes only functions that return an `Option` (of the same type).
  The pipeline will continue to operate on the initial value as long as `None` is returned from all functions.
  If a function in the pipeline returns `Some`, the macro will exit early and return that value.
  This can be useful if you want to try out several functions to see which can make use of that value in a specified order.

## Syntax Features

Any `pike` starts with an expression as initial value and requires you
to specify a function to transform that initial value.
```rust
let result = pike!(2 |> times2);
// same as times2(2)
```

You can get more fancy with functions, too, if you add parentheses like
in a normal function call, the passed parameters will be applied to that
function after the transformed value.

```rust
let result = pike!(2 |> (times(2)));
// same as times(2, 2)
```

You can pass closures \o/! A closure must be wrapped in parentheses as well.
```rust
let result = pike! {
  2
  |> (times(2))
  |> (|i: u32| i * 2)
};
// same as (|i: u32| i * 2)(time(2, 2))
```

If you want a function to be called as a method on the transform value, just pass it as a path.

```rust
let result = pike!("abcd" |> str::len);
// same as "abcd".len()
```

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

Pike is spritual successor to [pipeline.rs](https://github.com/johannhof/pipeline.rs) and derives its license from the said project.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally
submitted for inclusion in the work by you, as defined in the Apache-2.0
license, shall be dual licensed as above, without any additional terms or
conditions.
