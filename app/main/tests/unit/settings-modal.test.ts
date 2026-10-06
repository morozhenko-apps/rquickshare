// @vitest-environment happy-dom

import { flushPromises, mount } from '@vue/test-utils';
import { describe, expect, test, vi } from 'vitest';

import SettingsModal from '../../src/composables/SettingsModal.vue';
import type { TauriVM } from '../../src/vue_lib/helper/ParamsHelper';

function vm(options: { savedPort?: number; dialogResult?: string | null } = {}): TauriVM {
	const values = new Map<string, unknown>();
	if (options.savedPort !== undefined) values.set('port', options.savedPort);

	const store = {
		get: vi.fn(async (key: string) => values.get(key)),
		set: vi.fn(async (key: string, value: unknown) => {
			values.set(key, value);
		}),
		delete: vi.fn(async (key: string) => {
			values.delete(key);
		}),
		save: vi.fn(async () => undefined),
	};

	return {
		settingsOpen: true,
		autostart: false,
		realclose: true,
		startminimized: false,
		downloadPath: undefined,
		store,
		enable: vi.fn(async () => undefined),
		disable: vi.fn(async () => undefined),
		invoke: vi.fn(async () => undefined),
		dialogOpen: vi.fn(async () => options.dialogResult ?? null),
	} as unknown as TauriVM;
}

describe('SettingsModal', () => {
	test('loads the saved listening port when settings opens', async () => {
		const model = vm({ savedPort: 32100 });
		const wrapper = mount(SettingsModal, { props: { vm: model } });
		await flushPromises();

		const input = wrapper.get('input[type="number"]');
		expect((input.element as HTMLInputElement).value).toBe('32100');
		expect(model.store.get).toHaveBeenCalledWith('port');
	});

	test('saves and clears a fixed listening port', async () => {
		const model = vm();
		const wrapper = mount(SettingsModal, { props: { vm: model } });
		await flushPromises();

		const input = wrapper.get('input[type="number"]');
		await input.setValue('32100');
		await wrapper.findAll('button').find((button) => button.text() === 'Save')!.trigger('click');
		await flushPromises();

		expect(model.store.set).toHaveBeenCalledWith('port', 32100);
		expect(model.store.save).toHaveBeenCalled();
		expect(wrapper.text()).toContain('Saved. Restart the app to apply.');

		vi.mocked(model.store.set).mockClear();
		await input.setValue('');
		await wrapper.findAll('button').find((button) => button.text() === 'Save')!.trigger('click');
		await flushPromises();

		expect(model.store.delete).toHaveBeenCalledWith('port');
		expect(model.store.set).not.toHaveBeenCalledWith('port', expect.anything());
	});

	test('rejects an invalid listening port without mutating the store', async () => {
		const model = vm();
		const wrapper = mount(SettingsModal, { props: { vm: model } });
		await flushPromises();

		await wrapper.get('input[type="number"]').setValue('80');
		await wrapper.findAll('button').find((button) => button.text() === 'Save')!.trigger('click');
		await flushPromises();

		expect(wrapper.text()).toContain('Use a port from 1024 to 65535.');
		expect(model.store.set).not.toHaveBeenCalledWith('port', expect.anything());
		expect(model.store.save).not.toHaveBeenCalled();
	});

	test('applies startup toggles through the settings store', async () => {
		const model = vm();
		const wrapper = mount(SettingsModal, { props: { vm: model } });
		await flushPromises();

		const checkboxes = wrapper.findAll('input[type="checkbox"]');
		expect(checkboxes).toHaveLength(3);

		await checkboxes[0].setValue(true);
		await flushPromises();
		expect(model.enable).toHaveBeenCalled();
		expect(model.store.set).toHaveBeenCalledWith('autostart', true);
		expect(model.autostart).toBe(true);

		await checkboxes[1].setValue(true);
		await flushPromises();
		expect(model.store.set).toHaveBeenCalledWith('realclose', false);
		expect(model.realclose).toBe(false);

		await checkboxes[2].setValue(true);
		await flushPromises();
		expect(model.store.set).toHaveBeenCalledWith('startminimized', true);
		expect(model.startminimized).toBe(true);
	});

	test('updates the download folder and emits close', async () => {
		const model = vm({ dialogResult: '/tmp/received-files' });
		const wrapper = mount(SettingsModal, { props: { vm: model } });
		await flushPromises();

		const folderButton = wrapper.findAll('button').find((button) =>
			button.text().includes('Download folder'),
		)!;
		await folderButton.trigger('click');
		await flushPromises();

		expect(model.invoke).toHaveBeenCalledWith('change_download_path', {
			message: '/tmp/received-files',
		});
		expect(model.store.set).toHaveBeenCalledWith('download_path', '/tmp/received-files');
		expect(model.downloadPath).toBe('/tmp/received-files');

		await wrapper.findAll('button').find((button) => button.text() === 'Close')!.trigger('click');
		expect(wrapper.emitted('close')).toHaveLength(1);
	});
});
