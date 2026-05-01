# Agent instructions — signal-derive

You **MUST** read AGENTS.md at `github:ligoldragon/lore` — the workspace contract.

## Repo role

A proc-macro crate emitting compile-time schema descriptors over [signal](https://github.com/LiGoldragon/signal) record kinds.

---

## Status — direction under review

**Do not extend this crate's role without a fresh decision.** Per the rejected frames in [criome/ARCHITECTURE.md §10.1](https://github.com/LiGoldragon/criome/blob/main/ARCHITECTURE.md) (`nexus-as-storage`, `sema-as-string-store`, `seed-from-nexus-files`), the originally-intended use of this crate's emitted consts as a "bootstrap projection source" feeding nexus seed files is **wrong**. Schema bootstrap is hand-coded in criome's init; domain kinds enter sema through Assert frames at runtime. The compile-time-baked-data shape this crate produces no longer has a clear consumer.

The crate's continuation, repurposing, or retirement is tracked under bd issue covering signal-derive direction. Its `KindDescriptor` const + `#[derive(Schema)]` mechanism still works; what's open is whether anyone should be using it.
