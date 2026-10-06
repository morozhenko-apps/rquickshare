// @vitest-environment happy-dom

import { flushPromises, mount } from '@vue/test-utils';
import { describe, expect, test, vi } from 'vitest';

import ContentStatus from '../../src/composables/ContentStatus.vue';
import type { TauriVM } from '../../src/vue_lib/helper/ParamsHelper';

function vm(overrides: Partial<TauriVM> = {}): TauriVM {
	return {
		displayedIsEmpty: true,
		endpointsInfo: [],
		outboundPayload: undefined,
		isDragHovering: false,
		discoveryRunning: false,
		dialogOpen: vi.fn(),
		invoke: vi.fn(),
		...overrides,
	} as TauriVM;
}

describe('ContentStatus', () => {
	test('renders ready/drop state', () => {
		const wrapper = mount(ContentStatus, { props: { vm: vm() } });

		expect(wrapper.text()).toContain('Ready to share');
		expect(wrapper.text()).toContain('Drop files here');
		expect(wrapper.get('[aria-label="Ready"]').exists()).toBe(true);
	});

	test('selecting files emits payload and starts discovery', async () => {
		const dialogOpen = vi.fn().mockResolvedValue([
			{ path: '/tmp/photo.jpg' },
			{ path: '/tmp/notes.txt' },
		]);
		const invoke = vi.fn().mockResolvedValue(undefined);
		const wrapper = mount(ContentStatus, {
			props: {
				vm: vm({ dialogOpen, invoke }),
			},
		});

		await wrapper.get('button').trigger('click');
		await flushPromises();

		expect(dialogOpen).toHaveBeenCalledOnce();
		expect(invoke).toHaveBeenCalledWith('start_discovery');
		expect(wrapper.emitted('outboundPayload')).toEqual([
			[{ Files: ['/tmp/photo.jpg', '/tmp/notes.txt'] }],
		]);
		expect(wrapper.emitted('discoveryRunning')).toHaveLength(1);
	});

	test('does not restart discovery when it is already running', async () => {
		const dialogOpen = vi.fn().mockResolvedValue(['/tmp/photo.jpg']);
		const invoke = vi.fn().mockResolvedValue(undefined);
		const wrapper = mount(ContentStatus, {
			props: {
				vm: vm({ dialogOpen, invoke, discoveryRunning: true }),
			},
		});

		await wrapper.get('button').trigger('click');
		await flushPromises();

		expect(invoke).not.toHaveBeenCalled();
		expect(wrapper.emitted('outboundPayload')).toHaveLength(1);
	});
});
