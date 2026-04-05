# UAT Checklist for Tauri 2 Desktop Application

## Version: _________
## Date: ____________
## Tester: _________
## Platform: Windows ☐ | Linux ☐ | macOS ☐

## Pre-Flight Checks

- [ ] Application builds without errors: `cargo tauri build --debug --no-bundle`
- [ ] Application launches successfully
- [ ] No console errors on startup
- [ ] `.env` file exists and is configured
- [ ] Axum API server accessible on port 3001 (if required)

## Phase 1: Authentication & Login

### Login Page Display
- [ ] Login page renders within 3 seconds
- [ ] "Tauri App" heading visible
- [ ] "Welcome back" text displayed
- [ ] "Sign in with Google" button visible and enabled
- [ ] Email input field visible but disabled
- [ ] Lottie loading animation displays (if applicable)

### OAuth Flow
- [ ] Clicking "Sign in with Google" opens browser
- [ ] OAuth callback returns to app successfully
- [ ] User session stored in Tauri Store (`auth.json`)
- [ ] User redirected to `/counter` after login
- [ ] Session persists after app restart

### Auth Guard
- [ ] Navigating to `/counter` without auth redirects to `/login`
- [ ] Navigating to `/admin` without auth redirects to `/login`
- [ ] Navigating to `/agent` without auth redirects to `/login`
- [ ] Navigating to `/settings` without auth redirects to `/login`

### Token Expiry
- [ ] Expired token triggers `auth:expired` event
- [ ] User redirected to `/login` on expiry
- [ ] Auth store cleared properly
- [ ] No stale session data remains

## Phase 2: Counter Feature

### Basic Functionality
- [ ] Counter page loads with initial value `0`
- [ ] Counter value displayed in monospace font
- [ ] Three buttons visible: decrement, increment, reset

### Counter Operations
- [ ] Increment button increases value by 1
- [ ] Decrement button decreases value by 1
- [ ] Reset button returns value to 0
- [ ] Counter value persists after page reload
- [ ] Counter value stored in backend (libsql)

### Responsive Design
- [ ] Counter usable at 375px width (mobile)
- [ ] Buttons remain clickable on small screens
- [ ] Counter value remains visible

## Phase 3: Admin Dashboard

### Dashboard Display
- [ ] Admin page loads (if authenticated)
- [ ] "Admin Dashboard" heading visible
- [ ] Four stat cards displayed:
  - [ ] Total Users: 12,345
  - [ ] Active Sessions: 1,234
  - [ ] Revenue: $45,678
  - [ ] Growth: 8.2%

### Chart Placeholders
- [ ] "Revenue Over Time" chart area visible
- [ ] "User Activity" chart area visible
- [ ] Charts render (or show placeholder)

### Responsive Design
- [ ] Dashboard title visible at 375px width
- [ ] Stat cards stack properly on mobile
- [ ] Layout remains usable

## Phase 4: Agent Chat

### Chat Interface
- [ ] Agent page loads (if authenticated)
- [ ] Sidebar with conversation list visible
- [ ] "New Chat" button in sidebar
- [ ] "Select or create a conversation" text displayed
- [ ] Message input area with placeholder "Type a message..."
- [ ] Send button visible

### Send Button State
- [ ] Send button disabled when input is empty
- [ ] Send button enabled when input has text
- [ ] Send button clickable (functionality TBD)

### Responsive Design
- [ ] "New Chat" button visible at 375px width
- [ ] Message input usable on mobile
- [ ] Sidebar collapses or adapts appropriately

## Phase 5: Settings Page

### Settings Display
- [ ] Settings page loads (if authenticated)
- [ ] API Key input field visible
- [ ] Base URL input field visible
- [ ] Model configuration options visible
- [ ] Settings save to Tauri Store (`settings.json`)

### Settings Persistence
- [ ] Changes persist after app restart
- [ ] Settings load correctly on next launch

## Phase 6: Cross-Platform Validation

### Windows 10/11
- [ ] App builds without errors
- [ ] App launches from `.exe`
- [ ] WebView2 renders correctly
- [ ] System tray icon appears (if enabled)
- [ ] Right-click context menu works
- [ ] Keyboard shortcuts work (Ctrl+C, Ctrl+V, etc.)

### Linux (Ubuntu 22.04+)
- [ ] App builds without errors
- [ ] App launches from binary
- [ ] WebKit2GTK renders correctly
- [ ] System tray icon appears (if enabled)
- [ ] File dialogs work
- [ ] Deep links work (if applicable)

### macOS (if applicable)
- [ ] App builds without errors
- [ ] App launches from `.app` bundle
- [ ] WebKit renders correctly
- [ ] Menu bar integrates properly
- [ ] System tray (menu bar) icon appears

## Phase 7: Error Handling

### Network Failures
- [ ] App handles missing API server gracefully
- [ ] Error message displayed when backend unreachable
- [ ] App doesn't crash on network timeout
- [ ] Retry mechanism works (if implemented)

### Invalid State
- [ ] Invalid OAuth state parameter rejected
- [ ] Malformed deep links handled safely
- [ ] Corrupted Tauri Store data handled
- [ ] App recovers from crash without data loss

### Edge Cases
- [ ] Multiple rapid button clicks don't cause race conditions
- [ ] Counter handles large numbers correctly
- [ ] Session cleanup works after logout
- [ ] App handles missing `.env` file gracefully

## Phase 8: Performance

### Launch Time
- [ ] App launches within 5 seconds (cold start)
- [ ] Login page renders within 2 seconds
- [ ] No noticeable lag during navigation

### Runtime Performance
- [ ] No memory leaks after 30 minutes of use
- [ ] CPU usage remains reasonable (< 20% idle)
- [ ] No UI freezing during operations
- [ ] Smooth animations (if applicable)

### Resource Usage
- [ ] Binary size acceptable (< 50MB for debug build)
- [ ] Memory usage reasonable (< 200MB idle)
- [ ] Disk usage stable (no unbounded growth)

## Sign-Off

### Test Results
- Total tests executed: _____
- Passed: _____
- Failed: _____
- Blocked: _____
- Skipped: _____

### Issues Found
| ID | Description | Severity | Status |
|----|-------------|----------|--------|
|    |             |          |        |

### Approval
- [ ] All critical tests passed
- [ ] No blocking issues
- [ ] Performance acceptable
- [ ] Cross-platform validation complete
- [ ] Ready for release

**Tester Signature:** _________________  **Date:** _________

**QA Lead Approval:** _________________  **Date:** _________

**Product Owner Sign-off:** _________________  **Date:** _________
