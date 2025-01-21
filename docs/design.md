# The Dodge Programming Language Design Document

## Design goals

1. Simplicity
2. Convenient for writing game/simulation logic
3. Performance
4. Modular
5. Adhere to ECS and DOD paradigms

## Non-design goals

1. General purpose language

## Functional Requirements

1. Dodge has to provide a way to express game/simulation logic in a ECS notation.
2. Dodge has to provide a way to communicate with external systems in a way, natural for ECS.

## Non-functional Requirements

1. Compiled
2. Static typing
3. Strict typing
4. Compiler is written in Rust to LLVM
