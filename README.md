# 15-411 VM for Abstract Assembly

This repo contains the source for the Abstract Assembly Virtual
Machine that can be used to test compilers created for the 15-411
Compilers course. For now, the VM will only guarantee to support C0
features up to Lab 3. The abstract assembly will respresent the SSA
form of a CFG produced in the intermediate stages of a compiler.


## TOV

- [Abstract Assembly Language](#assembly-structure)
- [Extending the VM](#extending-the-vm)
- [Installation](#installation)
- [Using the VM](#usage)

## Assembly Structure

TODO: The design of the abstract assembly language is a
work-in-progress. Please let me know if you have any thoughts about
the structure, notation or anything you think that can be improved
upon.

The syntax in BNF form can be found in [format.txt](./format.txt).


### Identifiers

- Temporaries / Variables start with the letter `t` followed by a number, i.e. `t10`. May want to add support for named temps so we can do something like LLVM and have vars start with a symbol like `%`, `%10` or `%hello`.
- Basic Block labels start with the letter `B` followed by a number, i.e. `B10`. Same thoughts as before.
- Constant / Numbers are just unsigned 64-bit values. They are not prefixed with any symbols. They follow C0 conventions.
- Identifiers / Names also follow C0 conventions.
- An operand is either a temp or a constant.


### Instructions

For now, we will support the following 4 instructions:
- Move `{dest} = {src}`
  - `dest`: Temporary
  - `src`: Operand
- Unary Operation `{dest} = {op}{src}`
  - `dest`: Temporary
  - `src`: Operand
  - `{op}`: Unary Operation
- Binary Operation `{dest} = {src1} {op} {src2}`
  - `dest`: Temporary
  - `src1`: Operand
  - `op`: Binary Operation
  - `src2`: Operand
- Function Call `{dest} = {name}({srcs})`
  - `dest`: Temporary
  - `name`: Name of Function
  - `srcs`: Comma separated list of Operands

TODO: Discussing support for adding new instructions. This will
require some some of Trait-based system (similar to Java interfaces)
for dynamic expansion of instructions. The trait will have some
functions for identifying, parsing, and evaluating the
instruction. The problem is how to expand the Lexer to support new
identifiers. This might require using a custom lexer solution instead
of the Logos library we are currently using.


### Blocks

A basic block will start with a label and header. The label will
consist of a the block id (`B10`) followed by a list of block ids of
the blocks predecessors and then a colon. The predecessors are only
necessary if PHI functions are included in the block. After the label
comes a list of phi functions. It will look like so:

```
B9(B1, B2):
...

// Remember spacing doesnt matter
B10       ( B1, B2, B3 ):
t10 = PHI ( t2, t3, t4 )
```

Finally will be the list of instructions. Every block will end with one of the following special instructions:
- `ret {Operand}?`: Return a value or temporary at the end of the block
- `jmp {block-id}`: Jump to the indicated block
- `if {cond} {block-id-1} {block-id-2}`: Run the conditional operation. If it is true, then jump to the first block. Otherwise, jump to the second. The conditional can be the following
  - Binary Operation Expression `{ltemp} {op} {rtemp}`
  - Temporary (for a boolean value)


### Functions

At the top level, the assembly file will be divided into a list of
functions. Each function will start with an identifier followed by a
comma-separated list of temporaries for parameters, and then a
colon. It is then followed by a list of blocks

## Extending the VM

## Installation

## Usage


