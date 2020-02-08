# Arithmetic operators

Arithmetic operators do not perform any kind of overflow checking in
compile time. If an overflow happens, the Zinc VM will fail in runtime.

Both operands of a binary operator must always be of the same type, e.g.:
- `u8` and `u8`
- `i64` and `i64`
- `field` and `field`

> The operators `/`, `%`, `>=`, `>`, `<=`, `<` are temporary forbidden for the
> type `field`, but will probably become available soon.

#### Addition

`+` is a binary operator.

**Accepts**
1. Integer expression
2. Integer expression of the same type

**Returns** an integer result of the same type.

#### Subtraction

`-` is a binary operator.

**Accepts**
1. Integer expression
2. Integer expression of the same type

**Returns** an integer result of the same type.

#### Multiplication

`*` is a binary operator.

**Accepts**
1. Integer expression
2. Integer expression of the same type

**Returns** an integer result of the same type.

#### Division

`/` is a binary operator.

**Accepts**
1. Integer expression (anything but `field`)
2. Integer expression of the same type

**Returns** an integer result of the same type.

#### Remainder

`%` is a binary operator.

**Accepts**
1. Integer expression (anything but `field`)
2. Integer expression of the same type

**Returns** an integer result of the same type.

#### Negation

`-` is an unary operator.

**Accepts**
1. Integer expression (anything but `field`)

**Returns** a signed integer with same bitlength.