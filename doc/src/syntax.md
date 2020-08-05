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

### Vectors
```leafbuild
{v0, v1, v2, ...}
```

Where `v0`, `v1`, `v2`, ... can be of any type.

#### Getting a value out of a vector

Same as in C/C++:

```leafbuild
{1, 2, 3}[0] = 1
{1, 2, 3}[1] = 2
{1, 2, 3}[2] = 3
```

### Maps
```leafbuild
{k0: v0, k1: v1, k2: v2, ...}
```

Where `v0`, `v1`, `v2`, ... can be of any type, and `k0`, `k1`, `k2` should be names.
Example:

```leafbuild
{
    a: 1,
    b: 2+3,
    c: '1',
    d: '''
        A
        B
        C
       '''
}
```

#### Getting a value out of a map
Same as with vectors, but pass a string with the key instead of the index.
```leafbuild
{a: 1, b: 2+3, c: '1'}['a'] = 1
{a: 1, b: 2+3, c: '1'}['b'] = 5
{a: 1, b: 2+3, c: '1'}['c'] = '1'
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
Like in C/C:
```leafbuild
let a = 0 // declare it
a = 1 // assign 1 to it
a += 1 // and add one to it; same as a = a + 1
a -= 1 // subtract one; same as a = a - 1
a *= 12 // multiply by 12; same as a = a * 12
a /= 2 // divide by 2; same as a = a / 2
a %= 3 // take modulo 3 and assign it back; same as a = a % 3
``` 
values cannot change their type, unless the type changes into
[the error type](reference/special_types/error.md), in which case it can be assigned back
to the type of the original value.

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

# If(conditionals)

Work the same as in C/C++, the only difference being you don't need the parentheses after an `if`.
```leafbuild
if condition {
statements
} else if condition2 {
statements2
} else if condition3 {
statements3
} /*...*/ {
/*...*/
} else {
// in case all conditions above failed
}
```