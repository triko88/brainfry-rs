# BrainFry-rs

My Brainfuck interpreter re-written in Rust as a learning practice. I wanted to
learn Rust given now I'm a C++ developer which is relevant to my career,
especially when it comes to writing compilers. Brainfuck is an extremely
minimal esoteric programming language that has no good use except you can have
a programming language with merely 8 tokens. But for comp-sci nerds like me,
this is a good practice to write a compiler in a different programming
language.

For those who want to check out my interpreter in C++:
https://github.com/triko88/BrainFry

I have explained the underlying theory in this blog: 
https://pixeltrik.vercel.app/blogs/turing-machines-and-brainfry

Much like the original implementation, this is in form of a library but has few
key differences.

## Usage
Given it's a small and inconsequential project for most people except me
**(it acts as a proof to recruiters that I know how to program on Rust)**, I
didn't shared the crate to crates.io. But given this is a library-binary hybrid
crate, you can use it in either way.

### As Library
You want to use it as a library? Don't worry, Rust's superior package
management system `cargo` will handle that for you. All you need to do is use
the public functions as per your program's logic. 

In `Cargo.toml` of your project, type

```
[dependencies]
"brainfry_rs" = { git = "https://github.com/triko88/brainfry-rs" }
```

There is only one public function, `decode()` with parameters

    decode(src: String, debug: bool) -> String

### As Binary
For this you need to clone this repository and in the repository directory,
type

    cargo build

And use 

    brainfry-rs <filename> [--debug]

You can use `--debug` to see the state of memory tape after execution.

## So far what has changed?
Not much, after all this is a small interpreter for a very small language,
however there are some changes worth mentioning.

- **Mapping brackets pre-interpretation.** This one was different for me as in
original program, the program counter has to be moved all the way to the
matching bracket using a stack. This got reduced by pre-computing indices
acting as a primitive variation of symbol table.

- **Using tests to verify my design**, this wasn't in the original one as all tests
I did was manual. Rust's internal tools for unit testing makes TDD more
effective than in C++.

- The only downgrade in this program is using **only standard I/O** and not generic
I/O streams. This is due to my ignorance on streams in Rust, since I found this
good enough, I'm uploading this here.
