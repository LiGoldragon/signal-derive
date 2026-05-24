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

## Macro-pattern integration

**Status:** integrated into the brilliant macro library pattern per `reports/designer/326-v13-spirit-complete-schema-vision.md §3` (schemas as macro-pattern instance) — orphan candidate for retirement.

**Role:** this crate emits `impl signal::Kind for T` blocks for Rust types that participate in the signal record vocabulary — a compile-time descriptor (`signal::KindDescriptor`) walked by consumers like mentci-lib's `CompiledSchema`.

**Integration target:** Schema derive — orphan per `reports/designer/317-2`; may retire when macro pattern fully lands. The schema-engine upgrade subsumes this derive's purpose: a `.schema` file is the authoritative description of every record kind, and the brilliant macro library emits both the Rust types AND any descriptor structure consumers need from it. Once `primary-ezqx.1` MVP succeeds and downstream consumers cut over to schema-driven descriptors, this crate retires.

**Per-library concern:** orphan. Do not extend this crate; new derives that would otherwise land here should land as schema-engine machinery instead. Carries no current consumer pulling the descriptor from this derive (the previously-planned mentci-lib path did not land).

**References:**
- `reports/designer/326-v13-spirit-complete-schema-vision.md` — schema language + macro pattern
- `reports/designer/324-migration-mvp-spirit-handover-re-specification.md` — migration MVP
- `reports/operator/174-schema-import-header-design-critique-2026-05-24.md` — lowering + AssembledSchema form
- `reports/designer/317-2-…` — orphan-status reasoning
