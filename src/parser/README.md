# yoyok Grammar

## My little homemade recursive descent parser.

## Grammar

```bnf
<op>   ::= ['+' | '-' | '*' | '/']
<expr> ::= <expr> [ <op> <expr> ]*
         | <num>
         | <ident>
<simp> ::= let <ident> '=' <expr>
          | var <ident> '=' <expr>
<prgm> ::= [<simp> ';']* <expr>
```

## Examples

```rust
let x = 5 + 45;
let y = 45 + x - 20;
y
```
