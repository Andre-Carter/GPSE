FOOTER NOTES 

CURRENT INVESTIGATION
---------------------
Legacy source structure is being characterized before refactoring.

CONFIRMED
---------
- `gpse` is both a library and binary crate.
- `lib.rs` exposes `chemical` and `physical`.
- `physical::constants` contains a large canonical constant dataset.
- `chemical/elements.rs` contains an Element model and periodic-element data.
- `chemical/mod.rs` does not currently expose `elements`.
- `main.rs` independently declares `chemical` and `physical`.
- `main.rs` also directly consumes the library crate (`gpse::...`).
- The executable therefore appears to compile a duplicate local module
  tree alongside the library.

WARNING INVESTIGATION
---------------------
The duplicate module declarations are a strong candidate for at least
some of the large warning count observed during cargo check/clippy.

NOT YET DECIDED
---------------
- Whether `main.rs` should remove its local module declarations.
- Exact public API for chemical data.
- Whether `elements` should be directly exported from `chemical`.
- Final organization of physical constants.
- Constant metadata / symbol / alias representation.

NEXT INVESTIGATION
------------------
Characterize the actual compiler/clippy warnings and determine which
warnings originate from duplicate binary module compilation versus
intentional unused canonical data.






EXPERIMENT: Canonical Entity Architecture

Purpose:
Determine whether a small immutable entity-definition layer provides
a useful boundary between canonical physical data and simulation state.

Initial entities:
- Sun
- Earth
- Moon

Constraints:
- No simulation runtime.
- No ECS.
- No orbital mechanics.
- No mutable global state.
- No premature abstraction.
- Reuse existing physical constants where appropriate.

Success criteria:
- Clear ownership of canonical data.
- Simple imports.
- Deterministic values.
- Independently testable.
- No unnecessary coupling.
- Architecture remains easy to replace if the experiment fails.

OUR SACRED WORK FLOW

Edit
  ↓
Inspect
  ↓
cargo fmt
  ↓
cargo check/test/clippy
  ↓
git diff
  ↓
git status
  ↓
YOU decide what gets staged
  ↓
YOU approve the commit
  ↓
push