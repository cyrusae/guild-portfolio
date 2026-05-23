# Issue tracker example project

[Navigators Guild](https://github.com/Navigators-Guild/apprentice-onboarding/blob/main/02-the-methodology/02-tracking-your-work.md) issue tracking example project. Single per-project JSON file, deliberately minimalist, designed for paired human/AI use. Can be considered a conceptual sibling to `pbjson` and may someday be implemented more formally as such.

See `_docs`:

- [Annotated process document](_docs/PROCESS.md)
- [Initial design document](_docs/DESIGN.md) generated in concert with Claude Haiku
- [Initial list of test cases](_docs/TESTING.md) (no testing currently implemented)
- [Detailed plan for build](_docs/PLAN.md)

## File tree

```text
tracker/
├── _docs/
│   ├── DESIGN.md       # Specification and data model
│   ├── PLAN.md         # Atomic build steps used during development
│   ├── PROCESS.md      # Annotated development process
│   └── TESTING.md      # Test case inventory (not yet implemented)
├── src/
│   ├── main.rs         # Entry point and command dispatch
│   ├── cli.rs          # CLI interface definition (clap)
│   ├── data.rs         # Data model, enums, status derivation
│   ├── store.rs        # JSON file load/save
│   └── validation.rs   # Input validation for all user-supplied fields
├── Cargo.toml
└── README.md
```
