# Munin: Execution-Time Space Complexity Analysis

## Installation instructions

```
git clone git@github.com:ellifteria/munin
cd munin
sh install.sh
```

## Usage

### Munin

```
./munin -f path_to_munin_assembly_file -i input input input
```

### Munin assembler

```
./munin-compiler -f path_to_munin_super_basic_file -o path_to_munin_assembly_output
```

### Example algorithms

```
./munin-examples -a [0, 1, 2, 3] -p [0, 1]
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
| `set` | `D` | `S` | | Sets variable `D` equal to the value of `S` |
| `stl` | `D` | `S` | | Sets variable `D` equal to the length of `S` |
| `stnb` | `D` | `S` | `N` | Sets variable `D` equal to the `N`th bit of `S` |
| `iadd` | `D` | `S` | | Sets variable `D` equal to the value of `D + S` |
| `isub` | `D` | `S` | | Sets variable `D` equal to the value of `D - S` |
| `badd` | `D` | `S` | | Sets one-bit variable `D` equal to the value of the binary sum of one-bit `D` and one-bit `S`; sets `CARRY` flag |
| `bsub` | `D` | `S` | | Sets one-bit variable `D` equal to the value of the binary subtraction of one-bit `D` and one-bit `S`; sets the `UNDERFLOW` flag |
| `bsr` | `D` | `S` | | Sets variable `D` equal to the value of `D << S` |
| `bsl` | `D` | `S` | | Sets variable `D` equal to the value of `D >> S` |
| `cmp` | `A` | `B` | | Sets the `EQUAL` flag if `A == B` ; sets the `GREATER` flag if `A > B` |
| `clf` | | | | Clears all flags |
| `jmp` | `L` | | | Jumps to line `L` |
| `jon` | `C` | | | Jumps over the next instruction if the condition `C` is true |
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
| `e` | Equal | `EQUAL` |
| `ne` | Not equal | `!EQUAL` |
| `g` | Greater than | `GREATER` |
| `ge` | Greater than or equal to | `GREATER ^ EQUAL` |
| `l` | Less than | `!EQUAL & !GREATER` |
| `le` | Less than or equal to | `!GREATER` |
| `c` | Carry | `CARRY` |
| `nc` | No carry | `!CARRY` |
| `u` | Underflow | `UNDERFLOW` |
| `nu` | No underflow | `!UNDERFLOW` |


### Program flow

`jmp` operations in Munin jump to line numbers.
Line numbers in Munin assembly start at `0`, not `1`.
