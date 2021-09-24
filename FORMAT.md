# Syntax for Abstract Assembly Files in BNF Form

This syntax is written in BNF form, where any words surrounded by quotations refer to literals. Any other identifiers refer to other rules.

All whitespace except for newlines are ignored. We explicitly state where whitespace is allowed by including the `\n` literal.

BNF syntax uses Regex rules in general. Here are some of the common ones:
```
*       - Repeat 0 or more times
+       - Repeat 1 or more times
?       - Optional / Repeat 0 or 1 times
( ... ) - Group of elements
[a-b]   - Range of potential literals ranging from a to b  
```

*Note that comments are lines that start with "//"*


## Syntax

#### Overall File Structure
```
file = (fn "\n"*) *
```

#### Function
```
fn = id temp* "\n" instr* branch
   | id temp* "\n" block*
```

#### Basic Block
```
block = block_id "\n" instr* branch              # No Predecessors
      | block_id+ "\n" instr* branch    # With Predecessors
```

#### Instructions
```
instr = temp "=" oper               "\n"
      | temp "=" unop oper          "\n"
      | temp "=" oper binop oper    "\n"
      | temp "=" "phi" temp*        "\n"
      | temp "=" "call" id temp*    "\n"
      | "call" id temp*             "\n"
      | "if" oper block_id          "\n"
      | "print" oper                "\n"
      | "dump"                      "\n"
```
TODO: Add more details about some of the special operations

#### Branch Condition / Operation
```
branch = "ret" oper?                     "\n"
       | "jmp" block_id                  "\n"
       | "cmp" oper block_id block_id    "\n"
```

#### Operands
```
oper = temp | const
```

#### Temporary or Register
```
temp = "#" uint
     | "#" register
```

#### Register
```
register = eax
         | ebx
         | ecx
         | edx
         | edi
         | esi
         | ebp
         | r8d
         | r9d
         | r10d
         | r11d
         | r12d
         | r13d
         | r14d
         | r15d
```

#### Name of a Block
```
block_id = "@" uint
```

#### A Name or id
```
id = [a-zA-Z_][a-zA-Z0-9_]*
```

#### An Integer Number Value
```
const = (-?)(0 | [1-9][0-9]*)    # Decimal
      | 0[xX][0-9a-fA-F]+        # Hex
```

#### Natural Number / Unsigned Integer
```
uint = 0 | [1-9][0-9]*
```

#### Unary Operations
```
unop = "-"     # Negation
     | "~"     # Bitwise Not
     | "!"     # Logical Not
```

#### Binary Operations
```
binop = "+"     # Add
      | "-"     # Subtract
      | "*"     # Multiply
      | "/"     # Divide
      | "%"     # Mod
      | "<<"    # Arithmetic Left Shift
      | ">>"    # Arithmetic Right Shift
      | ">>>"   # Logical Right Shift
      | "=="    # Equality
      | "!="    # Inequality
      | "<"     # Less Than
      | "<="    # Less Than or Equal
      | ">"     # Greater Than
      | ">="    # Greater Than or Equal
      | "||"    # Logical Or
      | "&&"    # Logical And
      | "|"     # Bitwise Or
      | "^"     # Bitwise Xor
      | "&"     # Bitwise And
```

