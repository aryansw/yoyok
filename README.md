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
