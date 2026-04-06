const API_BASE_URL = process.env.TAURI_E2E_API_BASE_URL || 'http://127.0.0.1:3001';
const TENANT_INIT_URL = `${API_BASE_URL}/api/tenant/init`;
const COUNTER_RESET_URL = `${API_BASE_URL}/api/counter/reset`;
const COUNTER_INCREMENT_URL = `${API_BASE_URL}/api/counter/increment`;
const COUNTER_VALUE_URL = `${API_BASE_URL}/api/counter/value`;

export const TENANT_1 = {
  label: 'tenant-1',
  userSub: 'tenant_a_user',
  userName: 'Tenant A User',
};

export const TENANT_2 = {
  label: 'tenant-2',
  userSub: 'tenant_b_user',
  userName: 'Tenant B User',
};

const TENANT_PAIR = [TENANT_1, TENANT_2];

function toBase64Url(input) {
  return Buffer.from(input)
    .toString('base64')
    .replace(/=/g, '')
    .replace(/\+/g, '-')
    .replace(/\//g, '_');
}

function makeTenantToken(userSub) {
  const header = toBase64Url(JSON.stringify({ alg: 'HS256', typ: 'JWT' }));
  const payload = toBase64Url(JSON.stringify({ sub: userSub, exp: 4_102_444_800 }));
  return `${header}.${payload}.desktop-e2e`;
}

function authHeaders(tenant) {
  return {
    Authorization: `Bearer ${makeTenantToken(tenant.userSub)}`,
    'content-type': 'application/json',
  };
}

async function parseJsonSafe(response) {
  try {
    return await response.json();
  } catch {
    return null;
  }
}

export async function initTenantPair() {
  for (const tenant of TENANT_PAIR) {
    const response = await fetch(TENANT_INIT_URL, {
      method: 'POST',
      headers: { 'content-type': 'application/json' },
      body: JSON.stringify({
        user_sub: tenant.userSub,
        user_name: tenant.userName,
      }),
    });

    const body = await parseJsonSafe(response);
    if (!response.ok || !body || typeof body.tenant_id !== 'string' || body.tenant_id.length === 0) {
      throw new Error(
        `[${tenant.label}] tenant init failed: status=${response.status}, body=${JSON.stringify(body)}`,
      );
    }
  }
}

export async function readTenantCounter(tenant) {
  const response = await fetch(COUNTER_VALUE_URL, {
    method: 'GET',
    headers: authHeaders(tenant),
  });

  const body = await parseJsonSafe(response);
  if (!response.ok || !body || typeof body.value !== 'number') {
    throw new Error(
      `[${tenant.label}] read counter failed: status=${response.status}, body=${JSON.stringify(body)}`,
    );
  }

  return body.value;
}

export async function incrementTenantCounter(tenant, times = 1) {
  for (let i = 0; i < times; i += 1) {
    const response = await fetch(COUNTER_INCREMENT_URL, {
      method: 'POST',
      headers: authHeaders(tenant),
    });

    const body = await parseJsonSafe(response);
    if (!response.ok || !body || typeof body.value !== 'number') {
      throw new Error(
        `[${tenant.label}] increment failed at step ${i + 1}: status=${response.status}, body=${JSON.stringify(body)}`,
      );
    }
  }
}

export async function resetTenantPair(seedValue = 0) {
  await initTenantPair();

  for (const tenant of TENANT_PAIR) {
    const resetResponse = await fetch(COUNTER_RESET_URL, {
      method: 'POST',
      headers: authHeaders(tenant),
    });

    const resetBody = await parseJsonSafe(resetResponse);
    if (!resetResponse.ok || !resetBody || typeof resetBody.value !== 'number') {
      throw new Error(
        `[${tenant.label}] reset failed: status=${resetResponse.status}, body=${JSON.stringify(resetBody)}`,
      );
    }

    if (seedValue > 0) {
      await incrementTenantCounter(tenant, seedValue);
    }

    const current = await readTenantCounter(tenant);
    if (current !== seedValue) {
      throw new Error(
        `[${tenant.label}] reset baseline mismatch: expected ${seedValue}, got ${current}`,
      );
    }
  }
}
