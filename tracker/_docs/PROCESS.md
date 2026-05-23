# Process

## What I built

Rust-based simple issue tracker, command-line interface, usable by humans and agents. Generates a single running `.json` file per project. Intended to be extremely lightweight proof of concept. Developed for the issue-tracking assignment in the Navigators Guild curriculum and as a thought experiment.

## Build process

### Initial discussion

Developed in conversation with Claude Haiku.

Initial prompt:

> Clone this repo and read the README and the section on the issue tracker project. https://github.com/Navigators-Guild/apprentice-onboarding I’m going to work on the issue tracker CLI assignment tomorrow and I’d like to brainstorm it with you: it’s a toy example project, but I’d like help thinking of ways to make it mine/put my own spin on it so it’s still fun to do.

Second round:

> Maybe… the data modeling aspect of conceptualizing “issues” as something composable that can block each other and feed back into decision making? I don’t want to pretend I’m inventing something usable as opposed to reinventing chainlink or deciduous, but “what is an issue” draws my attention. There’s also the radical simplicity option, like making it a partner to pbjson: have project state live in a single json or jsonl file that’s portable enough to upload to Claude.ai as a snapshot of development being done on desktop.

Further discussion pinpointed the data model and polished the "issues snapshot counterpart to [pbjson](https://github.com/cyrusae/pbjson)" concept enough to make it feel like something I could pretend was usable enough to develop. (I later revisited the chapter and realized I was reinventing crosslink again.)

The pbjson comparison led to "is it ergonomic for agents":

> Help me think through the interface? I’m especially interested in making sure it’s ergonomic for agents— natural language conversation with me surfaces “I want to add a help flag but I’m stuck on drafting an official-looking man page and knowing how to invoke it”, Claude logs the issue “add help flag” and the contributing info I just gave.

Haiku generated the first draft of the [DESIGN.md](DESIGN.md) document based on the conversation after a couple rounds of clarifying questions about my workflow and me remembering that priority and labels were part of the assignment, and suggested the test cases in [TESTING.md](TESTING.md).

I preferred Haiku for the following reasons:

1. More affordable to clone entire repo for assignment context (GitHub robots.txt blocks me just linking the chapter)
2. Smaller model is somewhat less likely to make assumptions or sweeping decisions on its own
3. I try to stay in the habit of using smaller models generally

### Development

On-desktop development switched to Claude Sonnet in Claude Code. If I did further testing I would have supplemented with Gemini for cross-model testing/ease of getting a clean session immediately; for a larger project I would also have supplemented with code from other models (usually Gemini).

#### Pre-flight

Started with a clean Claude Code prompt and had it read the existing documents. Instructed it to read through the existing documents for context and suggest questions/changes pre-flight. It highlighted the following: `blockedBy` field versus `blocked` timeline event; question of whether issues should be reopen-able or not; AND versus OR tag filtering.

Decided:

1. Separate `blockedBy` and `stuck` states: blocked is a thing that happens from issues in relation to each other, stuck is a status within an issue. Refactoring documentation accordingly.
2. Made issues permanently close; if an issue comes up again it gets to be a new issue (ticky box dopamine).
3. AND tag filtering.

Claude updated DESIGN.md accordingly. Had it create PLAN.md with individual steps based on design doc.

#### Core features

Single pass: data model, CLI skeleton, core creation/deletion of events, status command, show command, delete command. Started working on priority. Creating with label works, filtering next.

#### Layer 1

Added label command features: lowercase labels with deduplication, final labels sort alphabetically.

#### Layer 2

Added label search (AND). Suggested going in larger steps because I had suggested small steps initially and Claude was Claude-typical willing to give me what I asked for.

#### Layer 3

Added "blocking" between issues. Claude noticed that ignoring blocks from nonexistent issues wasn't working exactly as described and fixed it (blocked status was still being added despite the spec saying it shouldn't be).

#### Layer 4


## What I learned

This was my first experience working in Rust. I should have thought about this, but it's significantly easier to get Claude to write and test a CLI tool than a GUI one because it can do all of its own testing without me needing to manually hammer out synthetic data. 

## Known issues

- I framed it as a sibling project to `pbjson` but it hasn't been deliberately designed to work in concert with it; if I were developing it as more than a one-day assignment I'd probably be trying to make them actively cross-compatible/more explicitly modeling it on `pbjson`, but in that case it would probably also be in the "bootleg skill" Python format.
- Individual commands aren't documented, availability of `help` is limited.
