import { describe, expect, test, vi } from 'vitest';

import { utils } from '../../src/vue_lib/utils';
import type { TauriVM } from '../../src/vue_lib/helper/ParamsHelper';

function vm(payload: TauriVM['outboundPayload'], invoke: TauriVM['invoke']): TauriVM {
	return {
		outboundPayload: payload,
		invoke,
		discoveryRunning: true,
		endpointsInfo: [{ id: 'peer' }],
	} as TauriVM;
}

describe('clearSending', () => {
	test('removes every ephemeral file before resetting discovery state', async () => {
		const invoke = vi.fn().mockResolvedValue(undefined);
		const target = vm({
			EphemeralFiles: [
				'/tmp/rquickshare-clipboard-1-2-0.png',
				'/tmp/rquickshare-clipboard-1-3-0.png',
			],
		}, invoke);

		await utils.clearSending(target);

		expect(invoke.mock.calls).toEqual([
			['remove_ephemeral_file', { path: '/tmp/rquickshare-clipboard-1-2-0.png' }],
			['remove_ephemeral_file', { path: '/tmp/rquickshare-clipboard-1-3-0.png' }],
			['stop_discovery'],
		]);
		expect(target.outboundPayload).toBeUndefined();
		expect(target.discoveryRunning).toBe(false);
		expect(target.endpointsInfo).toEqual([]);
	});

	test('cleanup failure does not prevent state reset or discovery stop', async () => {
		const warn = vi.spyOn(console, 'warn').mockImplementation(() => undefined);
		const invoke = vi.fn(async (command: string) => {
			if (command === 'remove_ephemeral_file') throw new Error('cleanup failed');
			return undefined;
		});
		const target = vm({
			EphemeralFiles: ['/tmp/rquickshare-clipboard-1-2-0.png'],
		}, invoke);

		await utils.clearSending(target);

		expect(invoke).toHaveBeenCalledWith('stop_discovery');
		expect(target.outboundPayload).toBeUndefined();
		expect(target.discoveryRunning).toBe(false);
		expect(target.endpointsInfo).toEqual([]);
		expect(warn).toHaveBeenCalledOnce();
		warn.mockRestore();
	});

	test('normal files never invoke ephemeral deletion', async () => {
		const invoke = vi.fn().mockResolvedValue(undefined);
		const target = vm({ Files: ['/home/user/document.pdf'] }, invoke);

		await utils.clearSending(target);

		expect(invoke).toHaveBeenCalledTimes(1);
		expect(invoke).toHaveBeenCalledWith('stop_discovery');
	});
});
