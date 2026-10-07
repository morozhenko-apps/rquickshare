import { describe, expect, test, vi } from 'vitest';

import { utils } from '../../src/vue_lib/utils';
import type { TauriVM } from '../../src/vue_lib/helper/ParamsHelper';

function vm(
	payload: TauriVM['outboundPayload'],
	invoke: TauriVM['invoke'],
	overrides: Partial<TauriVM> = {},
): TauriVM {
	return {
		outboundPayload: payload,
		invoke,
		discoveryRunning: true,
		endpointsInfo: [{ id: 'peer' }],
		requests: [],
		...overrides,
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

describe('display lifecycle', () => {
	test('terminal history alone keeps the outbound composer available', () => {
		const target = vm(undefined, vi.fn(), {
			discoveryRunning: false,
			endpointsInfo: [],
			requests: [{
				id: 'peer',
				direction: 'LibToFront',
				action: null,
				meta: null,
				state: 'Finished',
				rtype: null,
				error: null,
			}],
		});

		expect(utils.hasLiveDisplayContent(target)).toBe(false);
		expect(utils._displayedItems(target)).toHaveLength(1);
		expect(utils._displayedItems(target)[0].state).toBe('Finished');
	});

	test('active transfer, live endpoint, or outbound payload switches to live mode', () => {
		const invoke = vi.fn();

		expect(utils.hasLiveDisplayContent(vm(undefined, invoke, {
			endpointsInfo: [],
			requests: [{
				id: 'peer',
				direction: 'LibToFront',
				action: null,
				meta: null,
				state: 'ReceivingFiles',
				rtype: null,
				error: null,
			}],
		}))).toBe(true);

		expect(utils.hasLiveDisplayContent(vm(undefined, invoke))).toBe(true);

		expect(utils.hasLiveDisplayContent(vm(
			{ Files: ['/tmp/photo.jpg'] },
			invoke,
			{ endpointsInfo: [], requests: [] },
		))).toBe(true);
	});

	test('live endpoint wins over stale terminal history while preparing a new send', () => {
		const target = vm({ Files: ['/tmp/photo.jpg'] }, vi.fn(), {
			endpointsInfo: [{
				id: 'peer',
				name: 'Mercury',
				rtype: 'Phone',
				ip: '192.168.1.2',
				port: 12345,
				present: true,
			}],
			requests: [{
				id: 'peer',
				direction: 'LibToFront',
				action: null,
				meta: {
					source: {
						name: 'Mercury',
						device_type: 'Phone',
					},
				},
				state: 'Finished',
				rtype: null,
				error: null,
			}],
		});

		expect(utils._displayedItems(target)).toEqual([
			expect.objectContaining({
				id: 'peer',
				name: 'Mercury',
				endpoint: true,
				state: undefined,
			}),
		]);
	});

	test('active transfer still wins over the live endpoint with the same id', () => {
		const target = vm({ Files: ['/tmp/photo.jpg'] }, vi.fn(), {
			endpointsInfo: [{
				id: 'peer',
				name: 'Mercury',
				rtype: 'Phone',
				ip: '192.168.1.2',
				port: 12345,
				present: true,
			}],
			requests: [{
				id: 'peer',
				direction: 'LibToFront',
				action: null,
				meta: {
					source: {
						name: 'Mercury',
						device_type: 'Phone',
					},
				},
				state: 'SendingFiles',
				rtype: null,
				error: null,
			}],
		});

		expect(utils._displayedItems(target)).toEqual([
			expect.objectContaining({
				id: 'peer',
				name: 'Mercury',
				endpoint: false,
				state: 'SendingFiles',
			}),
		]);
	});
});

