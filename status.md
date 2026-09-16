# Current State

# Confirmed Findings

# Active Investigation

# Active Experiments

# Recent Decisions

# Next Actions

# Validation Results 
// for future outline not to worry. Praise Klang

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

Canonical scientific data can live independently.
Entities can provide physical properties.
Equations can operate on generic numerical inputs.
Callers compose the pieces.
Equations return values rather than performing I/O.
We have an unresolved units problem.
We have an unresolved invalid-input problem (distance = 0).
We haven't yet dealt with changing state/time.


