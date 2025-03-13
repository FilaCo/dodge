# The Dodge Programming Language Specification

Language edition 2025

## Table of contents

<!-- TOC -->

* [The Dodge Programming Language Specification](#the-dodge-programming-language-specification)
    * [Table of contents](#table-of-contents)
    * [Intro](#intro)
    * [Notation](#notation)
    * [Source Code Representation](#source-code-representation)
        * [Characters](#characters)
        * [Letters and Digits](#letters-and-digits)
    * [Lexical elements](#lexical-elements)
        * [Comments](#comments)
        * [Tokens](#tokens)
        * [Semicolons](#semicolons)
        * [Identifiers and keywords](#identifiers-and-keywords)
        * [Operators and punctuation](#operators-and-punctuation)
        * [Integer literals](#integer-literals)
        * [Floating-point literals](#floating-point-literals)
        * [Rune literals](#rune-literals)
        * [String literals](#string-literals)
    * [Constants](#constants)

<!-- TOC -->

## Intro

This is the reference manual for the Dodge programming language.

Dodge is a domain specific language (**DSL**) for game development designed with Entity Component System (**ECS**)
pattern and data oriented design (**DOD**) in mind. It is *strongly* typed. Programs are constructed from
modules, whose properties allow efficient management of dependencies.

The syntax is compact and simple allowing for easy analysis by non-programmers, such as game designers.

## Notation

The syntax is specified using a [variant](https://en.wikipedia.org/wiki/Wirth_syntax_notation) of Extended Backus-Naur
Form (EBNF):

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

Lowercase production names are used to identify lexical (terminal) tokens. Non-terminals are in CamelCase. Lexical
tokens are enclosed in double quotes "" or back quotes ``.

The form `a … b` represents the set of characters from a through b as alternatives. The horizontal ellipsis `…` is also
used elsewhere in the spec to informally denote various enumerations or code snippets that are not further specified.
The character `…` (as opposed to the three characters `...`) is not a token of the Dodge language.

## Source Code Representation

Source code is Unicode text encoded in UTF-8. The text is not canonicalized, so a single accented code point is distinct
from the same character constructed from combining an accent and a letter; those are treated as two code points. For
simplicity, this document will use the unqualified term character to refer to a Unicode code point in the source text.

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

A comment cannot start inside a rune or string literal, or inside a comment. A block comment containing no newlines acts
like a space. Any other comment acts like a newline.

Block comments can be recursive, so a sequence like `/* /* */` will not be considered terminated and will result in a
parsing error.

### Tokens

Tokens form the vocabulary of the Dodge language. There are three classes: *identifiers* and *keywords*, *operators* and
*punctuation*, and *literals*.

The following terms are used to denote *whitespace* token class:

```ebnf
whitespace          = whitespace_char { whitespace_char } .
whitespace_char     = horisontal_tab | newline | vertical_tab | form_feed | carriage_return | space | latin1_nextline | 
                      ltr_mark | rtl_mark | line_separator | paragraph_separator .
horisontal_tab      = /* the Unicode code point U+000A */ .
vertical_tab        = /* the Unicode code point U+000B */ .
form_feed           = /* the Unicode code point U+000C */ .
carriage_return     = /* the Unicode code point U+000D */ .
space               = /* the Unicode code point U+0020 */ .
latin1_nextline     = /* the Unicode code point U+0085 */ .
ltr_mark            = /* the Unicode code point U+200E */ .
rtl_mark            = /* the Unicode code point U+200F */ .
line_separator      = /* the Unicode code point U+2028 */ .
paragraph_separator = /* the Unicode code point U+2029 */ .
```

*whitespace* token is ignored except as it separates tokens that would otherwise combine into a single token.
Also, a newline or end of file may trigger the insertion of a [semicolon](#semicolons). While breaking the input
into tokens, the next token is the longest sequence of characters that form a valid token.

### Semicolons

The formal syntax uses semicolons `;` as terminators in a number of productions. Dodge programs may omit most of these
semicolons using the following two rules:

1. When the input is broken into tokens, a semicolon is automatically inserted into the token stream immediately after a
   line's final token if that token is
    - an identifier or keywords break, continue, or return
    - an integer, floating-point, imaginary, rune, or string literal
    - a punctuation `)`, `]`, or `}`
2. To allow complex statements to occupy a single line, a semicolon may be omitted before a closing `)` or `}`.

To reflect idiomatic use, code examples in this document elide semicolons using these rules.

### Identifiers and keywords

Identifiers name program entities such as components and systems. An identifier is a sequence of one or more letters and
digits. The first character in an identifier must be a letter.

```ebnf
identifier = letter { letter | unicode_digit } .
```

```text
a
_x9
SomeVar
αβ
```

Some identifiers are reserved *keywords* and may not be used as a name of the Dodge language entity.

```text
and         import  query
component   is      select
const       map     system
else        mod     where
if          or      with
```

To provide backward compatibility, the Dodge language declares a *raw identifier* feature. It is a regular identifier or
keyword wrapped with back quotes. This feature allows developers to use keywords as a name of the Dodge language
entities.

```ebnf
raw_identifier = "`" identifier "`" .
```

### Operators and punctuation

The following character sequences represent operators and punctuation:

```text
+    &     +=    &=     &&    ==    !=    (    )
-    |     -=    |=     ||    <     <=    [    ]
*    ^     *=    ^=     ++    >     >=    {    }
/    <<    /=    <<=    --    =     ...   ,    ;
%    >>    %=    >>=          !           .    :
     &^          &^=          ~
```

### Integer literals

An integer literal is a sequence of digits representing an integer constant. An optional prefix sets a non-decimal base:
`0b` or `0B` for binary, `0o` or `0O` for octal, and `0x` or `0X` for hexadecimal. A single 0 is considered a decimal
zero. Unnecessary leading zeroes are forbidden. In hexadecimal literals, letters a through f and A through F represent
values 10 through 15.

An optional suffix sets a type of integer literal: i8, u8, i16, u16, i32, u32, i64, u64, isize, usize. If the suffix
is absent, then a concrete type of the literal will be chosen from the context. If it is not possible, then a compile
error should be emitted.

For readability, an underscore character `_` may appear after a base prefix, before a type suffix or between successive
digits; such underscores do not change the literal's value.

```ebnf
int_lit        = ( dec_lit | bin_lit | oct_lit | hex_lit ) [ [ "_" ] int_lit_suffix ] .

int_lit_suffix = ( "i" | "u" ) ( "8" | "16" | "32" | "64" | "size" ) .

decimal_lit    = "0" | ( "1" … "9" ) [ [ "_" ] decimal_digits ] .
binary_lit     = "0" ( "b" | "B" ) [ "_" ] binary_digits .
octal_lit      = "0" [ "o" | "O" ] [ "_" ] octal_digits .
hex_lit        = "0" ( "x" | "X" ) [ "_" ] hex_digits .

decimal_digits = decimal_digit { [ "_" ] decimal_digit } .
binary_digits  = binary_digit { [ "_" ] binary_digit } .
octal_digits   = octal_digit { [ "_" ] octal_digit } .
hex_digits     = hex_digit { [ "_" ] hex_digit } .
```

```text
42
4_2
0o600
0O600       // second character is a capital letter 'O'
0xBadFace
0xBad_Face
0x_67_7a_2f_cc_40_c6
170141183460469231731687303715884105727
170_141183_460469_231731_687303_715884_105727
123u8
100500_i64

_42         // an identifier, not an integer literal
0600        // invalid: two literals Int(0) and Int(600)
0_600       // invalid: Int(0), Ident(_) and Int(600) parsed
42_         // invalid: _ must separate successive digits
4__2        // invalid: only one _ at a time
0_xBadFace  // invalid: _ must separate successive digits
256u8       // invalid: overflow error
```

### Floating-point literals

A floating-point literal is a decimal or hexadecimal representation of a [floating-point constant].

A decimal floating-point literal consists of an integer part (decimal digits), a decimal point, a fractional part (
decimal digits), and an exponent part (e or E followed by an optional sign and decimal digits). One of the integer part
or the fractional part may be elided; one of the decimal point or the exponent part may be elided. An exponent value exp
scales the mantissa (integer and fractional part) by 10<sup>exp</sup>.

A hexadecimal floating-point literal consists of a 0x or 0X prefix, an integer part (hexadecimal digits), a radix point,
a fractional part (hexadecimal digits), and an exponent part (p or P followed by an optional sign and decimal digits).
One of the integer part or the fractional part may be elided; the radix point may be elided as well, but the exponent
part is required. (This syntax matches the one given in IEEE 754-2008 §5.12.3.) An exponent value exp scales the
mantissa (integer and fractional part) by 2<sup>exp</sup.

An optional suffix sets a type of floating-point literal: f32 or f64. If the suffix is absent, then a concrete type of
the literal will be chosen from the context. If it is not possible, then a compile error should be emitted.

For readability, an underscore character `_` may appear after a base prefix, before a type suffix or between successive
digits; such underscores do not change the literal value.

```ebnf
float_lit         = ( decimal_float_lit | hex_float_lit ) [ [ "_" ] float_lit_suffix ] .

float_lit_suffix  = "f32" | "f64" .

decimal_float_lit = decimal_digits "." [ decimal_digits ] [ decimal_exponent ] |
                    decimal_digits decimal_exponent |
                    "." decimal_digits [ decimal_exponent ] .
decimal_exponent  = ( "e" | "E" ) [ "+" | "-" ] decimal_digits .

hex_float_lit     = "0" ( "x" | "X" ) hex_mantissa hex_exponent .
hex_mantissa      = [ "_" ] hex_digits "." [ hex_digits ] |
                    [ "_" ] hex_digits |
                    "." hex_digits .
hex_exponent      = ( "p" | "P" ) [ "+" | "-" ] decimal_digits .
```

```text
0.
72.40f64
072.40       // == 72.40
2.71828_f32
1.e+0
6.67428e-11
1E6
.25
.12345E+5
1_5.         // == 15.0
0.15e+0_2    // == 15.0

0x1p-2       // == 0.25
0x2.p10      // == 2048.0
0x1.Fp+0     // == 1.9375
0X.8p-0      // == 0.5
0X_1FFFP-16  // == 0.1249847412109375
0x15e-2      // == 0x15e - 2 (integer subtraction)

0x.p1        // invalid: mantissa has no digits
1p-2         // invalid: p exponent requires hexadecimal mantissa
0x1.5e-2     // invalid: hexadecimal mantissa requires p exponent
1_.5         // invalid: _ must separate successive digits
1._5         // invalid: _ must separate successive digits
1.5_e1       // invalid: _ must separate successive digits
1.5e_1       // invalid: _ must separate successive digits
1.5e1_       // invalid: _ must separate successive digits
```

### Rune literals

A rune literal represents a [rune constant], an integer value identifying a Unicode code point. A rune literal is
expressed as one or more characters enclosed in single quotes, as in `'x'` or `'\n'`. Within the quotes, any character
may appear except newline and unescaped single quote. A single quoted character represents the Unicode value of the
character itself, while multi-character sequences beginning with a backslash encode values in various formats.

The simplest form represents the single character within the quotes; since Go source text is Unicode characters encoded
in UTF-8, multiple UTF-8-encoded bytes may represent a single integer value. For instance, the literal `'a'` holds a
single byte representing a literal a, Unicode U+0061, value 0x61, while 'ä' holds two bytes (0xc3 0xa4) representing a
literal a-dieresis, U+00E4, value 0xe4.

Several backslash escapes allow arbitrary values to be encoded as ASCII text. There are four ways to represent the
integer value as a numeric constant: \x followed by exactly two hexadecimal digits; \u followed by exactly four
hexadecimal digits; \U followed by exactly eight hexadecimal digits, and a plain backslash \ followed by exactly three
octal digits. In each case the value of the literal is the value represented by the digits in the corresponding base.

Although these representations all result in an integer, they have different valid ranges. Octal escapes must represent
a value between 0 and 255 inclusive. Hexadecimal escapes satisfy this condition by construction. The escapes \u and \U
represent Unicode code points so within them some values are illegal, in particular those above 0x10FFFF and surrogate
halves.

After a backslash, certain single-character escapes represent special values:

```text
\a   U+0007 alert or bell
\b   U+0008 backspace
\f   U+000C form feed
\n   U+000A line feed or newline
\r   U+000D carriage return
\t   U+0009 horizontal tab
\v   U+000B vertical tab
\\   U+005C backslash
\'   U+0027 single quote  (valid escape only within rune literals)
\"   U+0022 double quote  (valid escape only within string literals)
```

An unrecognized character following a backslash in a rune literal is illegal.

```ebnf
rune_lit         = "'" ( unicode_value | byte_value ) "'" .
unicode_value    = unicode_char | little_u_value | big_u_value | escaped_char .
byte_value       = octal_byte_value | hex_byte_value .
octal_byte_value = `\` octal_digit octal_digit octal_digit .
hex_byte_value   = `\` "x" hex_digit hex_digit .
little_u_value   = `\` "u" hex_digit hex_digit hex_digit hex_digit .
big_u_value      = `\` "U" hex_digit hex_digit hex_digit hex_digit
                           hex_digit hex_digit hex_digit hex_digit .
escaped_char     = `\` ( "a" | "b" | "f" | "n" | "r" | "t" | "v" | `\` | "'" | `"` ) .
```

```text
'a'
'ä'
'🤡'
'\t'
'\000'
'\007'
'\377'
'\x07'
'\xff'
'\u12e4'
'\U00101234'
'\''         // rune literal containing single quote character
'aa'         // illegal: too many characters
'\k'         // illegal: k is not recognized after a backslash
'\xa'        // illegal: too few hexadecimal digits
'\0'         // illegal: too few octal digits
'\400'       // illegal: octal value over 255
'\uDFFF'     // illegal: surrogate half
'\U00110000' // illegal: invalid Unicode code point
```

### String literals

A string literal represents a string constant obtained from concatenating a sequence of characters. There are two forms:
raw string literals and interpreted string literals.

Raw string literals begins with one or more `#` characters and a quote, and is followed by zero or more valid
characters, up until a quote and the same number of `#` characters as began the string. If this terminator is not
matched before the end of current file, a compile error is emitted. Escaping rules do not apply in raw string literals;
specifically, the `\` character has no special meaning and is taken to be a literal backslash character.

Interpreted string literals are character sequences between double quotes, as in `"bar"`. Within the quotes, any
character may appear except newline and unescaped double quote. The text between the quotes forms the value of the
literal, with backslash escapes interpreted as they are in rune literals (except that `\'` is illegal and `\"` is
legal), with the same restrictions. The three-digit octal (\nnn) and two-digit hexadecimal (\xnn) escapes represent
individual bytes of the resulting string; all other escapes represent the (possibly multibyte) UTF-8 encoding of
individual characters. Thus, inside a string literal `\377` and `\xFF` represent a single byte of value `0xFF=255`,
while `ÿ`, `\u00FF`, `\U000000FF` and `\xc3\xbf` represent the two bytes `0xc3 0xbf` of the UTF-8 encoding of character
U+00FF.

```ebnf
string_lit             = raw_string_lit | interpreted_string_lit .
interpreted_string_lit = `"` { unicode_value | byte_value } `"` .
raw_string_lit         = ( "#" raw_string_lit "#" ) | ( "#" `"` { unicode_char | newline } `"` "#" ) .
```

```text
#"abc"#              // same as "abc"
#"\n
\n"#                 // same as "\\n\n\\n"
"\n"
"\""                 // same as #"""#
"Hello, world!\n"
"日本語"
"\u65e5本\U00008a9e"
"\xff\u00FF"
"\uD800"             // illegal: surrogate half
"\U00110000"         // illegal: invalid Unicode code point
```

These examples all represent the same string:

```text
"日本語"                                   // UTF-8 input text
#"日本語"#                                 // UTF-8 input text as a raw literal
"\u65e5\u672c\u8a9e"                    // the explicit Unicode code points
"\U000065e5\U0000672c\U00008a9e"        // the explicit Unicode code points
"\xe6\x97\xa5\xe6\x9c\xac\xe8\xaa\x9e"  // the explicit UTF-8 bytes
```

If the source code represents a character as two code points, such as a combining form involving an accent and a letter,
the result will be an error if placed in a rune literal (it is not a single code point), and will appear as two code
points if placed in a string literal.

## Constants

TBD
