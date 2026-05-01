# Architecture — signal-derive

Schema-introspection proc-macros over signal
record kinds. Designed in workspace/reports/115.

## Role

One responsibility: take a Rust type definition (struct or enum)
that participates in the signal record vocabulary, emit an
`impl signal::Kind for T` block describing the type's shape.

The shape lives in `signal::KindDescriptor`. The descriptor is a
compile-time constant; consumers (mentci-lib's `CompiledSchema`,
nexus-daemon's renderer when wired) walk it without runtime cost.

## Owns

- The single proc-macro derive: `#[derive(Schema)]`.
- The syn-tree walking that maps Rust field types to
  `FieldType` variants.

## Does not own

- The descriptor types (`KindDescriptor`, `FieldDescriptor`,
  `FieldType`, `KindShape`, the `Kind` trait, the `ALL_KINDS`
  catalogue) — those live in `signal`.
- Codec impls (`NotaRecord`, `NotaEnum`, etc.) — those are
  `nota-derive`'s concern. See lore/programming/abstractions.md §"The wrong-noun trap"
  for why.

## Code map

```
src/
├── lib.rs        proc-macro entry — dispatch by data shape
├── record.rs     struct → KindShape::Record codegen
├── enum_kind.rs  enum → KindShape::Enum codegen
└── field_type.rs syn::Type → FieldType lowering (the
                 mechanical mapping primitives + Slot<T> +
                 Vec<T> + Option<T> + nested type)
```

(Skeleton-as-design — file layout sketched; bodies filled in
incrementally as kinds onboard.)

## Cross-cutting context

- criome/ARCHITECTURE.md
  for the engine that consumes signal records.
- workspace/reports/115
  for the design.

## Status

Skeleton-as-design. The derive macro is wired for structs with
named fields and unit-variant enums, supporting the field-type
mapping in §1 of the design report. Coverage extends as more
signal record kinds onboard.
