# yoyok Grammar

## My little homemade recursive descent parser.

## Grammar

```bnf
<op>   ::= ['+' | '-' | '*' | '/']+
<type> ::= ['i' | 'u', 'f'] ['8' | '16' | '32' | '64']
         | 'bool'
         | 'char'
         | '(' [<type> ',']* ')'    // tuple
         | '['<type>' ';' <num>]'   // array
         | <type> -> <type>         // function
<expr> ::= <expr> [ <op> <expr> ]+
         | '(' <expr> ')'
         | <num>
         | <ident>
         | <ident> '=' <expr>
         | let <ident> [: <type>] '=' <expr>
         | var <ident> [: <type>] '=' <expr>
         | if <expr> '{' <seq> '}' else '{' <seq> '}'
<seq>  ::= | [<expr> ';']* <expr>
<prgm> ::= <seq>
```

TODO: To split up the grammar into multiple "groups"

## Examples

```rust
let x = 5 + 45;
let y = 45 + x - 20;
y
```

## Helpful Resources

- http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/grammar.html
