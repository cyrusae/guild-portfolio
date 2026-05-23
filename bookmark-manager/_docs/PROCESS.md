# Process

Document:

**What you built and why.** A paragraph summarizing the project.

**Your build process.** Walk through the layers you built, in order. For each layer, note:

- What you asked the agent to do
- Whether it worked on the first try
- What you had to fix or adjust
- Any verification failures and how you resolved them

**What you learned.** What surprised you? What was harder than expected? What would you do differently next time?

**Known issues.** Is there anything that doesn't work perfectly? Anything you'd want to improve? Being honest about known issues shows maturity and self-awareness, both things the guild values.

---

## What I built

Single-page bookmark manager with duplicate detection, notes/tags, search, editing, and deletion. Built as the first required project for the Navigators Guild and to experiment further with cyberpunk Catppuccin themes.

**Note:** I am aware I didn't exactly follow the instructions. This is because the instructions were going to drive me insane. I did my best to artificially inject the layering as opposed to including it in the design document naturally, which was difficult, but I have strong preferences about readable HTML/JS projects (namely, "every line of code should live in a file defined enough that the IDE knows how to highlight it"), Claude knows it, and I'm going to have to ask you to live with that.

## Design doc and process

I used Claude Haiku for brainstorming and rewrote the design doc it generated to attempt to conform to the project guidelines, including forcing the layered iterations. I did log the layer requirements in advance because I have limits. I broke my specifications into `DESIGN.md` and `TESTING.md` to separate out validation/edge case suggestions from the main document. The edge cases were suggested by Haiku.

Development stage started with a fresh Claude Code instance (Sonnet, `--system-prompt="."`) in a folder with the DESIGN and TESTING documentation.

### Layers

"What I asked the agent to do" is available verbatim in `DESIGN.md`.

#### Core stage

> **Requested:** Core features as defined in the spec (bookmarks displayed, can be added, persist); initial enhancements (URLs validated and trimmed, basic sanitization, confirmation toast when a bookmark is added successfully).

Approved of the base architecture Claude proposed based on my style guidelines. Implemented adding and displaying bookmarks (title + url only) with basic sanity-checking (no empty titles, no empty URLs, URLs must start `http` or `https`).

Noticed that the URL trimming for trackers etc. wasn't how I would have implemented it but wanted to see how Claude's implementation shook out.

#### Addition #1

> **Requested:** Added optional note and tags fields to the bookmark form. 

Verified that saving bookmarks without tags and/or notes was still functional. Verified that `tag, ` correctly cut off to just `tag`.

Notes didn't immediately support newlines/multi-paragraph notes due to needing a CSS fix. Claude was responsive when I noted this. Fix worked immediately (didn't even require reloading the page, that was nice).

#### Addition #2

> **Requested:** Edit and delete functions, with icons.

Edit and delete functions worked as intended (editing saves correctly, delete has a sanity check, icons are unobtrusive).

#### Addition #3

> **Requested:** All existing tags are displayed above the bookmark list; clicking a bookmark filters it.

Adding tags bar worked as intended. Noted for future reference that it was going to get unwieldy with more than like five tags but that's out of scope for the project as written. Confirmed that tag filtering worked. Decided that clicking tags should be an `AND` sort as opposed to `OR` but didn't implement it yet.

#### Addition #4

> **Requested:** Search bar next to tag filters.

Search bar implemented. Confirmed that it searches notes and titles but not tags (intended behavior) and that it searches within a tag when the tag is selected.

Noted that scrolling took the header off the top of the page. Asked for header to stick to the top of the page and contents to scroll under it. Stickied header accordingly. Debating whether to do the same to tags/search or if that would be too clunky; deferring this to after adding visual styling.

#### Enhancements

> **Requests:** Duplicate detection on bookmarks; added automatic URL prefixing and filling in URLs in the absence of a title; multi-tag select.

Verified that multi-tag select worked (I asked for AND select). Added duplicate detection and support for URLs missing their prefix.

#### Visual styling

Added a screenshot of an interface I like for additional styling guidance and adjusted coloring to be "Catppuccin Mocha but with darker blacks". First draft was pretty good. Requested refinements: changes to spacing, make URLs and tags be all caps for style reasons. Changed sizing of notes and URL fields to make the titles more distinctive. 

