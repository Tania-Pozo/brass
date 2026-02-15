# Todo
[ ] Actual transpiler
[ ] 'known' values
[ ] 'const' values
[ ] *always true* values
    example:
```brass
    prod NonZeronum(@num) where (self.0 != 0);
```
[ ] IR optimization `Option<NonZeroNum> == @num` (Because `Option::None` would be eq to `@num::ZERO`)
[ ] Testing
[ ] Generics
[ ] tree-sitter grammar


