<!-- SPDX-License-Identifier: MPL-2.0 -->
# docs/wiki — In-repo source for the GitHub Wiki

These markdown pages are the canonical source for
<https://github.com/hyperpolymath/absolute-zero/wiki>.

## Why in-repo?

* Wiki pages get reviewed in PRs like any other code.
* The wiki history is recoverable from `git log` even if GitHub Wiki
  goes away.
* AI agents read this directory directly without needing wiki access.

## Sync to GitHub Wiki

The GitHub Wiki is itself a git repo at
`https://github.com/hyperpolymath/absolute-zero.wiki.git`.

To push these pages:

```bash
# Clone the wiki repo alongside the main repo
git clone https://github.com/hyperpolymath/absolute-zero.wiki.git ../absolute-zero.wiki

# Copy the in-repo pages over
cp docs/wiki/*.md ../absolute-zero.wiki/

# Commit + push to the wiki
cd ../absolute-zero.wiki
git add .
git commit -m "Sync from absolute-zero/docs/wiki@$(cd ../absolute-zero && git rev-parse --short HEAD)"
git push
```

(Automation TODO: a `Justfile` recipe `just wiki-sync` plus a workflow
that runs the above on every push to main. Tracked in
`RSR_COMPLIANCE.adoc`.)

## Page index

| Page | Status |
|------|--------|
| `Home.md` | Landing |
| `Architecture.md` | Three-layer architecture |
| `Proof-Systems.md` | Why six provers; what each covers |
| `Verification.md` | Build + CI matrix |
| `ABI.md` | Idris2 ABI + DivMod lemma surface |
| `Roadmap.md` | Short; cross-links to ROADMAP.adoc |
| `Contributing.md` | Short; cross-links to CONTRIBUTING.adoc |
| `Glossary.md` | CNO, =st=, postulate, etc. |
| `FAQ.md` | Common questions |
| `Audit-Trail.md` | Resolved + open audit items |
| `_Sidebar.md` | Persistent sidebar (rendered on every wiki page) |
