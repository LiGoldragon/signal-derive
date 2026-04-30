# signal-derive

Schema-introspection proc-macros over [signal](https://github.com/LiGoldragon/signal)
record kinds.

```rust
use signal_derive::Schema;

#[derive(Schema, /* …other derives… */)]
pub struct Node {
    pub name: String,
}
```

This emits an `impl signal::Kind for Node` block whose
`DESCRIPTOR` const describes the type's shape — fields, field
types, enum variants — at compile time. Consumers walk the
descriptor without runtime cost.

See [ARCHITECTURE.md](ARCHITECTURE.md) for the role and
[mentci/reports/115](https://github.com/LiGoldragon/mentci/blob/main/reports/115-schema-derive-design-2026-04-30.md)
for the design.
