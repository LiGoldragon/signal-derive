# signal-derive — agent notes

Read [criome's `ARCHITECTURE.md`](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md)
and [mentci's `INTENTION.md`](https://github.com/LiGoldragon/mentci/blob/main/INTENTION.md)
before editing this crate. Cross-project rules — full English
words, methods on types, no stop-gaps — live in
[mentci's `AGENTS.md`](https://github.com/LiGoldragon/mentci/blob/main/AGENTS.md)
and [tools-documentation](https://github.com/LiGoldragon/tools-documentation).

## What this crate is

The proc-macro crate that emits schema descriptors over
[`signal`](https://github.com/LiGoldragon/signal) record kinds.
Designed in [mentci/reports/115](https://github.com/LiGoldragon/mentci/blob/main/reports/115-schema-derive-design-2026-04-30.md).

The single derive: `#[derive(Schema)]` on a struct or enum in
signal. The macro emits an `impl signal::Kind for T` block whose
`DESCRIPTOR` const describes the type's shape — name, fields,
field-types, enum variants. mentci-lib's `CompiledSchema` walks
that descriptor; nexus-daemon's renderer eventually does the same.

## What this crate is not

- **Not the schema types' home.** `KindDescriptor`,
  `FieldDescriptor`, `FieldType`, `Kind` all live in `signal`.
  `signal-derive` only emits code referencing them.
- **Not a codec emitter.** `nota-derive` owns codec impls
  (`NotaRecord`, `NotaEnum`, `NotaTransparent`, `NexusVerb`,
  `NexusPattern`). `signal-derive` is the schema-introspection
  derive sitting next to those, not folded into them — adjacency
  of types is not adjacency of concerns. See
  [tools-documentation/programming/abstractions.md §"The wrong-noun trap"](https://github.com/LiGoldragon/tools-documentation/blob/main/programming/abstractions.md).

## Scope today

`#[derive(Schema)]` on:

- structs with named fields (records like `Node`, `Edge`,
  `Graph`, `Principal`, `Theme`, `Layout`, `Tweaks`, …)
- enums with unit variants only (closed vocabularies like
  `RelationKind`, `IntentToken`, `GlyphToken`, `StrokeToken`,
  `ActionToken`, `SizeIntent`)

Field types the macro recognises:

- primitives: `String` → Text, `bool`, integer types, float types
- `Slot<Kind>` → SlotRef referencing the named kind
- `Vec<T>` → list of T
- `Option<T>` → optional T
- nested user types → Record { kind_name } (resolution happens
  at consumer side via `ALL_KINDS`)

## See also

- [signal](https://github.com/LiGoldragon/signal) — host of the
  types this derive emits descriptors against
- [mentci/reports/115](https://github.com/LiGoldragon/mentci/blob/main/reports/115-schema-derive-design-2026-04-30.md)
  — design report