Requested adding clickability to tags on bookmark cards as well as in the header and effects on mouseover. Removed bar on the side of the notes indent. Made edit/delete buttons visible only on mouseover.

Raised issue of mouseover-dependent buttons being questionable on mobile and suggested highlighting via tap; implemented this.

#### Testing

Told Claude to read the `TESTING.md` doc for desirable edge cases to handle and work through verifying that all of them could be handled.

Line breaks in notes stopped displaying as a result of the initial implementation of handling long notes; fixed immediately. No other issues noticed to have been introduced.

#### Letting Claude summarize the result

```text
  Core + all four Additions — display, add/edit/delete, localStorage persistence, notes, tags, filtering, search, all working together with no circular imports and no innerHTML anywhere.
                                                 
  All three Enhancements — auto-protocol prefixing, duplicate detection with an actionable toast, and multi-tag AND filtering.                  
   
  A solid visual style — darkened Catppuccin Mocha, JetBrains Mono, zero border radius, teal accents, the whole terminal aesthetic.             
                                                            
  Plus extras along the way — sticky header, tag badges as filter shortcuts, hover-reveal edit/delete with mobile tap-focus fallback, note truncation with expand toggle, URL hostname fallback for titles, and a fairly thorough security pass (textContent throughout, explicit protocol guard, render-time href check).      
```

### What I learned

Live Server is more powerful than I was aware of. Some specifics of how `is-clamped` works in CSS. What was harder than expected: adhering to the assignment structure instead of writing a design doc properly the first time was a nightmare and I hated it. Claude handled the artificially-layered design doc well, which was nice.

### Known issues

I don't know how to resist the urge to say "It's obviously unusable." It is. It's an unusable toy, that's what the requirement specified. If I wanted to actually bookmark things I'd use raindrop.io or Semble. If I wanted to build a bookmark manager I would actually use that wasn't reinventing the wheel I'd be... actually, no, I'd just be reinventing Semble, I don't have any meaningful additions to their graph model.

That being said:

- Tags bar is going to be unwieldy at best with more than, like, five tags; this is an issue with the design brief as written
- I'm not satisfied with the way the header works/scrolls, due to the tags issue
- Can't test from VSC on my phone so mobile functionality is being taken on faith
- Font sizes aren't as varied in Safari as they're supposed to be, which I'm reasonably sure is a Safari problem
- The design doesn't quite look like I want it to, which is a fault of the fact that I don't know how to articulate what I want; this is as close as I've been able to get and the agent performed well in accommodating it.
- Contrast is low relative to WCAG in some places due to personal aesthetic preference and the "personal" nature of the toy, as well as not having told Claude to do otherwise.
- It would be nice if paragraph breaks in notes on bookmarks compressed themselves elegantly instead of being a fully-proportional line break; this is also out of scope.

---

## Adversarial feedback

Adversarial feedback was generated using a fresh Gemini instance in the same folder with a request for adversarial feedback of the code base. Feedback is located in `FEEDBACK.md`.

Most points of feedback focused, understandably, on the security (almost none) and a11y (highly tailored to a single idiosyncratic user on desktop; for example, "actions hidden behind hover" was literally a request) findings. **I personally decided that the nature of the project meant that most of the adversarial review findings were out of scope**, since they were directed towards a version of the app that would see more than one day or one user worth of uptime.

It is worth noting that this doesn't mean *Claude* had an "excuse" for many of the points of feedback per se; the performance and robustness sections specifically would have been within scope of the assignment as dictated to Claude for it to get "right" on the first try.

Gemini identified two major performance issues in the code as written, layout thrashing and redundant text processing.

### Prompting a fix

> Check out `_docs/FEEDBACK.md`; I'd like to address the performance issues in #2 (layout thrashing and redundant data processing), one by one. Read that section of the document, make a plan for how to address item #2.1, execute it, and notify me.

Then,

> Great! Can we hit #2.2 while we're here?

An actual fix would have probably focused on the security findings first and would have merited continuous testing to make sure that no functionality was broken.

Stopping after this was honestly kind of difficult but I wanted to redirect myself to the next project as opposed to working through the entire feedback document. The process of doing so would have been straightforward and is left as an exercise for the reader.

Last request:

> Could you add an explanatory tree to `README.md` so the structure of the project is clear at a glance? Append to the existing content.
