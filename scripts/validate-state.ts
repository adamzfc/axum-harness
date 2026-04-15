/**
 * Validate State — verify service-local semantics align with global ownership rules.
 *
 * Usage:
 *   bun run scripts/validate-state.ts [--mode warn|strict]
 */

import { existsSync, readFileSync, readdirSync } from 'node:fs';
import path from 'node:path';
import process from 'node:process';

type Mode = 'warn' | 'strict';

interface ParsedArgs {
  mode: Mode;
}

interface StateIssue {
  level: 'warn' | 'error';
  scope: string;
  message: string;
}

interface ServiceModel {
  serviceDir: string;
  serviceName: string;
  entities: string[];
}

const workspaceRoot = process.cwd();

function parseArgs(argv: string[]): ParsedArgs {
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
    console.error('Usage: bun run scripts/validate-state.ts [--mode warn|strict]');
    process.exit(1);
  }

  return { mode };
}

function extractMatches(source: string, regex: RegExp): string[] {
  const matches: string[] = [];
  let match: RegExpExecArray | null;
  while ((match = regex.exec(source)) !== null) {
    matches.push(match[1]);
  }
  return matches;
}

function extractOwnsEntitiesSection(content: string): string {
  const start = content.indexOf('owns_entities:');
  if (start === -1) return '';

  const afterStart = content.slice(start);
  const nextSectionMatch = afterStart.slice('owns_entities:'.length).match(/\n\s{2}[a-z_]+:/);
  if (!nextSectionMatch || nextSectionMatch.index === undefined) {
    return afterStart;
  }

  return afterStart.slice(0, 'owns_entities:'.length + nextSectionMatch.index + 1);
}

function loadServiceModels(): ServiceModel[] {
  const servicesDir = path.join(workspaceRoot, 'services');
  if (!existsSync(servicesDir)) {
    return [];
  }

  const models: ServiceModel[] = [];
  for (const entry of readdirSync(servicesDir, { withFileTypes: true })) {
    if (!entry.isDirectory()) continue;

    const serviceDir = path.join(servicesDir, entry.name);
    const modelPath = path.join(serviceDir, 'model.yaml');
    if (!existsSync(modelPath)) continue;

    const content = readFileSync(modelPath, 'utf-8');
    const metadataName = content.match(/^\s*name:\s*([a-z][a-z0-9-]*)\s*$/m)?.[1] ?? entry.name;
    const ownsEntitiesSection = extractOwnsEntitiesSection(content);
    const entities = extractMatches(ownsEntitiesSection, /^\s*-\s+name:\s*([a-z][a-z0-9_-]*)\s*$/gm);

    models.push({
      serviceDir: entry.name,
      serviceName: metadataName,
      entities,
    });
  }

  return models.sort((a, b) => a.serviceDir.localeCompare(b.serviceDir));
}

function loadOwnershipMap(): Map<string, string> {
  const ownershipPath = path.join(workspaceRoot, 'platform', 'model', 'state', 'ownership-map.yaml');
  const map = new Map<string, string>();

  if (!existsSync(ownershipPath)) {
    return map;
  }

  const content = readFileSync(ownershipPath, 'utf-8');
  const entityBlocks = content.split(/\n\s*-\s+entity:\s*/g).slice(1);
  for (const block of entityBlocks) {
    const entity = block.match(/^([a-z][a-z0-9_-]*)/m)?.[1];
    const owner = block.match(/^\s*owner_service:\s*([a-z][a-z0-9-]*)\s*$/m)?.[1];
    if (entity && owner) {
      map.set(entity, owner);
    }
  }

  return map;
}

function validateState(mode: Mode): StateIssue[] {
  const issues: StateIssue[] = [];
  const ownershipMap = loadOwnershipMap();
  const serviceModels = loadServiceModels();
  const seenEntityOwners = new Map<string, string>();

  for (const model of serviceModels) {
    if (model.entities.length === 0) {
      issues.push({
        level: mode === 'strict' ? 'error' : 'warn',
        scope: `services/${model.serviceDir}/model.yaml`,
        message: 'missing owns_entities declarations',
      });
      continue;
    }

    for (const entity of model.entities) {
      const priorOwner = seenEntityOwners.get(entity);
      if (priorOwner && priorOwner !== model.serviceName) {
        issues.push({
          level: 'error',
          scope: entity,
          message: `entity declared by multiple services: ${priorOwner} and ${model.serviceName}`,
        });
      } else {
        seenEntityOwners.set(entity, model.serviceName);
      }

      const mappedOwner = ownershipMap.get(entity);
      if (!mappedOwner) {
        issues.push({
          level: mode === 'strict' ? 'error' : 'warn',
          scope: entity,
          message: `missing platform ownership-map entry for ${model.serviceName}`,
        });
        continue;
      }

      if (mappedOwner !== model.serviceName && mappedOwner !== model.serviceName.replace(/-service$/, '')) {
        issues.push({
          level: 'error',
          scope: entity,
          message: `ownership-map says ${mappedOwner}, service semantics say ${model.serviceName}`,
        });
      }
    }
  }

  return issues;
}

function printIssues(issues: StateIssue[]): void {
  if (issues.length === 0) {
    console.log('State validation passed');
    return;
  }

  for (const issue of issues) {
    const prefix = issue.level === 'error' ? 'ERROR' : 'WARN';
    console.log(`${prefix} [${issue.scope}] ${issue.message}`);
  }
}

function exitCodeForIssues(mode: Mode, issues: StateIssue[]): number {
  const hasBlocking = issues.some((issue) => issue.level === 'error');
  if (hasBlocking) return 1;
  if (mode === 'strict' && issues.length > 0) return 1;
  return 0;
}

const { mode } = parseArgs(process.argv.slice(2));
const issues = validateState(mode);
printIssues(issues);
process.exit(exitCodeForIssues(mode, issues));
