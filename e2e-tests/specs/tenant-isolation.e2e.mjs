import assert from 'node:assert/strict';
import {
  TENANT_1,
  TENANT_2,
  incrementTenantCounter,
  readTenantCounter,
  resetTenantPair,
} from '../helpers/tenant.mjs';

describe('Desktop Tenant Isolation', () => {
  const BASELINE = 0;
  const TENANT_1_WRITES = 2;

  beforeEach(async () => {
    await resetTenantPair(BASELINE);
  });

  it('uses the same fixed tenant pair as web harness', async () => {
    assert.equal(TENANT_1.userSub, 'tenant_a_user', 'tenant-1 userSub must match web fixture');
    assert.equal(TENANT_2.userSub, 'tenant_b_user', 'tenant-2 userSub must match web fixture');
  });

  it('tenant-1 write does not alter tenant-2 value (run-1)', async () => {
    await assertIsolationFlow({ runLabel: 'run-1', seed: BASELINE, writes: TENANT_1_WRITES });
  });

  it('tenant-1 write does not alter tenant-2 value (run-2, same seed)', async () => {
    await assertIsolationFlow({ runLabel: 'run-2', seed: BASELINE, writes: TENANT_1_WRITES });
  });
});

async function assertIsolationFlow({ runLabel, seed, writes }) {
  const tenant1Start = await readTenantCounter(TENANT_1);
  assert.equal(
    tenant1Start,
    seed,
    `[${runLabel}] tenant-1 baseline mismatch: expected ${seed}, got ${tenant1Start}`,
  );

  await incrementTenantCounter(TENANT_1, writes);

  const tenant1AfterWrite = await readTenantCounter(TENANT_1);
  const expectedTenant1 = seed + writes;
  assert.equal(
    tenant1AfterWrite,
    expectedTenant1,
    `[${runLabel}] tenant-1 write result mismatch: expected ${expectedTenant1}, got ${tenant1AfterWrite}`,
  );

  const tenant2AfterWrite = await readTenantCounter(TENANT_2);
  assert.equal(
    tenant2AfterWrite,
    seed,
    `[${runLabel}] tenant-2 leaked after tenant-1 writes: expected ${seed}, got ${tenant2AfterWrite}`,
  );
}
