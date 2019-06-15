# Lemon_Rust

A port of the Lemon Parser Generator  to Rust.

## Deprecation Warning!
This project is being deprecated in favor of [_pomelo_](https://crates.io/crates/pomelo), a reimplementation of this very same code as a Rust procedural macro. You should use that for any new code, and also consider porting your existing code.

## Introduction

From the original [Lemon documentation](http://www.hwaci.com/sw/lemon/lemon.html):

> Lemon is an LALR(1) parser generator for C or C++. It does the same job as "bison" and "yacc". But lemon is not another bison or yacc clone. It uses a different grammar syntax which is designed to reduce the number of coding errors. Lemon also uses a more sophisticated parsing engine that is faster than yacc and bison and which is both reentrant and thread-safe.

Change _C_ or _C++_ to _Rust_ and you will get also the [well known benefits](http://www.rust-lang.org/) of this language.

My thanks and acknowledgement to D. Richard Hipp, the original creator of Lemon.

## Differences between Lemon_C and Lemon_Rust

Lemon_Rust works basically the same as Lemon_C. The main difference is, obviously, that it generates Rust code instead of C code. However there are a few other differences, due to the differences in the generated languages.

In the grammar file there are basically the following differences:

1. There is no `%name` or `%token_prefix`directives. In C all the symbol names are global so this directive allows you to change the name of the generated symbols. In Rust, the generated file will be compiled as a _crate_ and it will have its own namespace.  So changing the names of the generated symbols is not needed.
2. There are no destructors (`%destructor`, `%token_destructor` and `%default_destructor`). If you need to write destructors, just implement the `Drop` trait for the associated type.
3. There is no `%token_type` directive. In Lemon_C, all tokens must have the same type, while non-terminal symbols may have different types. In Lemon_Rust there is no such distinctions, so you can specify the type for each individual token with ` %type`.
4. There is a new `%derive_token` directive. It is used to add the `#[derive()]` directive to the generated `Token` type.

In the invocation of the program:

1. There is no `-m` option. That was for writing a `makeheaders` file, but Rust does not need that.
2. The default template file is not `lempar.c` but `lempar.rs`.

## Detailed documentation

See the [`DOCUMENTATION.md`](./DOCUMENTATION.md) file included in the source tree.
