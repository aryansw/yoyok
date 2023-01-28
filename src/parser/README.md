# yoyok Grammar

## My little homemade recursive descent parser.

## Grammar

```bnf
<op>   ::= ['+' | '-' | '*' | '/']+
<expr> ::= <expr> [ <op> <expr> ]+
         | '(' <expr> ')'
         | <num>
         | <ident>
         | <ident> '=' <expr>
         | let <ident> '=' <expr>
         | var <ident> '=' <expr>
         | if <expr> '{' <expr> '}' else '{' <expr> '}'
<seq>  ::= | [<expr> ';']* <expr>
<prgm> ::= <seq>
```

TODO: Rewrite this to handle scopes and Units

## Examples

```rust
let x = 5 + 45;
let y = 45 + x - 20;
y
```

## Helpful Resources

- http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/grammar.html
