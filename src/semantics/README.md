# Pending Work

## Affinity Typing
 - Borrowing and ownership rules in linearity checker
 - Adding in coroutine in the language and the type checker

## Type Checking References
 - Any reference to a variable will return a Reference type. This is a pointer to the actual value
 - On the rhs of an operation, we can implicitly dereference the reference
 - On the lhs of some operations, we need the reference itself, like assignment and indexing

## Mutability Semantics
 - Seems like mutability is a property of the reference.
 - We need to correctly transfer mutability when we pass references around
 - This includes introducing a new reference type that is mutable (and changing the current reference while at it to be a new token)