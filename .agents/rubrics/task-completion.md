# Task Completion Rubric

**Purpose:** Checklist to verify a task or feature is fully complete before marking it done.
**Enforced by:** Agent self-check + CI (`just verify`).

---

## Requirements Alignment

Before declaring a task complete, confirm:

- [ ] The task goal is clear and verifiable in one sentence
- [ ] All acceptance criteria from the original request are met
- [ ] Input, output, and constraints were correctly identified
- [ ] Any uncertain items identified during implementation are documented with their impact
- [ ] The implementation follows the minimum viable change principle (no over-engineering)

If any acceptance criterion is unmet, the task is NOT complete. Document what is missing and why.

---

## Verification Passed

All automated checks must pass:

```bash
# Full quality check (fmt + lint + typecheck + test)
just verify

# Type generation — no drift
just typegen

# Rust tests
cargo test
```

Additional checks:

- [ ] Frontend tests (Vitest) all pass
- [ ] E2E tests (Playwright) all pass for modified flows
- [ ] No lint warnings, or warnings are explicitly suppressed with justification
- [ ] No TypeScript compilation errors
- [ ] No Rust compilation warnings (or suppressed with `#[allow(...)]` and documented reason)
- [ ] Binary size has not grown significantly (> 1MB increase requires explanation)

---

## No Regression

Existing functionality must be unaffected:

- [ ] Full test suite passes (not just tests for modified code)
- [ ] No new compiler warnings introduced
- [ ] No unrelated files were modified
- [ ] No directory structure changes (unless explicitly part of the task)
- [ ] No new dependencies added (unless explicitly approved)
- [ ] No infrastructure or build chain modifications (unless explicitly part of the task)
- [ ] Generated files in `frontend/generated/` are consistent with contracts (no drift)

---

## Documentation Updated

All relevant documentation reflects the changes:

- [ ] `AGENTS.md` updated if any execution rules or constraints changed
- [ ] `README.md` updated if usage or setup instructions changed
- [ ] OpenAPI spec updated (via `utoipa` annotations) if API endpoints changed
- [ ] Commit message or PR description includes a changelog entry
- [ ] If changes affect other modules, relevant maintainers have been notified
- [ ] Playbooks or rubrics updated if the task modified how agents should work

---

## Output Format

When marking a task complete, output the following summary (per `AGENTS.md` Section 6):

1. **Task goal** — One sentence describing what was accomplished
2. **Change summary** — What changed and why
3. **Files involved** — List of modified, created, or deleted files
4. **Verification results** — Which checks passed (just verify, cargo test, vitest, etc.)
5. **Risks / limitations / incomplete items** — Any known issues or trade-offs
6. **Next steps** — Recommended follow-up work (if any)

---

## Completion Checklist

Final self-check before marking done:

- [ ] **Requirements aligned**: All acceptance criteria satisfied
- [ ] **Verification passed**: `just verify` / `cargo test` / Vitest all green
- [ ] **No regression**: Existing tests pass, no new warnings, no unrelated changes
- [ ] **Documentation updated**: Relevant docs, README, API spec synchronized
- [ ] **Output format**: Summary provided per `AGENTS.md` Section 6
- [ ] **Boundary compliant**: No cross-layer import violations (see `boundary-compliance.md`)
- [ ] **Code quality**: Passes `code-review.md` rubric check

If any item above is unchecked, the task is NOT complete. Return to implementation or document why the item does not apply.
