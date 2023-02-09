# yoyok Compiler

Unsurprisingly similar syntax to Rust.

```
fn main() -> i32 {
  let x = 5 + 1199;
  var y: i32 = if x < 0 {
      0
  } else {
      1
  };
  while y < x {
      y = y + 1;
  };
  x
}
```

## Documentation

- Parser: [src/parser/README.md](src/parser/README.md)
- AST: [src/ast/ast.rs](src/ast/ast.rs)

## Running

```
cargo run -- examples/comments.yk -v
```

## Testing

By default, the tests are run in release mode, so that the tests run faster.

```
cargo test
```

## Random Scribbles

Concurrent ML

- (Similar to Coroutines)
- Asynchronous CML (Reasoned about Coroutine like Behaviour)
  - Unbuffered accessing
  - https://www.cs.purdue.edu/homes/suresh/papers/pldi11.pdf
- Look at CML and ACML
- Linear Types are important for Ownership (Affine-typing)
  - Combinators: Linear Types for CML
  - Type systems for Concurrent Programs
