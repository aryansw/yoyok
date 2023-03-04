# Pending Work

## References for Arrays and Tuples
  - Refactor the values in [value.rs](value.rs) to be references to the actual locations in the environment, instead of the values themselves
  - Even references themselves return a "pointer" to the memory location of the reference
  - Binary operations are allowed to implicitly dereference the references

## Support Lifetimes for Variables
To do this, we have
  1. Every variable has a lifetime attached to it, using an integer, in the environment
  2. Typically, the lifetime of a variable is the same as the lifetime of it's scope
  3. Everytime run_exprs is called, we're entering a new scope. Increment the lifetime counter, and every variable created in this scope has this lifetime
  4. When run_exprs returns, we're leaving the scope. Decrement the lifetime counter, and remove all variables with this lifetime from the environment
  5. If any variables are allowed to live longer than their scope, we can use a reference counter to keep track of how many scopes they're still alive in
