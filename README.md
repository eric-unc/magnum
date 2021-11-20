# MagnumVM
MagnumVM is a custom process virtual machine. Made for [queer_hack 2021](https://queer-hack21.devpost.com/).

<img src="gayrust.jpg" width="200">

_(This image is [not made by me](https://twitter.com/whoisaldeka/status/1165148059484880896) and is licensed under CC-BY. Unfortunately there is not a bisexual version.)_

## Components
* **MagnumVM**: a virtual machine for executing Magna (binary executable) files.
* **Scriba**: a library for writing Magna files.
* **Magnum Common**: a library with common constants.

## Specifications
### Magna (binary executable)
MagnumVM targets a custom binary executable format called ***Magna***. This format should have the file extension `.magna`.

Magna is loosely inspired by [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format).

#### Header
| Offset | Size (bytes) | Description
| :------ | :------ | :------
| `0x0` | 3 | `MVM` in ASCII/`0x4D564D` in hex; the [file signature/magic number](https://en.wikipedia.org/wiki/List_of_file_signatures).
| `0x3` | 1 | `0x0`; the target version of MagnumVM.
| `0x4` | 8 | `0x35`; starting location (in bytes) of text section.
| `0xC` | 8 | Size (in bytes) of text section.
| `0x14` | 8 | Location of read-only section.
| `0x1C` | 8 | Size of read-only section.
| `0x24` | 8 | Location of initialized writable section.
| `0x2C` | 8 | Size of initialized writable section.
| `0x34` | 8 | Size of uninitialized writable memory.

#### Text section
The text section is right after the header, so it should always have the offset `0x35`. This section contains the actual instructions to be executed by MagnumVM. Each instruction is 32-bits.

#### Read-only section
The read only section is after the text section. This section is intended for non-executable read-only data, such as constants.

#### Initialized writable section
The initialized writable section is after the read-only section. This section is intended for non-executable, writable initialized data, such as global variables.

#### Uninitialized writable data
Because uninitialized writable data is uninitialized, it does not need a section, but it does need a specified size for runtime.

### MagnumVM (runtime organization)
#### Memory organization
Memory is divided into two pools: text and data. In the future, it could also have a heap, but implementing a garbage collector is a lot of work.

The text section of memory is read-only. The data section of memory is divided into the read-only section, the global writable section, and the stack.

#### Instructions
Each instruction is 32-bits, with the first 8 being the opcode. Immediates follow little endian.

| Instruction | Opcode | Arguments | Description
| :------ | :------ | :------ | :------
| `nop` | `0x00` | N/A | No operation.
| `loadi_b` | `0x01` | Immediate (value): `inst[0..7]` | Loads immediate byte onto stack.
| `loadi_2b` | `0x02` | Immediate (value): `inst[0..15]` | Loads immediate double byte onto stack.
| `loadi_4b` | `0x03` | Immediate (value): `inst[0..23]` | Loads immediate quadruple byte onto stack. Sign not carried.
| `loadi_8b` | `0x04` | Immediate (value): `inst[0..23]` | Loads immediate octuple byte onto stack. Sign not carried.
| `load` | `0x10` | Immediate (size): `inst[16..23]`, immediate (address): `inst[0..15]` | Loads a byte/bytes from memory address onto stack.
| `loads` | `0x11` | Immediate (size): `inst[16..23]`, immediate (address): `inst[0..15]` | Loads a byte/bytes from offset of stack base onto stack.
| `store` | `0x15` | Immediate (size): `inst[16..23]`, immediate (address): `inst[0..15]` | Store a byte/bytes from stack (popping them) to memory location.
| `pop` | `0x20` | Immediate (amount of bytes): `inst[0..7]` | Pops a byte/bytes from stack.
| `func_b` | `0x30` | Function (opcode): `inst[0..7]` | Treats byte(s) from stack as `u8` and performs function from values (see functions README section).
| `sys` | `0x40` | System call (opcode): `inst[0..7]` | Performs system call (see system calls README section).

##### Functions
| Function | Function opcode | Description
| :------ | :------ | :------
| `add` | `0x00` |
| `sub` | `0x01` | 
| `mul` | `0x02` |
| `div` | `0x03` |

##### System calls
| System call | System call opcode | Description
| :------ | :------ | :------
| `put_b` | `0x00` | Prints value of byte as a `u8`.
| `put_c` | `0x10` | Prints byte as an ASCII char.

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
  * [Cargo](https://github.com/rust-lang/cargo)

## Authors
1. Eric Schneider
