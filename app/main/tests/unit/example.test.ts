import { describe, expect, test } from 'vitest';

import { parseListeningPort } from '../../src/vue_lib/network';

describe('parseListeningPort', () => {
	test('uses automatic port selection when the input is empty', () => {
		expect(parseListeningPort('')).toEqual({ port: null, error: null });
		expect(parseListeningPort('   ')).toEqual({ port: null, error: null });
	});

	test('accepts the supported user port range', () => {
		expect(parseListeningPort('1024')).toEqual({ port: 1024, error: null });
		expect(parseListeningPort('32100')).toEqual({ port: 32100, error: null });
		expect(parseListeningPort(32100)).toEqual({ port: 32100, error: null });
		expect(parseListeningPort('65535')).toEqual({ port: 65535, error: null });
	});

	test.each(['1023', '65536', '1.5', 'abc', 80, 70000, Number.NaN, Number.POSITIVE_INFINITY, Number.NEGATIVE_INFINITY])(
		'rejects invalid port %s',
		(value) => {
			const result = parseListeningPort(value);

			expect(result.port).toBeNull();
			expect(result.error).toBe('Use a port from 1024 to 65535.');
		},
	);
});
