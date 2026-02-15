# Brass
This is a litle project of mine and an idea for a programming language that can transpile to other languages

## Problems
Different languages have very different types of syntax and ways to do things and accounting for them makes this very weird.
The problems lie in where different languages draw the line for abstraction
for example C doesnt have generics while Rust does, not allowing generics in brass or
de-abstracting generics in brass to make the C transpiler simpler would make the rust transpiler put out obfuscated code

## Targets brass hopes to support soon
C (No generics, compiled)
LLVM (very low level)
Rust (Generics, compiled)
Lua (Generics?, interpreted)
