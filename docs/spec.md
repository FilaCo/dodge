# The Dodge Programming Language Specification

Language version: **dodge0.1**

## Table of contents

- [The Dodge Programming Language Specification](#the-dodge-programming-language-specification)
  - [Table of contents](#table-of-contents)
  - [Intro](#intro)
  - [Notation](#notation)
  - [Source Code Representation](#source-code-representation)
    - [Characters](#characters)
    - [Letters and Digits](#letters-and-digits)
  - [Lexical elements](#lexical-elements)
    - [Comments](#comments)
    - [Tokens](#tokens)
    - [Semicolons](#semicolons)
    - [Identifiers](#identifiers)
    - [Keywords](#keywords)
    - [Operators and punctuation](#operators-and-punctuation)
    - [Integer literals](#integer-literals)
    - [Predeclared identifiers](#predeclared-identifiers)

## Intro

This is the reference manual for the Dodge programming language.

Dodge is a domain specific language (**DSL**) for game development designed with Entity Component System (**ECS**) pattern and data oriented design (**DOD**) in mind. It is *statically* *strongly* typed. Programs are constructed from modules, whose properties allow efficient management of dependencies.

The syntax is compact and simple allowing for easy analysis by non-programmers, such as game designers.

## Notation

The syntax is specified using a [variant](https://en.wikipedia.org/wiki/Wirth_syntax_notation) of Extended Backus-Naur Form (EBNF):

```ebnf
Syntax      = { Production } .
Production  = production_name "=" [ Expression ] "." .
Expression  = Term { "|" Term } .
Term        = Factor { Factor } .
Factor      = production_name | token [ "…" token ] | Group | Option | Repetition .
Group       = "(" Expression ")" .
Option      = "[" Expression "]" .
Repetition  = "{" Expression "}" .
```

Productions are expressions constructed from terms and the following operators, in increasing precedence:

```text
|   alternation
()  grouping
[]  option (0 or 1 times)
{}  repetition (0 to n times)
```

Lowercase production names are used to identify lexical (terminal) tokens. Non-terminals are in CamelCase. Lexical tokens are enclosed in double quotes "" or back quotes ``.

The form `a … b` represents the set of characters from a through b as alternatives. The horizontal ellipsis `…` is also used elsewhere in the spec to informally denote various enumerations or code snippets that are not further specified. The character `…` (as opposed to the three characters `...`) is not a token of the Dodge language.

## Source Code Representation

Source code is Unicode text encoded in UTF-8. The text is not canonicalized, so a single accented code point is distinct from the same character constructed from combining an accent and a letter; those are treated as two code points. For simplicity, this document will use the unqualified term character to refer to a Unicode code point in the source text.

Each code point is distinct; for instance, uppercase and lowercase letters are different characters.

### Characters

The following terms are used to denote specific Unicode character categories:

```ebnf
newline        = /* the Unicode code point U+000A */ .
unicode_char   = /* an arbitrary Unicode code point except newline */ .
unicode_letter = /* a Unicode code point categorized as "Letter" */ .
unicode_digit  = /* a Unicode code point categorized as "Number, decimal digit" */ .
```

### Letters and Digits

The underscore character `_` (U+005F) is considered a lowercase letter.

```ebnf
letter        = unicode_letter | "_" .
hex_digit     = decimal_digit | "A" … "F" | "a" … "f" .
decimal_digit = "0" … "9" .
octal_digit   = "0" … "7" .
binary_digit  = "0" | "1" .
```

## Lexical elements

### Comments

Comments serve as program documentation. There are three forms:

1. Line comments start with the character sequence `//` and stop at the end of the line.
2. Block comments start with the character sequence `/*` and stop with the first subsequent character sequence `*/`.

A comment cannot start inside a rune or string literal, or inside a comment. A block comment containing no newlines acts like a space. Any other comment acts like a newline.

### Tokens

Tokens form the vocabulary of the Dodge language. There are four classes: *identifiers*, *keywords*, *operators* and *punctuation*, and *literals*. *White space*, formed from spaces (U+0020), horizontal tabs (U+0009), carriage returns (U+000D), and newlines (U+000A), is ignored except as it separates tokens that would otherwise combine into a single token. Also, a newline or end of file may trigger the insertion of a [semicolon](#semicolons). While breaking the input into tokens, the next token is the longest sequence of characters that form a valid token.

### Semicolons

The formal syntax uses semicolons `;` as terminators in a number of productions. Dodge programs may omit most of these semicolons using the following two rules:

1. When the input is broken into tokens, a semicolon is automatically inserted into the token stream immediately after a line's final token if that token is
   - an identifier
   - an number or string literal
   - one of the keywords break, continue, or return
   - a punctuation `)`, `]`, or `}`
2. To allow complex statements to occupy a single line, a semicolon may be omitted before a closing `)` or `}`.

To reflect idiomatic use, code examples in this document elide semicolons using these rules.

### Identifiers

Identifiers name program entities such as components and systems. An identifier is a sequence of one or more letters and digits. The first character in an identifier must be a letter.

```ebnf
identifier = letter { letter | unicode_digit } .
```

```text
a
_x9
SomeVar
αβ
```

Some identifiers are [predeclared](#predeclared-identifiers).

### Keywords

The following keywords are reserved and may not be used as identifiers.

```text
and
component
const
else
if 
import
is
map
mod
module
or
select
system
where
with
```

### Operators and punctuation

The following character sequences represent [operators](#operators) and punctuation:

```text
+ = ... ( )
- <     [ ]
* >     { }
/       , ;
        . :
```

### Number literals



### Predeclared identifiers
