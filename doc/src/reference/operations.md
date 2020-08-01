# Operations

1. All math operators are defined on same signedness numbers.
2. If they are of different sizes, the smaller one will be promoted
to the size of the bigger one.
3. Addition is defined when at least one operand is of type `string`,
the other one being converted to a string and the result being their
concatenation.
4. All other possible combinations of operators and operands will
yield an `error` object, type described [here](special_types/error.md).

Examples:
```leafbuild
i32 + u32 = error
i32 + u64 = error

i32 + i32 = i32
i32 / i32 = i32
i32 - i64 = i64
u64 * u32 = u64

'1' + 2 = '12'
2 + '3' = '23'
'12' - 2 = error
```
