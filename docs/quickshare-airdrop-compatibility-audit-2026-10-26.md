# Quick Share compatibility and AirDrop feasibility audit

Mode A, research/planning slice requested by the owner on 2026-10-26.
Baseline: `b19031a94332223d90d67556cd4960851f4882f1` on dev; executable baseline
`6ae62f62b17b058160d3246873ecdd34377ec9f0` (six-green CI, uninstalled candidate).

## Scope and execution plan

Investigate the complete discovery/connection/authentication/consent/transport/
payload/termination path against public Google Nearby source and current product
documentation. Distinguish confirmed source defects, missing capabilities,
performance hypotheses and measured physical behavior. Assess AirDrop separately.
This slice does not promise or implement all parity features at once.

1. [x] Read repository guide, live backlog, roadmap, identity and BWU decisions;
   refresh remote ownership/open issues. Clean dev, one worktree, origin has only
   dev/master, no open PR or observed concurrent owner. Local Ubuntu shell,
   network/Git/connected GitHub and web research available. Sequential terminals.
2. [ ] Map production paths and test/physical evidence; inspect current primary
   upstream sources. Record exact code anchors and immutable source revisions
   where available, rather than treating proto fields as implemented features.
3. [ ] Rank defects and optimization candidates with mechanism, confidence,
   risk, required regression and acceptance measurement. Preserve approved A2.2
   legacy BWU compatibility and verified-only identity choices.
4. [ ] Build Quick Share parity matrix and AirDrop feasibility/options with
   dependency/privilege/hardware constraints. Architectural choices are proposals,
   not approved implementation. No proprietary handshake or cloud dependency
   introduced without a specific subsequent decision.
5. [ ] Update live backlog and existing roadmap links; record a concrete ordered
   implementation queue. Validate documentation links/whitespace and workflow
   policy; commit documentation separately and normal-push dev after tip checks.

Code changes: none planned in this research slice. Documentation changes: this
audit, backlog and roadmap pointer. Architecture: alternatives documented only.
Risks: confusing Google public C++ source with Android GMS implementation,
unsynchronized clocks, unsupported modern AirDrop claims, or silently weakening
consent/trust to obtain interoperability. Rollback: inverse docs-only commit.
No new test suites, hardware traffic, service restarts, package installation,
master/tag/publication, credential extraction or system-network changes planned.

## Findings and evidence

Research in progress; final classification and sources follow in this document.
