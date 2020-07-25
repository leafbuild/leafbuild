# Syntax

The full syntax reference.

## Comments
Same as in C/C++, a single line comment starts with `//` and goes
on to the end of the line, and a block comment starts with `/*`
and ends with `*/`

Examples:

```c
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
```c
0, 1, 2,
0777, // octal number
0x12349abcdef, // hex number
```

### String values
A simple string begins and ends with `'`, and may not contain newlines.
You can also use multiline strings; those begin and end with `'''`.

Examples:
```
'A simple single line string',

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
You can declare a variable like:

```
a = 0
b = 1
c = 'a string'
d = '''A
multiline
string'''
e = a // e = 0
```

### Assigning a value to a variable
It's the same as declaring one; values can change their type.

## Accessing properties

You can access properties of values like so:

```
value.property_name
```

## Calling functions

You can call functions like this:
```
function_name(positional_args, kwargs)
```

The `positional_args` is a list of comma-separated args, and `kwargs`
is a list of comma-separated key-value arguments, the key and value
being separated by `:`.

Examples:

```c
f(a, b, c: d, e: f)
// note that you can also have only positional args or kwargs, without needing the extra comma between them
g(0, a)
h(a: b, c: d)
```

You can also have a trailing comma and split function calls over multiple lines:
```c
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

```
base_value.method_name(positional_args, kwargs)
```

Note that `method_name(positional_args, kwargs)` works the same way functions do,
so all the rules described in [calling functions](#calling-functions) apply here
as well.