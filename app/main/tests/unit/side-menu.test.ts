// @vitest-environment happy-dom

import { mount } from '@vue/test-utils';
import { describe, expect, test } from 'vitest';

import SideMenu from '../../src/composables/SideMenu.vue';
import type { TauriVM } from '../../src/vue_lib/helper/ParamsHelper';

function vm(overrides: Partial<TauriVM> = {}): TauriVM {
	return {
		visibility: 'Visible',
		outboundPayload: undefined,
		...overrides,
	} as TauriVM;
}

describe('SideMenu', () => {
	test('renders receive visibility and emits visibility toggle', async () => {
		const wrapper = mount(SideMenu, { props: { vm: vm() } });

		expect(wrapper.text()).toContain('Receiving');
		expect(wrapper.text()).toContain('Visible to everyone');
		expect(wrapper.text()).toContain('Nearby devices can find this computer');

		await wrapper.get('button').trigger('click');
		expect(wrapper.emitted('invertVisibility')).toHaveLength(1);
	});

	test('renders hidden receive state', () => {
		const wrapper = mount(SideMenu, {
			props: { vm: vm({ visibility: 'Invisible' }) },
		});

		expect(wrapper.text()).toContain('Hidden');
		expect(wrapper.text()).toContain('not advertised');
	});

	test('renders outbound files and emits cancel', async () => {
		const wrapper = mount(SideMenu, {
			props: {
				vm: vm({
					outboundPayload: {
						Files: ['/tmp/photo.jpg', '/tmp/notes.txt'],
					},
				}),
			},
		});

		expect(wrapper.text()).toContain('2 files');
		expect(wrapper.text()).toContain('photo.jpg');
		expect(wrapper.text()).toContain('notes.txt');

		await wrapper.get('button').trigger('click');
		expect(wrapper.emitted('clearSending')).toHaveLength(1);
	});
});
