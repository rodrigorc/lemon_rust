# Lemon\_Rust

A port of the Lemon Parser Generator  to Rust.

##Introduction

From the original [Lemon documentation](http://www.hwaci.com/sw/lemon/lemon.html):

> Lemon is an LALR(1) parser generator for C or C++. It does the same job as ``bison'' and ``yacc''. But lemon is not another bison or yacc clone. It uses a different grammar syntax which is designed to reduce the number of coding errors. Lemon also uses a more sophisticated parsing engine that is faster than yacc and bison and which is both reentrant and thread-safe.

Change _C or C++_ to _Rust_ and you will get also the [well known benefits](http://www.rust-lang.org/) of this language.

My thanks and acknowledgement to D. Richard Hipp, the original creator of Lemon.

##Differences between Lemon\_C and Lemon\_Rust

Lemon\_Rust works basically the same as Lemon\_C. The main difference is, obviously, that it generates Rust code instead of C code. However there are a few other differences, due to the differences in the generated languages.

In the grammar file there are basically the following differences:

1. There is no `%name` or `%token_prefix`directives. In C all the symbol names are global so this directive allows you to change the name of the generated symbols. In Rust, the generated file will be compiled as a _crate_ and it will have its own namespace.  So changing the names of the generated symbols is not needed.
2. There are no destructors (`%destructor`, `%token_destructor` and `%default_destructor`). If you need to write destructors, just implement the `Drop` trait for the associated type.
3. There is no `%token_type` directive. In Lemon\_C, all tokens must have the same type, while non-terminal symbols may have different types. In Lemon\_Rust there is no such distinctions, so you can specify the type for each individual token with ` %type`.

In the invocation of the program:

1. There is no `-m` option. That was for writing a `makeheaders` file, but Rust does not need that.
2. The default template file is not `lempar.c` but `lempar.rs`.

##Detailed documentation

To be completed...
