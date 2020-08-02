# Syntax

The full syntax reference.

## Comments
Same as in C/C++, a single line comment starts with `//` and goes
on to the end of the line, and a block comment starts with `/*`
and ends with `*/`

Examples:

```leafbuild
// a single line comment

// another single line comment

/*
 A block comment
 that can
 go on for
 multiple
 lines
 */
```


## Values

### Integer values
As of now, only 32-bit signed ints are supported.
Int values work as they do in C/C++:
- prefixed with `0` means it is an octal number
- prefixed with `0x` means it is a hex number

Examples of values:
```leafbuild
0, 1, 2,
0777, // octal number
0x12349abcdef, // hex number
```

### Booleans
`true` and `false`, just like in C++, or C with the `<stdbool.h>` header.

```leafbuild
true, false
```

### String values
A simple string begins and ends with `'`, and may have escaped apostrophes;
should not contain newlines.
You can also use multiline strings; those begin and end with `'''`.

Examples:
```leafbuild
'A simple single line string',

'A simple string with \'escaped\' apostropes'

'''A
multiline
string
example
'''
```


## Variables

Variables, once assigned a value, will live for as long as the file
they are declared in is processed.

### Declaring
Variables are declared with the `let` keyword:

```leafbuild
let variable_name = value
```

Examples:

```leafbuild
let a = 0
let b = 1
let c = 'a string'
let d = '''A
multiline
string'''
let e = a // e = 0
```

### Assigning a value to a variable
It's the same as declaring one; values cannot change their type, unless the type changes into [the error type](reference/special_types/error.md).

## Accessing properties

You can access properties of values like so:

```leafbuild
value.property_name
```

## Calling functions

You can call functions like this:
```leafbuild
function_name(positional_args, kwargs)
```

The `positional_args` is a list of comma-separated args, and `kwargs`
is a list of comma-separated key-value arguments, the key and value
being separated by `:`.

Examples:

```leafbuild
f(a, b, c: d, e: f)
// note that you can also have only positional args or kwargs, without needing the extra comma between them
g(0, a)
h(a: b, c: d)

project()
```

You can also have a trailing comma and split function calls over multiple lines:
```leafbuild
f(
  0,
  1,
)
g(
  a: b,
  c: d,
)
```

> **Note**: newlines are allowed only after commas.

## Calling methods

You can call methods like this:

```leafbuild
base_value.method_name(positional_args, kwargs)
```

Note that `method_name(positional_args, kwargs)` works the same way functions do,
so all the rules described in [calling functions](#calling-functions) apply here
as well.