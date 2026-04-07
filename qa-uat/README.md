# QA/UAT Testing Framework for Tauri 2 Desktop Application

## Overview

This directory contains the **Quality Assurance (QA)** and **User Acceptance Testing (UAT)** infrastructure for the cross-platform Tauri 2 desktop application.

## Testing Strategy

### 1. E2E Desktop Testing (Tauri Playwright)

**Location:** `e2e-desktop-playwright/`

**Stack:**
- Playwright with Tauri integration
- Direct Tauri binary execution (no WebDriver needed)
- Migrated from legacy WDIO in Phase 14 (2026-04-07)

**Test Coverage:**
- ✅ Login page (auth guard, OAuth flow, responsive design)
- ✅ Counter page (increment/decrement/reset, auth guard, tenant isolation)
- ✅ Desktop-specific behaviors

**Commands:**
```bash
# Run desktop E2E tests (auto-builds binary)
just test-desktop

# Fast run (cached build)
just test-desktop-fast

# Full E2E gate (web + desktop)
just test-e2e-full
```

**Note:** Legacy `e2e-tests/` (WDIO) has been retired. See `.agents/playbooks/e2e/migration.md` for migration details.

### 2. Web E2E Testing (Playwright)

**Location:** `apps/client/web/app/tests/e2e/`

**Coverage:** 37 tests across all pages
- Login flow (5 tests)
- Counter (8 tests)
- Admin Dashboard (7 tests)
- Agent Chat (7 tests)
- Tenant Isolation (4 tests)
- Token Refresh (6 tests)

### 3. Component Testing (Vitest)

**Location:** `apps/client/web/app/tests/component/`

**Coverage:** Unit-level component testing with mocks

## Platform Support

| Platform | WDIO Desktop E2E | Playwright Web E2E |
|----------|------------------|-------------------|
| Windows  | ✅ Supported      | ✅ Supported       |
| Linux    | ✅ Supported      | ✅ Supported       |
| macOS    | ⚠️ Force only     | ✅ Supported       |

## Prerequisites

### Required Tools
```bash
# Install dependencies via mise
mise install

# Install Node.js dependencies
just setup

# Verify toolchain
just doctor
```

### Windows-Specific Setup
1. Ensure Visual Studio Build Tools 2019+ is installed (provides cl.exe)
2. Ensure WebView2 runtime is present (Windows 10+ includes it)
3. Run terminal as Administrator if encountering permission issues

## Test Execution

### Local Development
```bash
# Run all desktop E2E tests
just test-desktop

# Run web E2E tests
just test-e2e

# Run both
just test-e2e-full
```

### CI/CD Integration
```bash
# In CI pipeline (GitHub Actions)
moon run repo:test-e2e-full

# Test results are output to:
# - e2e-desktop-playwright/playwright-report/  (HTML report)
# - e2e-desktop-playwright/test-results/       (Playwright results)
```

## QA Checklist

Before releasing a new version, verify:

### Functional Testing
- [ ] Login page loads and shows Google Sign-In button
- [ ] OAuth flow completes successfully (mock or real)
- [ ] Counter page: increment, decrement, reset work
- [ ] Admin dashboard displays stats correctly
- [ ] Agent chat: input area, send button, sidebar visible
- [ ] Auth guard redirects unauthenticated users to login

### Cross-Platform Testing
- [ ] Tests pass on Windows 10/11
- [ ] Tests pass on Linux (Ubuntu 22.04+)
- [ ] UI is responsive at mobile viewport (375x667)
- [ ] App builds successfully on each platform

### Performance
- [ ] App launches within 5 seconds
- [ ] Login page renders within 2 seconds
- [ ] Page navigation completes within 1 second
- [ ] No memory leaks after extended use (30+ minutes)

### Edge Cases
- [ ] App handles missing network gracefully
- [ ] Auth token expiry triggers logout
- [ ] Invalid OAuth state parameter rejected
- [ ] App recovers from crash without data loss

## UAT Acceptance Criteria

### User Stories

#### 1. Login & Authentication
**As a user, I want to:**
- See a clear login page with Google Sign-In
- Be redirected to the app after successful authentication
- Have my session persist across app restarts
- Be logged out when my token expires

**Acceptance:**
- ✅ Google Sign-In button visible and clickable
- ✅ OAuth callback processes correctly
- ✅ Session persists in Tauri Store
- ✅ Expired token triggers re-authentication

#### 2. Counter Feature
**As a user, I want to:**
- View the current counter value
- Increment, decrement, and reset the counter
- See changes persist across page reloads

**Acceptance:**
- ✅ Counter displays initial value (0)
- ✅ Increment button increases value by 1
- ✅ Decrement button decreases value by 1
- ✅ Reset button returns value to 0
- ✅ Changes persist via backend (libsql)

#### 3. Admin Dashboard
**As an admin, I want to:**
- View system statistics at a glance
- See user counts, sessions, revenue, and growth
- View charts for trends over time

**Acceptance:**
- ✅ Dashboard title visible
- ✅ 4 stat cards display correct values
- ✅ Chart placeholders render
- ✅ Data loads from backend API

#### 4. Agent Chat
**As a user, I want to:**
- Create and manage conversations
- Type messages and send them
- See message history in sidebar

**Acceptance:**
- ✅ "New Chat" button visible in sidebar
- ✅ Message input area present
- ✅ Send button disabled when input empty
- ✅ Send button enabled when input has text

## Troubleshooting

### Common Issues

#### `tauri-driver not found`
```bash
cargo install tauri-driver --locked
```

#### `App binary not found`
```bash
# Build the app first
cargo tauri build --debug --no-bundle --manifest-path apps/client/native/src-tauri/Cargo.toml
```

#### Tests fail with timeout errors
- Increase timeout in `wdio.conf.mjs` (currently 120s)
- Check if app is launching slowly due to debug mode
- Verify Axum API server is running on port 3001 (if needed)

#### WebDriver connection refused
- Ensure `tauri-driver` is running and listening on port 4444
- Check no other process is using port 4444
- Try killing stale tauri-driver processes: `pkill tauri-driver` (Linux/macOS) or `taskkill /F /IM tauri-driver.exe` (Windows)

## Reporting

### Test Results
After running tests, check:
- **Console output:** Playwright spec reporter
- **HTML Report:** `e2e-desktop-playwright/playwright-report/index.html`
- **Test Results:** `e2e-desktop-playwright/test-results/`

### CI Artifacts
In GitHub Actions, test results are uploaded as artifacts:
- `desktop-e2e-playwright-tauri-evidence` (Playwright reports)
- `web-e2e-evidence` (Web Playwright reports)

## Future Improvements

- [ ] Add visual regression testing (Percy/Chromatic)
- [ ] Add accessibility testing (axe-core)
- [ ] Add performance benchmarking
- [ ] Add network failure simulation tests
- [ ] Add multi-window testing
- [ ] Add system tray interaction tests
- [ ] Add Tauri Store persistence tests
- [ ] Add IPC command invoke/response tests
- [ ] Add real OAuth flow tests (not just mock)
- [ ] Add cross-platform matrix execution (parallel on Win/Linux/macOS)
