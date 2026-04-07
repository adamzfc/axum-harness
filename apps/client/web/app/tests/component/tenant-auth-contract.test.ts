import { describe, expect, it } from 'vitest';
import { TENANT_A } from '../fixtures/tenant';
import { buildTenantAuthHeaders } from '../fixtures/auth';

describe('web tenant fixture auth contract', () => {
	it('builds Bearer authorization header for tenant bootstrap requests', () => {
		const headers = buildTenantAuthHeaders(TENANT_A.userSub);

		expect(headers.Authorization).toMatch(/^Bearer\s.+\..+\..+$/);
		expect(headers['content-type']).toBe('application/json');
	});
});
