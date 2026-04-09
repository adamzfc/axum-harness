#!/usr/bin/env bun
/**
 * run-api-e2e.ts — Newman API E2E test runner (cross-platform, bun-based)
 *
 * Usage:
 *   # Step 1: Start the server (in another terminal)
 *   cargo run --package runtime_server
 *
 *   # Step 2: Run E2E tests
 *   just test-api-e2e
 *   bun servers/api/tests/e2e/run-api-e2e.ts
 *
 * Prerequisites:
 *   - bun (package manager + runtime)
 *   - servers/api/tests/e2e/env.json with valid jwtToken
 *
 * The server must already be running on port 3001 before executing this script.
 */

import { $ } from "bun";
import path from "path";
import fs from "fs";

// ── Path resolution ──────────────────────────────────────────
const SCRIPT_DIR = import.meta.dir;
const PROJECT_ROOT = path.resolve(SCRIPT_DIR, "../../../..");
const COLLECTION = path.join(SCRIPT_DIR, "collection.json");
const ENV_FILE = path.join(SCRIPT_DIR, "env.json");
const PORT = process.env.API_PORT ?? "3001";

// ── Colors ────────────────────────────────────────────────────
const C = {
  green: (s: string) => `\x1b[32m${s}\x1b[0m`,
  yellow: (s: string) => `\x1b[33m${s}\x1b[0m`,
  red: (s: string) => `\x1b[31m${s}\x1b[0m`,
};

function banner(text: string) {
  console.log(C.green(`═══ ${text} ═══`));
}

// ── Prerequisites check ──────────────────────────────────────
function checkFiles() {
  const errors: string[] = [];
  if (!fs.existsSync(COLLECTION)) {
    errors.push(`Collection not found: ${COLLECTION}`);
  }
  if (!fs.existsSync(ENV_FILE)) {
    errors.push(
      `Environment file not found: ${ENV_FILE}\n  Copy from example: cp ${ENV_FILE}.example ${ENV_FILE}`,
    );
  }
  if (errors.length > 0) {
    console.error(errors.map((e) => C.red(`✗ ${e}`)).join("\n"));
    process.exit(1);
  }
}

// ── Server health check ──────────────────────────────────────
async function checkServer(port: string): Promise<boolean> {
  const healthUrl = `http://localhost:${port}/healthz`;
  try {
    const res = await fetch(healthUrl, { method: "GET" });
    return res.ok;
  } catch {
    return false;
  }
}

// ── Run Newman ───────────────────────────────────────────────
async function runNewman(): Promise<number> {
  console.log(C.green("═══ Running Tests ═══"));

  try {
    await $`bunx newman run ${COLLECTION} -e ${ENV_FILE} --bail --reporter-cli --reporter-junit-extra --reporter-junit-extra-output ${path.join(SCRIPT_DIR, "report.xml")}`.cwd(
      PROJECT_ROOT,
    );
    return 0;
  } catch (exitErr: unknown) {
    if (
      exitErr &&
      typeof exitErr === "object" &&
      "exitCode" in exitErr &&
      typeof (exitErr as { exitCode?: number }).exitCode === "number"
    ) {
      return (exitErr as { exitCode: number }).exitCode;
    }
    return 1;
  }
}

// ── Main ─────────────────────────────────────────────────────
async function main() {
  banner("Newman API E2E Tests");
  checkFiles();

  // Verify server is running
  if (!(await checkServer(PORT))) {
    console.error(
      C.red(`✗ Server is not running on port ${PORT}`),
    );
    console.log(
      C.yellow(
        `  Start the server first: cargo run --package runtime_server`,
      ),
    );
    process.exit(1);
  }

  console.log(C.green(`✓ Server is ready on port ${PORT}`));

  const exitCode = await runNewman();
  if (exitCode === 0) {
    banner("All E2E tests passed ✓");
  } else {
    console.error(
      C.red(`═══ E2E tests failed (exit code: ${exitCode}) ═══`),
    );
    console.log(
      C.yellow(`  Report: ${path.join(SCRIPT_DIR, "report.xml")}`),
    );
  }
  process.exit(exitCode);
}

main();
