#!/usr/bin/env node

import net from 'node:net';
import path from 'node:path';
import process from 'node:process';
import { fileURLToPath } from 'node:url';

const API_HOST = '127.0.0.1';
const API_PORT = 3001;
const WEB_PORT = 5173;
const READYZ_URL = `http://${API_HOST}:${API_PORT}/readyz`;
const READYZ_TIMEOUT_MS = 20_000;
const READYZ_POLL_INTERVAL_MS = 500;

const __dirname = fileURLToPath(new URL('.', import.meta.url));
const workspaceRoot = path.resolve(__dirname, '..', '..');
const svelteTypesDir = path.join(workspaceRoot, 'apps', 'client', 'web', 'app', '.svelte-kit', 'types');

function sleep(ms) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

function fail(message, suggestions = []) {
	console.error(`[runtime-preflight] FAIL: ${message}`);
	for (const suggestion of suggestions) {
		console.error(`[runtime-preflight] hint: ${suggestion}`);
	}
	process.exit(1);
}

function info(message) {
	console.log(`[runtime-preflight] ${message}`);
}

async function isPortOccupied(port, host = API_HOST) {
	if (process.env.E2E_PREFLIGHT_TEST_MODE === '1') {
		const status = process.env.E2E_PREFLIGHT_PORTS_STATUS ?? 'free';
		if (status === 'free') return false;
		const busy = status.replace('busy:', '').split(',').map((p) => Number.parseInt(p.trim(), 10));
		return busy.includes(port);
	}

	return new Promise((resolve) => {
		const socket = net.createConnection({ host, port }, () => {
			socket.end();
			resolve(true);
		});

		socket.on('error', () => {
			socket.destroy();
			resolve(false);
		});
	});
}

async function checkReadyz() {
	if (process.env.E2E_PREFLIGHT_TEST_MODE === '1') {
		return (process.env.E2E_PREFLIGHT_READYZ_STATUS ?? 'down') === 'ok';
	}

	const deadline = Date.now() + READYZ_TIMEOUT_MS;
	while (Date.now() < deadline) {
		try {
			const response = await fetch(READYZ_URL);
			if (response.ok) {
				return true;
			}
		} catch {
			// keep polling
		}

		await sleep(READYZ_POLL_INTERVAL_MS);
	}

	return false;
}

async function checkSvelteTypes() {
	if (process.env.E2E_PREFLIGHT_TEST_MODE === '1') {
		return (process.env.E2E_PREFLIGHT_TYPES_STATUS ?? 'ok') === 'ok';
	}

	const fs = await import('node:fs');
	return fs.existsSync(svelteTypesDir);
}

async function main() {
	info('checking API readyz health...');
	const ready = await checkReadyz();
	if (!ready) {
		fail(`API runtime is not ready at ${READYZ_URL} (timeout ${READYZ_TIMEOUT_MS}ms)`, [
			'start runtime server first: rtk cargo run -p runtime_server',
			'or run full desktop stack: rtk moon run repo:dev-desktop'
		]);
	}

	info('checking web type artifacts...');
	const typesReady = await checkSvelteTypes();
	if (!typesReady) {
		fail(`missing required SvelteKit types directory: ${svelteTypesDir}`, [
			'generate sync artifacts: rtk bun run --cwd apps/client/web/app check'
		]);
	}

	info('checking port hygiene (5173/3001)...');
	const webPortBusy = await isPortOccupied(WEB_PORT);
	if (webPortBusy) {
		fail(`port ${WEB_PORT} is occupied before web lane bootstrap`, [
			'stop the stale process bound to 5173, then rerun gate'
		]);
	}

	const apiPortBusy = await isPortOccupied(API_PORT);
	if (!apiPortBusy) {
		fail(`port ${API_PORT} is not listening although readyz check passed`, [
			'confirm runtime_server startup logs and retry'
		]);
	}

	info('preflight passed (readyz + ports + types)');
}

await main();
