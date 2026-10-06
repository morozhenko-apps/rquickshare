import { describe, expect, test, vi } from 'vitest';

import { writeClipboardWithFallback } from '../../src/vue_lib/clipboard';

describe('writeClipboardWithFallback', () => {
	test('uses the primary writer when it succeeds', async () => {
		const primary = vi.fn(async () => undefined);
		const fallback = vi.fn(async () => undefined);

		await expect(writeClipboardWithFallback('hello', primary, fallback))
			.resolves.toBe('primary');
		expect(primary).toHaveBeenCalledWith('hello');
		expect(fallback).not.toHaveBeenCalled();
	});

	test('uses the fallback writer when the primary writer fails', async () => {
		const primary = vi.fn(async () => {
			throw new Error('primary failed');
		});
		const fallback = vi.fn(async () => undefined);

		await expect(writeClipboardWithFallback('hello', primary, fallback))
			.resolves.toBe('fallback');
		expect(fallback).toHaveBeenCalledWith('hello');
	});

	test('returns null when every available writer fails', async () => {
		const primary = vi.fn(async () => {
			throw new Error('primary failed');
		});
		const fallback = vi.fn(async () => {
			throw new Error('fallback failed');
		});

		await expect(writeClipboardWithFallback('hello', primary, fallback))
			.resolves.toBeNull();
	});

	test('returns null when the primary writer fails and no fallback exists', async () => {
		const primary = vi.fn(async () => {
			throw new Error('primary failed');
		});

		await expect(writeClipboardWithFallback('hello', primary))
			.resolves.toBeNull();
	});
});
