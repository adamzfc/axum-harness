# Code Review Rubric

**Purpose:** Agent code review checklist for code quality. Complements `boundary-compliance.md` (which covers architecture boundaries).
**Enforced by:** CI (`cargo test`, `vitest`, `tsc`) + agent review (this rubric).

---

## Naming Conventions

### Rust

- Functions and variables: `snake_case`
- Types, structs, enums, traits: `PascalCase`
- Constants: `SCREAMING_SNAKE_CASE`
- Files: `snake_case.rs`

### TypeScript / Svelte

- Functions and variables: `camelCase`
- Components and interfaces: `PascalCase`
- Files: `kebab-case.ts` for utilities, `PascalCase.svelte` for components

### Prohibited

- Single-letter variable names (except loop counters `i`, `j`, `k`)
- Ambiguous abbreviations (`ctx` → use `context`, `req` → use `request`)
- Hungarian notation or type prefixes (`strName`, `arrItems`)

---

## Error Handling

### Rust

- Use `thiserror` to define error types for public APIs
- Never use `unwrap()` in production code (tests are fine)
- Never use `panic!()` except in tests or truly unrecoverable states
- Prefer `Result<T, AppError>` over `Option` when callers need error context
- Use `?` operator for propagation, not manual match chains

### TypeScript

- Always use `try/catch` around async operations (`fetch`, database calls, file I/O)
- Never swallow errors — at minimum, log them
- Return structured error responses from API endpoints:
  ```typescript
  { error: { code: string, message: string, details?: unknown[] } }
  ```
- Use `zod` or equivalent for input validation before processing

### Prohibited

- `console.log` left in production code (use proper logging)
- Empty `catch {}` blocks
- `panic!()` in non-test Rust code
- Throwing raw strings in TypeScript (always throw `Error` instances)

---

## Test Coverage

### Rust

- Every new public function must have a `#[cfg(test)]` unit test
- Error paths must be tested, not just happy paths
- Use `#[test]` for unit tests, `#[tokio::test]` for async tests
- Test naming: `fn should_return_error_when_input_is_empty()`

### Svelte / TypeScript

- Every new component must have a Vitest render test
- Test that component renders with required props
- Test user interactions (clicks, form submissions)
- Test error states and loading states

### API Endpoints

- Every new endpoint must have an integration test
- Test success response structure
- Test validation errors (400/422)
- Test authentication/authorization (401/403)

### Coverage Requirements

- New code line coverage ≥ 80%
- All branches tested (not just linear paths)
- Test names describe behavior, not implementation: `describe('searches markets by query')` not `describe('calls searchMarkets')`

---

## Documentation Completeness

### Rust

- Every public function must have `///` doc comments
- Include at minimum: what it does, what it returns, when it errors
- Use `# Examples` blocks for non-trivial APIs
- Use `# Panics`, `# Errors`, `# Safety` sections where applicable

### TypeScript

- Every exported function must have JSDoc comments
- Include `@param`, `@returns`, and `@throws` tags
- Use `@example` blocks for complex utilities

### Feature Documentation

- New features must update `AGENTS.md` or relevant README if they change how agents work
- New API endpoints must have `utoipa` annotations for OpenAPI spec generation
- Breaking changes must include migration instructions in commit message or PR description

---

## Review Checklist

When reviewing code changes, verify:

- [ ] Naming follows conventions — no ambiguous abbreviations or single-letter names
- [ ] Error handling is complete — no `unwrap()`, `panic!()`, or `console.log` in production
- [ ] New code has corresponding tests (unit, integration, or component)
- [ ] Public functions have documentation comments (`///` or JSDoc)
- [ ] No cross-layer import violations (see `boundary-compliance.md`)
- [ ] No unused variables, imports, or dependencies
- [ ] Functions ≤ 50 lines, files ≤ 800 lines
- [ ] Nesting depth ≤ 4 levels
- [ ] Input validation present at API boundaries
- [ ] No hardcoded secrets, API keys, or tokens
- [ ] Error messages do not leak sensitive information (stack traces, SQL errors)
