# STATUS — hwscan (archive snapshot)

## What this is

A small, self-contained Rust crate for **cross-platform hardware detection and tier calculation**. It detects CPU/GPU/RAM/disk characteristics and reduces them to a single "hardware tier" (1–5) suitable for guiding model selection, capability gating, or routing decisions.

The crate was part of a larger toolkit (`PersonalLog`/`Tripartite1`-era work) where every runtime needed to make a "what can this machine actually do?" decision before loading models. `hwscan` was the leaf module that answered that question.

## What's interesting here

- **Tier as a routing primitive.** The `tier()` function is the smallest useful abstraction here — a single integer that downstream code can switch on. Many projects reinvent this ad hoc; having a published crate with a stable tier scale means downstream code can be portable.
- **Cross-platform without abstraction leaks.** `cpu.rs`, `gpu.rs`, `platform.rs`, `disk.rs`, `tier.rs` are split per-subsystem rather than per-OS. Platform differences surface only in `platform.rs`. This is the "split by concept, not by platform" pattern that scales further than the more common `unix.rs` / `windows.rs` split.
- **Detection under a 500ms budget.** README claims sub-500ms detection. The fast-path implementation is worth a read even if you don't ship the crate — many "system info" libraries assume they can take seconds.
- **Five-example testbench.** `examples/basic.rs`, `integration.rs`, `json_output.rs`, `markdown_report.rs`, `tier_rec.rs` form a useful pattern: each example demonstrates a different *output mode* of the same underlying library. Makes the crate easy to evaluate without writing glue code.

## Why we moved on

We ended up writing tier detection in-app in our primary deployment rather than as a published crate — the host environment was already constrained (specific GPU families, specific RAM floors), so the "universal detection" surface was overkill. The crate stayed useful as a reference implementation but stopped earning its place in the dependency graph.

This doesn't mean the patterns are obsolete. The `tier()` abstraction and the "split by concept, not by platform" file layout are still good ideas. The reason to publish this is so that someone else building a "what can this machine run?" library can crib the structure without rediscovering it.

## Known caveats in this archive snapshot

- The initial commit included 1110 files of `target/` debug build artifacts and a `Cargo.lock`. These have been **untracked** in commit `0b60f28` (this archive's only modification to history) but the original commit object is preserved in the local repo. Anyone forking will see only the source tree.
- No CHANGELOG.md or `examples/README.md` describing what each example demonstrates.
- Documentation on docs.rs/crate.io will lag this snapshot until re-published upstream.

## License

Dual-licensed MIT OR Apache-2.0. Both `LICENSE-MIT` and `LICENSE-APACHE` are present in the repo.

## Date

Snapshot archived 2026-09-21 from local working copy at `C:\reseachlocal\claudeSuperInstance\hwscan`.