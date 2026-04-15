/**
 * Validate Workflows — lightweight completeness checks for platform workflow models.
 *
 * Usage:
 *   bun run scripts/validate-workflows.ts [--mode warn|strict]
 */

import { existsSync, readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';

type Mode = 'warn' | 'strict';

interface WorkflowIssue {
  level: 'warn' | 'error';
  file: string;
  message: string;
}

function parseMode(argv: string[]): Mode {
  let mode: Mode = 'warn';
  for (let index = 0; index < argv.length; index += 1) {
    const arg = argv[index];
    if (arg === '--mode') {
      const value = argv[index + 1];
      if (value === 'warn' || value === 'strict') {
        mode = value;
        index += 1;
        continue;
      }
    }

    console.error(`Unknown argument: ${arg}`);
    console.error('Usage: bun run scripts/validate-workflows.ts [--mode warn|strict]');
    process.exit(1);
  }
  return mode;
}

function validateWorkflow(content: string, file: string, mode: Mode): WorkflowIssue[] {
  const issues: WorkflowIssue[] = [];
  const requiredSnippets = [
    ['idempotency_key:', 'missing top-level idempotency_key'],
    ['checkpoint_policy:', 'missing checkpoint_policy'],
    ['compensation:', 'missing compensation section'],
    ['recovery:', 'missing recovery section'],
    ['resume_from:', 'missing recovery resume strategy'],
    ['operator_intervention_on:', 'missing operator intervention trigger'],
  ] as const;

  for (const [snippet, message] of requiredSnippets) {
    if (!content.includes(snippet)) {
      issues.push({
        level: mode === 'strict' ? 'error' : 'warn',
        file,
        message,
      });
    }
  }

  const stepBlocks = content.split(/\n\s*-\s+name:\s*/g).slice(1);
  for (const block of stepBlocks) {
    const stepName = block.match(/^([^\n]+)/)?.[1]?.trim() ?? 'unknown-step';
    if (!/checkpoint:\s*true/.test(block)) {
      issues.push({
        level: mode === 'strict' ? 'error' : 'warn',
        file,
        message: `step ${stepName} is missing checkpoint: true`,
      });
    }
    if (!/idempotency_key:/.test(block)) {
      issues.push({
        level: mode === 'strict' ? 'error' : 'warn',
        file,
        message: `step ${stepName} is missing idempotency_key`,
      });
    }
  }

  return issues;
}

function main(): number {
  const mode = parseMode(process.argv.slice(2));
  const workflowsDir = path.join(process.cwd(), 'platform', 'model', 'workflows');
  const issues: WorkflowIssue[] = [];

  if (!existsSync(workflowsDir)) {
    console.error('Workflow directory not found');
    return 1;
  }

  for (const entry of readdirSync(workflowsDir, { withFileTypes: true })) {
    if (!entry.isFile() || !entry.name.endsWith('.yaml')) continue;
    const file = path.join(workflowsDir, entry.name);
    const content = readFileSync(file, 'utf-8');
    issues.push(...validateWorkflow(content, path.relative(process.cwd(), file), mode));
  }

  if (issues.length === 0) {
    console.log('Workflow validation passed');
    return 0;
  }

  for (const issue of issues) {
    const prefix = issue.level === 'error' ? 'ERROR' : 'WARN';
    console.log(`${prefix} [${issue.file}] ${issue.message}`);
  }

  if (issues.some((issue) => issue.level === 'error')) {
    return 1;
  }

  return mode === 'strict' ? 1 : 0;
}

process.exit(main());
