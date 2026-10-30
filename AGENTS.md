# rQuickShare engineering contract

All project documentation, code comments and commit messages are in English; converse with the owner in Russian. Keep the repository's GPL-3.0 attribution and upstream provenance intact.

## Development and branches

- `master` is the sole long-lived branch. Use a short-lived branch and a reviewed PR for changes, preserve required checks and inspect concurrent work first. Retire branches only after their unique work is preserved. Never force-push or bypass protected mainline checks. Publishing still requires separate explicit release authorization.
- Target `master` for reviewed changes; do not recreate a permanent `dev` branch. Remove task branches only when their work is preserved and no agent/worktree still owns them.
- Keep code/config and documentation in separate commits. Record decisions and rollback boundaries before architectural work; implement the root cause, not a workaround.
- **Never poll a running CI or package workflow.** Trigger the appropriate final gate only after finishing the change, give the owner its URL and current unverified scope, then stop. Continue only after a completion event or a new user instruction. Do not repeatedly query workflow status, trigger more builds while a run is active or start CI for documentation-only changes.
- CI success is not proof of installed-device behavior. Run stable source checks at the end of work, race-sensitive/heavy checks only at the applicable final gate, and keep real-device/peer acceptance separate. Do not claim a feature complete until its required source and native evidence is available.

## Security and product invariants

rQuickShare is a nearby-sharing client with explicit incoming consent. Treat peer advertisements, labels, file names, clipboard payloads and transport messages as untrusted.

- Device identity and route equivalence require authenticated, session-bound evidence. Never merge peers based only on visible name, IP, MAC or untrusted mDNS metadata. Retain independent routes when identity proof is insufficient.
- Maintain consent/decline precedence, transfer ownership, bounded buffers, cancellation and interruption safety. Reject ambiguous state instead of completing or moving someone else's transfer.
- Private data, credentials, device identities, file paths and packet traces must not enter public diagnostics, screenshots, issues or documentation.
- Preserve Linux native daemon, user-session and desktop ownership boundaries. A passing Rust fixture does not establish a valid Android peer exchange.
- Releases must preserve licensing, signing, immutable artifacts, reviewed protected ref and explicit owner authorization. No tag, private signing operation, R2 write or production deployment without separate permission.

## Documentation

`README.md` is written for users: motivation, functions, supported platforms, feedback and license. It must not expose personal machinery, internal hosting/bucket layout, unpublished platform promises, CI status, draft translations, cryptographic account custody or release timelines.

Keep `docs/README.md` as a concise navigation page, `docs/product-direction.md` as a product contract, and `docs/active-backlog.md` as a short queue pointing to actual open Issues. Avoid dated diaries in active documents. Historic reports can remain in Git history and archived documents but must never be treated as the latest state or duplicated in entrypoints.

Accept bug reports and improvement requests through Issues, not unsolicited outside PRs. Preserve fork rights and upstream license notices.

## Completion

Report what was changed, what was actually tested and what remains unverified. A CI trigger is a handoff point, not a reason to query it repeatedly.
