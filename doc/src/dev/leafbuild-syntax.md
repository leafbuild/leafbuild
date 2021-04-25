# `leafbuild-syntax`

## `syn_tree`

Defines CST structures in `syn_tree::implementation` and reexports
them in `syn_tree`.

## `syn_tree::visitor`
Also defines 2 visitors in `syn_tree::visitor`:
`Visitor` and `ThreadSafeVisitor`. 

The only difference between them being:
`ThreadSafeVisitor` takes `&self`
as receiver to methods and has 2 requirements: `Send + Sync`.

There is also an `impl<T: ThreadSafeVisitor> Visitor for T`, forwarding all calls to the `ThreadSafeVisitor` impl.

## `parser`
Provides an interface for parsing text into rowan `GreenNode`s.


