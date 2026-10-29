# Native 1.0.0 tag admission recovery

Date: 2026-10-29. MODE A: fix a proven release-workflow defect before reissuing the not-yet-published Native 1.0.0 tag.

## Evidence and root cause

- Annotated `native/v1.0.0` tag object `53ffa6bcdc11fa6f8ef3a5b3cfcd28cd4a34b364` points to protected `master` commit `388be27b116d7b4abc790ebb922eb4617e197bfe`.
- [Publish run 40960139207](https://github.com/morozhenko-apps/rquickshare/actions/runs/40960139207) failed in admission before Mode B, package, signing or R2 publication.
- GitHub checkout fetched the annotated tag, then its force-fetch of the commit ID into `refs/tags/native/v1.0.0` replaced the **runner-local** annotated-tag ref with the commit. `git cat-file -t refs/tags/native/v1.0.0` consequently returned `commit` rather than `tag`.
- The remote GitHub tag object remained valid; source signing keys, R2 access and app behavior were not implicated by this failure.

## Fix and regressions

1. After checkout, explicitly fetch the remote tag ref into a dedicated **non-tag** validation ref, without overwriting the runner's checkout tag or the remote release tag.
2. Require that fetched ref's object type is `tag`, and its dereferenced commit is exactly both the triggering `GITHUB_SHA` and the protected `master` HEAD. Retain canonical product version and Debian revision validation.
3. Update source-policy checks to require remote annotated-tag proof, and add an isolated local Git fixture reproducing checkout's replacement of a local tag and rejecting a remotely lightweight tag. Run the affected stable tests once at final gate.
4. Review and merge only this validated source change to protected `master`. After the merge, the owner explicitly requests moving/reissuing `native/v1.0.0` to the new exact protected master commit, retaining its annotated form. Preserve the previous tag-object SHA in evidence; do not move any other tags.
5. Verify the remote ref/object before and after the single authorized replacement. The retag is **not** proof of successful publication: full Mode B, signed R2/APT, public HTTPS and GitHub Release checks still apply. Stop at the started workflow without polling.

## Exceptional version/tag handling

The repository normally prohibits moving published immutable tags. This is a **one-off explicit owner instruction** for the first failed Native 1.0.0 launch: no GitHub Release was created, no Mode B/package/publish step ran, and the R2 `linux/InRelease` did not exist at the last owner check. A successful published tag must never be retargeted. If new evidence reveals any publication or a concurrent change, stop rather than force-overwrite. Apply force-with-lease against the exact old remote annotated tag object when the corrected master is ready. Never bypass protected master review, release Environment approval, package checks, or signing.

Rollback before tag replacement: revert the isolated source fix and leave the existing tag untouched. After tag replacement: preserve its audit trail and do not silently restore old code or retarget a published release.
