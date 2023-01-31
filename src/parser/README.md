# yoyok Grammar

## My little homemade recursive descent parser.

## Grammar

```bnf
<op>   ::= ['+' | '-' | '*' | '/', '=', '>']+
<type> ::= ['i' | 'u', 'f'] ['8' | '16' | '32' | '64']
         | 'bool'
         | 'char'
         | '(' [<type> ',']* ')'    // tuple
         | '['<type>' ';' <num>]'   // array
         | <type> -> <type>         // function
<value> ::= <num>
         | 'true' | 'false'
         | ' <char> '
         | " <string> "
<expr> ::= <expr> [ <op> <expr> ]*
         | '(' <expr> ')'
         | <value>
         | '[' [<expr> ',']* ']'
         | <ident>
         | let <ident> [: <type>] '=' <expr>
         | var <ident> [: <type>] '=' <expr>
         | if <expr> '{' <seq> '}' else '{' <seq> '}'
         | while <expr> '{' <seq> '}'
         | <expr> ( [<expr> ',']* )
<fun>  ::= fn <ident> '(' [<ident> ':' <type> ',']* ')' ['->' <type>]? '{' <seq> '}'
<seq>  ::= | [<expr> ';']* <expr>
<prgm> ::= [ <fun> ]+
```

TODO: To split up the grammar into multiple "groups"

## Examples

```rust
let x = 5 + 45;
let y = 45 + x - 20;
y
```

Most of the parsing testing is actually done using proptest, see ![proptest](../ast/proptest.rs).

- These tests ensure the parser can parse any valid AST that's generated and presented using ![prettyprint](../ast/prettyprint.rs).
- The proptest also checks that the parsed AST is consistent with the AST produced by prettyprint.
- This, at the very least, forces consistency between the parser and prettyprint.

To see a single sample, try:

```
> cargo run -- -v
```

The programs generated are almost definitely not valid, but they are valid ASTs, and the parser should be able to parse them.

## Helpful Resources

- http://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/grammar.html
