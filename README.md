# Munin: Execution-Time Space Complexity Analysis

## Installation instructions

```
git clone git@github.com:ellifteria/munin
cd munin
sh install.sh
```

## Usage

```
./munin -a [0, 1, 2, 3] -p [0, 1]
```

| `-a` value | algorithm |
| --- | --- |
| 0 | ADD |
| 1 | PAL-ADD |
| 2 | LIN-ADD |
| 3 | PAL |

| `-p` value | pretty printing of variable values |
| --- | --- |
| 0 | off |
| 1 | on |

## Assembly reference

### Operations

| Operator | Operand 1 | Operand 2 | Operand 3 | Description |
| --- | --- | --- | --- | --- |
| `set` | `D` | `S` | | Sets variable D equal to the value of S |
| `set-to-length-of` | `D` | `S` | | Sets variable D equal to the length of S |
| `set-to-nth-bit` | `D` | `S` | `N` | Sets variable D equal to the Nth bit of S |
| `int-add` | `D` | `S` | | Sets variable D equal to the value of D + S |
| `int-subtract` | `D` | `S` | | Sets variable D equal to the value of D - S |
| `bit-add` | `D` | `S` | | Sets one-bit variable D equal to the value of the binary sum of one-bit D and one-bit S; sets carry flag |
| `bit-subtract` | `D` | `S` | | Sets one-bit variable D equal to the value of the binary subtraction of one-bit D and one-bit S; sets the underflow flag |
| `bit-shift-right` | `D` | `S` | | Sets variable D equal to the value of D << S |
| `bit-shift-left` | `D` | `S` | | Sets variable D equal to the value of D >> S |
| `compare` | `A` | `B` | | Sets the equal flag if A == B ; sets the greater flag if A > B |
| `clear-flags` | | | | Clears all flags |
| `jump-to` | `less` | | | Jumps to line less |
| `jump-over-next-if` | `carry` | | | Jumps over the next instruction if the condition carry is true |
| `end` | | | | Ends the program |

### Variables

Munin uses three types of variables:

| Type | Restrictions | Prefix | Example (first available variable)
| --- | --- | --- | --- |
| Variable | Can be read and written in the `INPUT` and `EXECUTION` phases; are cleared upon entering the `EXECUTION` phase | `v` | `v0` |
| Bit | Are only one bit in size; can be read and written in the `INPUT` and `EXECUTION` phases; are cleared upon entering the `EXECUTION` phase | `b` | `b0` |
| Input | Can be read and written in the `INPUT` phase; can only be read in the `EXECUTION` phase | `i` | `i0` |

Variables must be created sequentially starting at `0` for each type of variable.
This means that the first variable created must be `v0` followed by `v1`, `v2`, `v3`, etc.
The number of variables in each type of memory does not impact the numbering of variables in other types.
As such, the first variable, bit, and input must be `v0`, `b0`, and `i0`, regardless of whether or not there are other variables, bits, and inputs.

Leading `0`s do not matter; `v0`, `v00`, `v000000000`, etc. are all equivalent in Munin assembly.

### Flags

The Munin virtual device has four flags: `EQUAL`, `GREATER`, `CARRY`, and `UNDERFLOW`.
Conditions are the following:

| Condition | Meaning | Flags |
| --- | --- | --- |
| `equal` | Equal | `EQUAL` |
| `not-equal` | Not equal | `!EQUAL` |
| `greater` | Greater than | `GREATER` |
| `greater-or-equal` | Greater than or equal to | `GREATER ^ EQUAL` |
| `less` | Less than | `!EQUAL & !GREATER` |
| `less-or-equal` | Less than or equal to | `!GREATER` |
| `carry` | Carry | `CARRY` |
| `no-carry` | No carry | `!CARRY` |
| `underflow` | Underflow | `UNDERFLOW` |
| `no-underflow` | No underflow | `!UNDERFLOW` |


### Program flow

`jump-to` operations in Munin jump to line numbers.
Line numbers in Munin assembly start at `0`, not `1`.
