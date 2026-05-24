## What I built

`entangle`--CLI tool for mirroring GitHub <-> Tangled.org repos easily, because I kept having to go through a multi-command process with a tab for the instructions open every time I wanted to do that.

## Build process

### Initial documentation 

Wrote DESIGN.md based on conversation iterating with Claude Sonnet. (It's a human-written one.) Split TESTING.md off into its own document once it filled the screen.

### Pre-flight check

Claude Code, new session, system-prompt `"."`. Started by asking for review of and feedback on the design docs. 

Things Claude identified:

1. Verifying that `entangle shove` is a convenience alias for `git push origin --all && git push origin --tags` as opposed to anything to scope creep on
2. Raised question of checking whether a private GitHub repo exists; I don't care much about private GitHub repo because Tangled has to be public so I assume public-public match, but worth considering whether it's possible to easily support the mismatch.
3. Question of whether `origin` not matching the generated `origin` "prompting an overwrite" fails out of `init` or what if it doesn't work--also worth discussing further
4. Suggested pinging GitHub for valid usernames as opposed to just regex matching on setup, which I'm interested in.
5. Crate setup: Claude suggested also adding `indicatif` for progress and `dialoguer` for interactivity
6. General questions: verified that `user.tngl.sh` really is what a username looks like (Claudes tend to have trouble with ATProto usernames).

Decided that the checking for valid names can be over internet and fall back on regex if there's an error that doesn't appear to be due to a hard existence error, and that "Okay, different fetch `origin`, that's your prerogative I guess" is valid.

Allowed Claude to edit the design/testing docs to reflect conversation. Prompted an explicit PLAN.md with integrated testing/documentation/validation questions throughout.

Then I remembered I was supposed to be using Chainlink and added that and had to re-start the session.

### Layer 1

Set up crate, set up stubs for all commands, set up module layout, smoke test, dependencies. Set up testing going forward.

### Layer 2

Added config loading and saving, added tests for config loading and saving.

### Layer 3

`validate.rs`--validate (regex, character limits, stripping invalid characters, lowercasing) inputs: Tangled usernames, GitHub usernames, repo names.

### Layer 4

`entangle set` non-interactive config file building and tests.

### Layer 5

`entangle setup` interactive counterpart using `dialoguer` for the prompts. Unit tests implemented; Claude tried to punt on the integration test due to not knowing how `dialoguer` would behave under testing but I caught it and nudged it to do the actual test.

`rexpect` was needed as an additional dev dependency for the test to work because `dialoguer` wants a real terminal for input. This was probably the most complex part thus far.

### Layer 6

Set up building URLs for Tangled and GitHub remotes.

### Layer 7

Set up remotes validation via gix (ls-remote equivalent). 

### Layer 8

Git init works, README detection hint works, gitignore detection works, you can start in a repository and have it detect whether the repository is initiated and contains basic structure.

### Layer 9

Really, *really* long turn--wish I could've found a good stopping point for it (maybe feature in one turn, then tests in the next turn?) but I'm not sure how I would've seen that coming enough to set it up. I don't think I've ever seen Claude take a turn this long, it ends up being over half an hour with compaction in the middle. So that's kind of concerning! It looks like the problem ended up being just (just) a newline conflict in `rexpect` with the fake terminal for testing.

*Finally* got to the `entangle init` command working for origin repo determination (the actual adding push URLs still hasn't happened yet).

Remembered that we needed to revisit the question of verbosity levels. My idea was quiet/verbose/debug levels (`debug` passing `gix` errors). Stopped to implement this instead of leaving it to post-MVP. `-q` and `--debug` verbosity levels now work.

### Layer 10

Pause to go "I think we're missing things from the original plan doc" because realized we've been moving past "decision point" checkpoints with Claude-determined defaults. Debated and decided against being overly cute in detecting other code forges because it resulted in too many branching paths for "GitHub when GitHub was expected but not the expected GitHub URL" versus "Tangled when GitHub was expected" versus... you get the picture.

Expected outputs are now wired up.

`entangle init` can notify the user when the origin isn't expected and either replace or keep it as-is. Stopped because I was out of API budget.

### Layer 11

Clarified with Claude that remote validation (risk of making a valid URL but not a valid remote) wasn't implemented fully (I am trying to get in the habit of "anything deferred or remaining decision points?" because it will eat them). 

`entangle init` adds push remotes. Fixed mirror vs canon-origin URLs being in the wrong order.

### Layer 12 

Clarify `entangle shove` reminder hint phrasing.

Add `entangle shove`--helper function for `git push origin --all && git push origin --tags` for repo setup. 

### Layer 13

Stopped computing paths manually and used `load` and `save` functions for the config. 

Decided to switch order from plan between "hardening" and "polish". Requested general hardening against the rest of the imaginable test cases (although we're already at over 200 tests so looking forward to seeing what even is left that we haven't hit en route?), then had to stop because I was out of API budget again.

Hardening caught a lack of testing for `entangle set` at the binary level.

### Layer 14

Basic CI setup. `clippy` compliance from "accumulated formatting oddities". Made a plan for CI/CD and crates.io release to implement later.

### Layer 15

General polish--`indicatif` spinners, `owo-colors`. 

Live test of terminal interface; feedback on help text.

### Layer 16

Assistance drafting README. "Walk me through a v1.0.0-alpha release." Got assistance with `Cargo.toml` and verified crates.io setup steps.

### Layer 17

Addressing first round of adversarial Gemini feedback from earlier today (additional test recommendations, concern about config brittleness, one concern about quotes handling I overruled). Things worth addressing: string mutation instead of using `gix` to edit git config if possible (set Claude to investigate whether possible), one case-sensitivity bug, one thing to document about baroque SSH setups. Most of the test cases Gemini called out already exist and many of the remainder aren't feasible to simulate.

Added tests for user CTRL+C'ing out at the last minute, corrupt git config, read-only config. Made handling of string matching of stderr outputs for not finding repos more reliable.

### Layer 18

Did another round of Gemini feedback against the feature-complete crate. Annotated feedback with my read of whether it was particularly valid and handed it back to Claude for evaluation.

Requested a move from the bespoke `atomic_write` function to using the `tempfile` crate; expanding the list of potential shell metacharacters; and basic config re-validation on load.

### Layer 19

Windows CI pipeline isn't working because `rexpect` isn't working on Windows; asked Claude to look into it. Verdict: `rexpect` only works on Unix, apparently? 

`cargo fmt` failing in CI pipeline due to a commenting disagreement, fixed.

### Layer 20

Preparing for `v0.1.0` release because I want to be able to install it for myself via Cargo, found another atomic write issue (for the `config.json` file this time). Fixed with tempfiles.

Cargo release successful, can now use on my machine having installed via cargo.

## What I learned

- Turn length can be unpredictable with this complicated a Rust build in a way I haven't run into before. (Possibly because I'm working with libraries that Claude doesn't know as well as Python?) 
- Difference between package/crates.io and binary names (`entangle` is taken but I don't have to rename the binary about it). 
- Even when it has a process document, "have you deferred anything and are there any remaining decision points?" is going to be necessary. 
- I should probably think through pacing and instruct Claude to commit its work instead of depending on my willingness to switch tabs. 
- AI-based CLI development continues to be easier than it feels like it has any right to be, but also the amount of setup needed to get to the point of "thing that could've been a Bash alias if I were being lazy for myself" was startling to see in practice (my poor, poor usage limit...). 
- Baby's first crates.io experience, Rust release methodology, `release-plz`.
- Use `clippy` and `fmt` more often/remind Claude to use them more often/set up CI structure early, drift happens. Practice setting up Rust CI structure should help.
- "Comments inside array/slice literals: rustfmt treats freestanding comment lines inside &[...] as trailing comments on the preceding item, shifting them rightward. If you want grouped comments above each group in a long const array, the only reliable option is to break the single const into multiple named consts and combine them, or accept the trailing-comment style."
- Atomic writes seem to be a weakness for Claude/something where it needs nudged repeatedly.
- This dance:

```text
So the full sequence in order:

  git checkout main
  git pull origin main
  git tag v0.1.0
  git push origin main --tags
  cd entangle
  cargo publish
  git checkout dev

  That way the tag, the main tip, and the published crate all point to the same commit.
```

## Known issues

- The overlap between restrictions on GitHub and Tangled repos isn't perfect and `entangle` enforces the maximally restrictive interpretation as opposed to special-casing "well, this repo is legal on GitHub but not Tangled, and it's only being applied as an alias to a GitHub repo, and...": some edge case `entangle init`s would fail if the original GitHub repo had a non-Tangled-compliant name (e.g., periods). 
- [x] Document this edge case explicitly in the README ✅ 2026-05-23
- By design: this only supports Tangled and GitHub. Other code forges would be nice (will probably leave an issue open for it if someone's got a burning desire, maybe?) but would require restructuring to have arbitrary preferences and mirrors.
- Per the above, I think it would be unfeasible to mirror more than once per `entangle` run--the most I could reasonably support would be alternate default and alternate mirror, not "arbitrary mirrors". I don't use anything but Tangled or GitHub so it's low priority and not a user flow I'm experienced with.
- [x] Document this explicitly in the repo ✅ 2026-05-23
- Core knowable issue: I don't speak Rust; I am deferring to the agent on best practices and line-by-line code to an even greater extent than I would be in a language I felt like I could read. 
- Commit history gotcha: if I were wise and brave and good I would have been committing after every step/layer, or had Claude do it; I was not wise and brave and good at this time. (I was also developing on `main`, which made sense in this case but should be called out as a bad habit to indulge and was stopped after a running alpha exists.)
- Little worried about whether having literally an `entangle/entangle/` directory is ever going to come back to haunt me. (Currently the README.md stub is annoying but fine.)
- Network errors versus auth errors are identified via string matching because that's what stderr gives me to work with, may be slightly brittle in non-English locales.
