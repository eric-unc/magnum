# MagnumVM
MagnumVM is a custom process virtual machine. Made for [queer_hack 2021](https://queer-hack21.devpost.com/).

<img src="gayrust.jpg" width="200">

_(This image is [not made by me](https://twitter.com/whoisaldeka/status/1165148059484880896) and is licensed under CC-BY. Unfortunately there is not a bisexual version.)_

## Components
* **MagnumVM**: a virtual machine for executing custom Magna files.
* **Scriba**: a library for writing binary Magna files.

## Specifications
### Magna (binary executable)
MagnumVM targets a custom binary executable format called ***Magna***. This format should have the file extension `.magna`.

Magna is loosely inspired by [ELF](https://en.wikipedia.org/wiki/Executable_and_Linkable_Format).

#### Header
| Offset | Size (bytes) | Description
| :------ | :------ | :------
| 0x0 | 3 | `MVM` in ASCII/`0x4D564D` in hex; the [file signature/magic number](https://en.wikipedia.org/wiki/List_of_file_signatures).
| 0x3 | 1 | `0x0`; the target version of MagnumVM.
| 0x4 | 4 | `0x1D`; location (in bytes) of text section.
| 0x8 | 4 | Size (in bytes) of text section.
| 0xC | 4 | Location of read-only section.
| 0x10 | 4 | Size of read-only section.
| 0x14 | 4 | Location of initialized writable section.
| 0x18 | 4 | Size of initialized writable section.
| 0x1C | 4 | Size of uninitialized writable memory.

#### Text section
The text section is right after the header, so it should always have the offset `0x1D`. This section contains the actual instructions to be executed by MagnumVM. Each instruction is 32-bits.

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
I'm not entirely sure how this will work just yet, but we'll see. Each instruction should be 32-bits, little endian. MagnumVM is basically a stack machine.

## Technologies used
* [Rust](https://github.com/rust-lang/rust)
  * [Cargo](https://github.com/rust-lang/cargo)

## Authors
1. Eric Schneider
