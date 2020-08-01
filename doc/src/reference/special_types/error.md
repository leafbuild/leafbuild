# The `error` type

The `error` type is a special type: you get
one when you screw something up.

## Where it appears
The error type appears only in the following situations:
- Incompatible operands for an operation
  (see [the page on operations](../operations.md)
  to find out which are compatible and which ones aren't)

Take the following example:

```leafbuild
print((f() + 1) * (2 + 3) / (4 * 5))
```

Now if the result of `f()` is of the `void` type,
the interpreter will issue the following (soft) errors:
```
 error: Incompatible operands
   ┌─ build.leaf
   │
   │ print((f() + 1) * (2 + 3) / (4 + 5))
   │        ^^^   - i32
   │        │      
   │        void
 
 error: Incompatible operands
   ┌─ build.leaf
   │
   │ print((f() + 1) * (2 + 3) / (4 + 5))
   │       ^^^^^^^^^   ------- i32
   │       │            
   │       error
 
 error: Incompatible operands
   ┌─ build.leaf
   │
   │ print((f() + 1) * (2 + 3) / (4 + 5))
   │       ^^^^^^^^^^^^^^^^^^^   ------- i32
   │       │                      
   │       error
 
 -- (error)
```

And will continue, eventually actually printing `(error)`.

The `error` type is never allowed to appear in the arguments
of the [`project()`](../functions/project_and_module/project.md)
and [`module()`](../functions/project_and_module/module.md)
functions.