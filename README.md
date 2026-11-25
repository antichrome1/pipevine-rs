# `pipevine`

This crate provides a procmacro to pipeline statements with a
placeholder to avoid reading a deeply nested statement from inside
out.

```rust
let somevar = 5;
let result = pipevine!(somevar,
                       add(1, <>),
                       mul(<>, 3),
                       double,
                       add(10, mul(<>, 2)),
                       |x| x + 4,
                       <> + <> - 2,
                       if <> % 2 == 0 { <> / 2 } else { <> * 3 + 1 },
                       negate,
);
print!("{result}"); // -85
```

## License

ISC © 2026 Aditya Singh <antichrome1@proton.me>
