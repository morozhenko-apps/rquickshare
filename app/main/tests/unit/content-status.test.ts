// @vitest-environment happy-dom

import { flushPromises, mount } from '@vue/test-utils';
import { describe, expect, test, vi } from 'vitest';

const { readTextMock } = vi.hoisted(() => ({
	readTextMock: vi.fn(),
}));

vi.mock('@tauri-apps/plugin-clipboard-manager', () => ({
	readText: readTextMock,
}));

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

	test('selecting folders emits folder paths through the same files payload', async () => {
		const dialogOpen = vi.fn().mockResolvedValue(['/tmp/Trip', '/tmp/Work']);
		const invoke = vi.fn().mockResolvedValue(undefined);
		const wrapper = mount(ContentStatus, {
			props: {
				vm: vm({ dialogOpen, invoke }),
			},
		});

		const folderButton = wrapper.findAll('button').find((button) =>
			button.text().includes('Select folders'),
		)!;
		await folderButton.trigger('click');
		await flushPromises();

		expect(dialogOpen).toHaveBeenCalledWith({
			title: 'Select folders to send',
			directory: true,
			multiple: true,
		});
		expect(wrapper.emitted('outboundPayload')).toEqual([
			[{ Files: ['/tmp/Trip', '/tmp/Work'] }],
		]);
		expect(invoke).toHaveBeenCalledWith('start_discovery');
		expect(wrapper.emitted('discoveryRunning')).toHaveLength(1);
	});

	test('cancelled folder picker does not start discovery or emit a payload', async () => {
		const dialogOpen = vi.fn().mockResolvedValue(null);
		const invoke = vi.fn().mockResolvedValue(undefined);
		const wrapper = mount(ContentStatus, {
			props: {
				vm: vm({ dialogOpen, invoke }),
			},
		});

		const folderButton = wrapper.findAll('button').find((button) =>
			button.text().includes('Select folders'),
		)!;
		await folderButton.trigger('click');
		await flushPromises();

		expect(wrapper.emitted('outboundPayload')).toBeUndefined();
		expect(invoke).not.toHaveBeenCalledWith('start_discovery');
	});

	test('shares clipboard text and starts discovery', async () => {
		readTextMock.mockResolvedValueOnce('https://example.com');
		const invoke = vi.fn().mockResolvedValue(undefined);
		const wrapper = mount(ContentStatus, {
			props: {
				vm: vm({ invoke }),
			},
		});

		const clipboardButton = wrapper.findAll('button').find((button) =>
			button.text().includes('Paste clipboard'),
		)!;
		await clipboardButton.trigger('click');
		await flushPromises();

		expect(wrapper.emitted('outboundPayload')).toEqual([
			[{ Text: 'https://example.com' }],
		]);
		expect(invoke).toHaveBeenCalledWith('start_discovery');
		expect(wrapper.emitted('discoveryRunning')).toHaveLength(1);
	});

	test('falls back to a clipboard image when text is unavailable', async () => {
		readTextMock.mockRejectedValueOnce(new Error('not text'));
		const invoke = vi.fn(async (command: string) => {
			if (command === 'save_clipboard_image') return '/tmp/rquickshare-clipboard-test.png';
			return undefined;
		});
		const wrapper = mount(ContentStatus, { props: { vm: vm({ invoke }) } });

		const clipboardButton = wrapper.findAll('button').find((button) =>
			button.text().includes('Paste clipboard'),
		)!;
		await clipboardButton.trigger('click');
		await flushPromises();

		expect(wrapper.emitted('outboundPayload')).toEqual([
			[{ EphemeralFiles: ['/tmp/rquickshare-clipboard-test.png'] }],
		]);
		expect(invoke).toHaveBeenCalledWith('start_discovery');
	});


	test('uses image fallback for blank text without restarting active discovery', async () => {
		readTextMock.mockResolvedValueOnce('   ');
		const invoke = vi.fn(async (command: string) => {
			if (command === 'save_clipboard_image') {
				return '/tmp/rquickshare-clipboard-1-2-0.png';
			}
			return undefined;
		});
		const wrapper = mount(ContentStatus, {
			props: {
				vm: vm({ invoke, discoveryRunning: true }),
			},
		});

		const clipboardButton = wrapper.findAll('button').find((button) =>
			button.text().includes('Paste clipboard'),
		)!;
		await clipboardButton.trigger('click');
		await flushPromises();

		expect(wrapper.emitted('outboundPayload')).toEqual([
			[{ EphemeralFiles: ['/tmp/rquickshare-clipboard-1-2-0.png'] }],
		]);
		expect(invoke).toHaveBeenCalledWith('save_clipboard_image');
		expect(invoke).not.toHaveBeenCalledWith('start_discovery');
		expect(wrapper.emitted('discoveryRunning')).toHaveLength(1);
	});

	test('shows an inline error when clipboard has no shareable content', async () => {
		readTextMock.mockResolvedValueOnce('   ');
		const invoke = vi.fn(async (command: string) => {
			if (command === 'save_clipboard_image') throw new Error('not image');
			return undefined;
		});
		const wrapper = mount(ContentStatus, { props: { vm: vm({ invoke }) } });

		const clipboardButton = wrapper.findAll('button').find((button) =>
			button.text().includes('Paste clipboard'),
		)!;
		await clipboardButton.trigger('click');
		await flushPromises();

		expect(wrapper.text()).toContain('Clipboard does not contain shareable text or an image.');
		expect(wrapper.emitted('outboundPayload')).toBeUndefined();
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
