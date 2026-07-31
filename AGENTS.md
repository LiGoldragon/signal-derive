# Agent instructions — signal-derive

## Repo role

A proc-macro crate emitting compile-time schema descriptors over signal record kinds.

## Status

The crate's role is open: schema bootstrap is hand-coded in criome's init, and domain kinds enter sema through Assert frames at runtime, so the original consumer for these compile-time descriptors no longer applies. Tracked under bd issue covering signal-derive direction. The `KindDescriptor` const + `#[derive(Schema)]` mechanism remains in place pending decision.

## Protos estate status

Stack: correct-new destination
Status: component-associated, adoption unresolved
Absence of a direct notation edge is not proof of adoption.
