#The Lemon\_Rust Parser Generator

Lemon\_Rust is a port to Rust of the Lemon Parser Generator (from now on, Lemon\_C) originally written by D. Richard Hipp for his SQLite parser.
The program itself it is still written in C, but the generated code is 100% Rust.

This Lemon\_Rust guide is shamelessly based on the original [Lemon\_C guide](http://www.hwaci.com/sw/lemon/lemon.html).

*Lemon* is an LALR(1) parser generator for Rust. It does the same job as *bison* and *yacc*. But *lemon* is not another *bison* or *yacc* clone.
It uses a different grammar syntax which is designed to reduce the number of coding errors.
*Lemon* also uses a more sophisticated parsing engine that is faster than *yacc* and *bison* and which is both reentrant and thread-safe.

This document is an introduction to the *lemon* parser generator.

## Theory of Operation

The main goal of *lemon* is to translate a context free grammar (CFG) for a particular language into Rust code that implements a parser for that language.
The program has two inputs:

 * The grammar specification.
 * A parser template file.

Typically, only the grammar specification is supplied by the programmer. *lemon* comes with a default parser template which works fine for most applications. But the user is free to substitute a different parser template if desired.

Depending on command-line options, *lemon* will generate one or two files:

 * Rust code to implement the parser.
 * An information file that describes the states of the generated parser automaton.

By default, both output files are generated. The report file is omitted when `-q` is selected.

The grammar specification file uses a `.y` suffix, by convention. In the examples used in this document, we'll assume the name of the grammar file is `gram.y`. A typical use of *lemon* would be the following command:

    lemon_rust gram.y

This command will generate two output files named `gram.rs` and `gram.out`. The first is Rust code to implement the parser, the second is the report that explains the states used by the parser automaton.

## Command Line Options

The behavior of *lemon* can be modified using command-line options. You can obtain a list of the available command-line options together with a brief explanation of what each does by typing:

    lemon -?

As of this writing, the following command-line options are supported:

 * -b
 * -c
 * -g
 * -q
 * -s
 * -x

The `-b` option reduces the amount of text in the report file by printing only the basis of each parser state, rather than the full configuration.
The `-c` option suppresses action table compression. Using `-c` will make the parser a little larger and slower but it will detect syntax errors sooner.
The `-g` option causes no output files to be generated at all. Instead, the input grammar file is printed on standard output but with all comments, actions and other extraneous text deleted. This is a useful way to get a quick summary of a grammar.
The `-q` option suppresses the report file.
Using `-s` causes a brief summary of parser statistics to be printed. Like this:

    Parser statistics: 74 terminals, 70 nonterminals, 179 rules
                       340 states, 2026 parser table entries, 0 conflicts

Finally, the `-x` option causes *lemon* to print its version number and copyright information and then stop without attempting to read the grammar or generate a parser.

## The Parser Interface

*Lemon* doesn't generate a complete, working program. It only generates a Rust crate that implement a parser. This section describes the interface to that crate.

Before a program begins using a *lemon*-generated parser, the program must first create the parser. A new parser is created as follows:

    let mut parser = gram::Parser::new(arg);

Here, `gram` is the generated module, maybe from a `mod parser;` declaration. `Parser` is the struct that represents the parser and `new()` the function that creates and initializes a new parser.

The `new()`function may an argument, depending on the gramar. If the grammar specification file request it, the `new()` function will have a parameter that can be of any type chosen by the programmer. The parser doesn't do anything with this argument except to pass it through to action routines. This is a convenient mechanism for passing state information down to the action routines without having to use global variables.

After a parser has been created, the programmer must supply it with a sequence of tokens (terminal symbols) to be parsed. This is accomplished by calling the following function once for each token:

    parser.parse(token);

The first argument to the `parse()` function is a value of the generated `Token` enumeration that tells the parser the type of the next token in the data stream. There is one token variant for each terminal symbol in the grammar. (A value of `Token::EOI` is a special flag to the parser to indicate that the end of input has been reached.) Some variants will have an associated value, depending on the type of the token. Typically the token variant will be a broad category of tokens such as _identifier_ or _number_ and the data will be the name of the identifier or the value of the number.

Note that this function will take ownership of the passed token, unless it implements the `Copy` trait (see `%derive_token`).

A typical use of a *lemon* parser might look something like the following:

    fn parse_expression<R: BufRead>(read: &mut R) -> gram::Expression {
        let tokenizer = Tokenizer::new(read);
        let mut parser = gram::Parser::new(gram::State::new());
        while let Some(token) = match tokenizer.next_token() {
            parser.parse(token);
        }
        parser.parse(gram::Token::EOI);
        Ok(parser.into_extra().into_tree())
    }

This example shows a user-written routine that parses an input stream and returns a parse tree. (We've omitted all error-handling from this example to keep it simple.) We assume the existence of some kind of tokenizer which is created using `Tokenizer::new()`. The `Tokenizer::next_token()` function on retrieves the next token from the input file and returns an `Option<gram::Token>`. The enum data is assumed to be some type that contains details about each token, such as its complete text, what line it occurs on, etc.

This example also assumes the existence of structure of type `gram::State` that holds state information about a particular parse. An instance of such a type is created with a call to `gram::State::new()` and then passed into the parser upon initialization, as the optional argument. The action routine specified by the grammar for the parser can use the this value to hold whatever information is useful and appropriate. This value can be borrowed using the function `parser.extra()` or moved out of the parser with `parser.into_extra()`. In the example, we note that the `into_tree()` function will convert this value into the root of the parse tree.

The core of this example as it relates to *lemon* is as follows:

    fn parse_file() {
        let mut parser = gram::Parser::new();
        while let Some(token) = get_next_token() {
            parser.parse(token);
        }
        parser.parse(gram::Token::EOI);
    }

Basically, what a program has to do to use a *lemon*-generated parser is first create the parser, then send it lots of tokens obtained by tokenizing an input source. When the end of input is reached, the `parse()` routine should be called one last time with a token type of `EOI`. This step is necessary to inform the parser that the end of input has been reached.

<!-- NO TRACE yet!

There is one other interface routine that should be mentioned before we move on. The ParseTrace() function can be used to generate debugging output from the parser. A prototype for this routine is as follows:

   ParseTrace(FILE *stream, char *zPrefix);
After this routine is called, a short (one-line) message is written to the designated output stream every time the parser changes states or calls an action routine. Each such message is prefaced using the text given by zPrefix. This debugging output can be turned off by calling ParseTrace() again with a first argument of NULL (0).
 --> 

## Differences With *yacc* and *bison*

Programmers who have previously used the *yacc* or *bison* parser generator will notice several important differences between *yacc* and/or *bison* and *lemon*.

 * In *yacc* and *bison*, the parser calls the tokenizer. In *lemon*, the tokenizer calls the parser.
 * *lemon* uses no global variables. *yacc* and *bison* use global variables to pass information between the tokenizer and parser.
 * *lemon* allows multiple parsers to be running simultaneously. *yacc* and *bison* do not.

These differences may cause some initial confusion for programmers with prior *yacc* and *bison* experience. But after years of experience using *lemon*, I firmly believe that the *lemon* way of doing things is better.

## Input File Syntax

The main purpose of the grammar specification file for *lemon* is to define the grammar for the parser. But the input file also specifies additional information *lemon* requires to do its job. Most of the work in using *lemon* is in writing an appropriate grammar file.

The grammar file for *lemon* is, for the most part, free format. It does not have sections or divisions like *yacc* or *bison*. Any declaration can occur at any point in the file. *lemon* ignores whitespace (except where it is needed to separate tokens) and it honors the same commenting conventions as Rust.

### Terminals and Nonterminals

A terminal symbol (token) is any string of alphanumeric and underscore characters that begins with an upper case letter. A terminal can contain lowercase letters after the first character, but the usual convention is to make terminals all upper case. A nonterminal, on the other hand, is any string of alphanumeric and underscore characters than begins with a lower case letter. Again, the usual convention is to make nonterminals use all lower case letters.

In *lemon*, terminal and nonterminal symbols do not need to be declared or identified in a separate section of the grammar file. *lemon* is able to generate a list of all terminals and nonterminals by examining the grammar rules, and it can always distinguish a terminal from a nonterminal by checking the case of the first character of the name.

*yacc* and *bison* allow terminal symbols to have either alphanumeric names or to be individual characters included in single quotes, like this: ')' or '$'. *lemon* does not allow this alternative form for terminal symbols. With *lemon*, all symbols, terminals and nonterminals, must have alphanumeric names.

### Grammar Rules

The main component of a *lemon* grammar file is a sequence of grammar rules. Each grammar rule consists of a nonterminal symbol followed by the special symbol `::=` and then a list of terminals and/or nonterminals. The rule is terminated by a period. The list of terminals and nonterminals on the right-hand side of the rule can be empty. Rules can occur in any order, except that the left-hand side of the first rule is assumed to be the start symbol for the grammar (unless specified otherwise using the `%start` directive described below.) A typical sequence of grammar rules might look something like this:

    expr ::= expr PLUS expr.
    expr ::= expr TIMES expr.
    expr ::= LPAREN expr RPAREN.
    expr ::= VALUE.

There is one non-terminal in this example, `expr`, and five terminal symbols or tokens: `PLUS`, `TIMES`, `LPAREN`, `RPAREN` and `VALUE`.

Like *yacc* and *bison*, *lemon* allows the grammar to specify a block of code that will be executed whenever a grammar rule is reduced by the parser. In *lemon*, this action is specified by putting the code (contained within curly braces {...}) immediately after the period that closes the rule. For example:

    expr ::= expr PLUS expr.   { println!("Doing an addition..."); }

In order to be useful, grammar actions must normally be linked to their associated grammar rules. In *yacc* and *bison*, this is accomplished by embedding a `$$` in the action to stand for the value of the left-hand side of the rule and symbols `$1`, `$2`, and so forth to stand for the value of the terminal or nonterminal at position 1, 2 and so forth on the right-hand side of the rule. This idea is very powerful, but it is also very error-prone. The single most common source of errors in a *yacc* or *bison* grammar is to miscount the number of symbols on the right-hand side of a grammar rule and say `$7` when you really mean `$8`.

*Lemon* avoids the need to count grammar symbols by assigning symbolic names to each symbol in a grammar rule and then using those symbolic names in the action. In *yacc* or *bison*, one would write this:

    expr -> expr PLUS expr  { $$ = $1 + $3; };

But in *lemon*, the same rule becomes the following:

    expr(A) ::= expr(B) PLUS expr(C).  { A = B + C; }

In the *lemon* rule, any symbol in parentheses after a grammar rule symbol becomes a place holder for that symbol in the grammar rule. This place holder can then be used in the associated Rust action to stand for the value of that symbol.
The *lemon* notation for linking a grammar rule with its reduce action is superior to *yacc*/*bison* on several counts. First, as mentioned above, the *lemon* method avoids the need to count grammar symbols. Secondly, if a terminal or nonterminal in a *lemon* grammar rule includes a linking symbol in parentheses but that linking symbol is not actually used in the reduce action, then an error message is generated. For example, the rule

    expr(A) ::= expr(B) PLUS expr(C).  { A = B; }

will generate an error because the linking symbol `C` is used in the grammar rule but not in the reduce action.

If you have several terminal tokens that can be used in the same place you can put them all in the same rule, separated with `|`.

    expr(A) ::= SMALLNUMBER|BIGNUMBER(B).  { A = B; }

which is a shortcut of

    expr(A) ::= SMALLNUMBER(B).  { A = B; }
    expr(A) ::= BIGNUMBER(B).    { A = B; }

If you use a symbolic name (`(B)` in the example) with such a compound token, then all these tokens must be of the same type. However, if there is no symbolic name, then they may have different types.

### Precedence Rules

*lemon* resolves parsing ambiguities in exactly the same way as *yacc* and *bison*. A shift-reduce conflict is resolved in favor of the shift, and a reduce-reduce conflict is resolved by reducing whichever rule comes first in the grammar file.

Just like in *yacc* and *bison*, *lemon* allows a measure of control over the resolution of paring conflicts using precedence rules. A precedence value can be assigned to any terminal symbol using the `%left`, `%right` or `%nonassoc` directives. Terminal symbols mentioned in earlier directives have a lower precedence that terminal symbols mentioned in later directives. For example:

    %left AND.
    %left OR.
    %nonassoc EQ NE GT GE LT LE.
    %left PLUS MINUS.
    %left TIMES DIVIDE MOD.
    %right EXP NOT.

In the preceding sequence of directives, the `AND` operator is defined to have the lowest precedence. The `OR` operator is one precedence level higher. And so forth. Hence, the grammar would attempt to group the ambiguous expression

    a AND b OR c

like this

    a AND (b OR c)

The associativity (left, right or nonassoc) is used to determine the grouping when the precedence is the same. `AND` is left-associative in our example, so

    a AND b AND c

is parsed like this

    (a AND b) AND c

The `EXP` operator is right-associative, though, so

    a EXP b EXP c

is parsed like this

    a EXP (b EXP c)

The nonassoc precedence is used for non-associative operators. So

    a EQ b EQ c

is an error.

The precedence of non-terminals is transferred to rules as follows: The precedence of a grammar rule is equal to the precedence of the left-most terminal symbol in the rule for which a precedence is defined. This is normally what you want, but in those cases where you want to precedence of a grammar rule to be something different, you can specify an alternative precedence symbol by putting the symbol in square braces after the period at the end of the rule and before any Rust-code. For example:

    expr = MINUS expr.  [NOT]

This rule has a precedence equal to that of the NOT symbol, not the MINUS symbol as would have been the case by default.

With the knowledge of how precedence is assigned to terminal symbols and individual grammar rules, we can now explain precisely how parsing conflicts are resolved in *lemon*. Shift-reduce conflicts are resolved as follows:

 * If either the token to be shifted or the rule to be reduced lacks precedence information, then resolve in favor of the shift, but report a parsing conflict.
 * If the precedence of the token to be shifted is greater than the precedence of the rule to reduce, then resolve in favor of the shift. No parsing conflict is reported.
 * If the precedence of the token it be shifted is less than the precedence of the rule to reduce, then resolve in favor of the reduce action. No parsing conflict is reported.
 * If the precedences are the same and the shift token is right-associative, then resolve in favor of the shift. No parsing conflict is reported.
 * If the precedences are the same the the shift token is left-associative, then resolve in favor of the reduce. No parsing conflict is reported.
 * Otherwise, resolve the conflict by doing the shift and report the parsing conflict.

Reduce-reduce conflicts are resolved this way:

 * If either reduce rule lacks precedence information, then resolve in favor of the rule that appears first in the grammar and report a parsing conflict.
 * If both rules have precedence and the precedence is different then resolve the dispute in favor of the rule with the highest precedence and do not report a conflict.
 * Otherwise, resolve the conflict by reducing by the rule that appears first in the grammar and report a parsing conflict.

### Special Directives

The input grammar to *lemon* consists of grammar rules and special directives. We've described all the grammar rules, so now we'll talk about the special directives.

Directives in *lemon* can occur in any order. You can put them before the grammar rules, or after the grammar rules, or in the mist of the grammar rules. It doesn't matter. The relative order of directives used to assign precedence to terminals is important, but other than that, the order of directives in *lemon* is arbitrary.

*lemon* supports the following special directives:

 * `%extra_argument`
 * `%include`
 * `%code`
 * `%left`
 * `%name`
 * `%nonassoc`
 * `%parse_accept`
 * `%parse_failure`
 * `%right`
 * `%start_symbol`
 * `%syntax_error`
 * `%type`
 * `%fallback`
 * `%wildcard`
 * `%token_class`
 * `%derive_token`

Each of these directives will be described separately in the following sections:

#### The `%extra_argument` directive

The `%extra_argument` directive instructs *lemon* to add a parameter to the `Parser::new()` function it generates. *Lemon* doesn't do anything itself with this extra argument, but it does make the argument available to Rust-code action routines, and so forth, as the expression `self.extra`. For example, if the grammar file contains:

    %extra_argument { MyStruct }

Then the function generated will be of the form `Parser::new(extra: MyStruct)` and all action routines will have access to a member variable named `self.extra` of type `&mut MyStruct` that is the value of the stored argument.

Moreover, there will be two extra public member functions in the `Parser` struct: `pub fn extra(&self) -> &MyStruct` and `pub fn into_extra(self) -> MyStruct`.

#### The `%include` directive

The `%include` directive specifies Rust code that is included at the top of the generated parser. You can include any text you want -- the *lemon* parser generator copies to blindly. If you have multiple `%include` directives in your grammar file, their values are concatenated before being put at the beginning of the generated parser.

The `%include` directive is very handy for getting some file-level directives at the beginning of the generated parser. For example:

    %include { #![allow(unstable)] }

#### The `%code` directive

The `%code` directive is just like `%include` but the code is added at the end of the generated file, instead of at the beginning.

#### The `%left` directive

The `%left` directive is used (along with the `%right` and `%nonassoc` directives) to declare precedences of terminal symbols. Every terminal symbol whose name appears after a `%left` directive but before the next period (`.`) is given the same left-associative precedence value. Subsequent `%left` directives have higher precedence. For example:

    %left AND.
    %left OR.
    %nonassoc EQ NE GT GE LT LE.
    %left PLUS MINUS.
    %left TIMES DIVIDE MOD.
    %right EXP NOT.

Note the period that terminates each `%left`, `%right` or `%nonassoc` directive.

LALR(1) grammars can get into a situation where they require a large amount of stack space if you make heavy use or right-associative operators. For this reason, it is recommended that you use `%left` rather than `%right` whenever possible.

The `%nonassoc` directive

This directive is used to assign non-associative precedence to one or more terminal symbols. See the section on precedence rules or on the `%left` directive for additional information.

#### The `%parse_accept` directive

The `%parse_accept` directive specifies a block of Rust code that is executed whenever the parser accepts its input string. To _accept_ an input string means that the parser was able to process all tokens without error.

For example:

    %parse_accept {
        println!("parsing complete!");
    }

#### The `%parse_failure` directive

The `%parse_failure` directive specifies a block of Rust code that is executed whenever the parser fails to complete. This code is not executed until the parser has tried and failed to resolve an input error using is usual error recovery strategy. The routine is only invoked when parsing is unable to continue.

    %parse_failure {
        println!("Giving up.  Parser is hopelessly lost...");
    }

#### The `%right` directive

This directive is used to assign right-associative precedence to one or more terminal symbols. See the section on precedence rules or on the `%left` directive for additional information.

<!--
The %stack_overflow directive

The %stack_overflow directive specifies a block of C code that is executed if the parser's internal stack ever overflows. Typically this just prints an error message. After a stack overflow, the parser will be unable to continue and must be reset.

   %stack_overflow {
     fprintf(stderr,"Giving up.  Parser stack overflow\n");
   }
You can help prevent parser stack overflows by avoiding the use of right recursion and right-precedence operators in your grammar. Use left recursion and and left-precedence operators instead, to encourage rules to reduce sooner and keep the stack size down. For example, do rules like this:

   list ::= list element.      // left-recursion.  Good!
   list ::= .
Not like this:
   list ::= element list.      // right-recursion.  Bad!
   list ::= .
The %stack_size directive

If stack overflow is a problem and you can't resolve the trouble by using left-recursion, then you might want to increase the size of the parser's stack using this directive. Put an positive integer after the %stack_size directive and *lemon* will generate a parse with a stack of the requested size. The default value is 100.

   %stack_size 2000
-->

#### The `%start_symbol` directive

By default, the start symbol for the grammar that *lemon* generates is the first non-terminal that appears in the grammar file. But you can choose a different start symbol using the `%start_symbol` directive.

    %start_symbol prog

#### The `%syntax_error` directive

The `%syntax_error` directive specify code that will be called when a syntax error occurs. Inside this code, is run inside a private function with arguments `(&mut self, token: &Token)` where `self` points to the `Parser` object and
`token` points to the token that causes the error. See the section _Error Processing_ for more details.

#### The `%type` directive

This directive is used to specify the data types for values on the parser's stack associated with terminal and non-terminal symbols. Usually, you will make the type of the terminal symbols to some kind of token struct. The type associated to a non-terminal will be the type of the data associated to the corresponding variant of the `Token` enumeration. For example:

    %type VALUE { i32 }

Then the `Token` enumeration will have a variant such as:

    pub Token {
        ...
        VALUE(i32),
    }

Typically the data type of a non-terminal is a parse-tree structure that contains all information about that non-terminal For example:

    %type expr { Expr }

Each entry on the parser's stack is actually an enum containing variants of all data types for every symbol. *Lemon* will automatically use the correct element of this enum depending on what the corresponding symbol is. But the grammar designer should keep in mind that the size of the enum will be the size of its largest element. So if you have a single non-terminal whose data type requires 1K of storage, then your 100 entry parser stack will require 100K of heap space. If you are willing and able to pay that price, fine. You just need to know.

#### The `%fallback` directive

This directive defines an alternative token that will be used instead of another if the original one cannot be parsed. For example:

    %fallback ID X Y Z.

declares the token `ID` as a fallback for any of the other tokens. If the input stream passes any of these three tokens and they cannot be parsed, then the parser will try parsing an `ID` before considering it an error.

The fallback token (`ID` in the example) must have the same type of every other token that it replaces, or no type at all.

#### The `%wildcard` directive

This directive defines a token that will be used when any other token cannot be parsed. For example:

    %wildcard ANY.

The wildcard token must not have a type.

#### The `%token_class` directive

This directive declares a compound token class. For example:

    %token_class number INTEGER FLOAT DOUBLE.

is equivalent but more efficient than:

    number(A) ::= INTEGER(B). { A = B; }
    number(A) ::= FLOAT(B).   { A = B; }
    number(A) ::= DOUBLE(B).  { A = B; }

or also:

    number(A) ::= INTEGER|FLOAT|DOUBLE(B). { A = B; }

Naturally, if they use a symbolic name (`(A)` in the example), then all the tokens must have the same type.

#### The `%derive_token` directive

This directive declares traits that the Token enum should automatically
derive. For example:

    %derive_token {Debug,PartialEq}

results in the following code generated in the parser:

    #[derive(Debug,PartialEq
    )]
    pub enum Token {
        EOI, //0
        ...
    }

### Error Processing

After extensive experimentation over several years, it has been discovered that the error recovery strategy used by *yacc* is about as good as it gets. And so that is what *lemon* uses.

When a *lemon*-generated parser encounters a syntax error, it first invokes the code specified by the `%syntax_error` directive, if any. It then enters its error recovery strategy. The error recovery strategy is to begin popping the parsers stack until it enters a state where it is permitted to shift a special non-terminal symbol named `error`. It then shifts this non-terminal and continues parsing. But the `%syntax_error` routine will not be called again until at least three new tokens have been successfully shifted.

If the parser pops its stack until the stack is empty, and it still is unable to shift the error symbol, then the `%parse_failed` routine is invoked and the parser resets itself to its start state, ready to begin parsing a new file. This is what will happen at the very first syntax error, of course, if there are no instances of the `error` non-terminal in your grammar.
