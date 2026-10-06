// @vitest-environment happy-dom

import { flushPromises, mount } from '@vue/test-utils';
import { describe, expect, test, vi } from 'vitest';

import Heading from '../../src/composables/Heading.vue';
import type { TauriVM } from '../../src/vue_lib/helper/ParamsHelper';

function vm(overrides: Partial<TauriVM> = {}): TauriVM {
	return {
		hostname: 'Alcotester',
		version: '0.11.5',
		new_version: undefined,
		...overrides,
	} as TauriVM;
}

describe('Heading', () => {
	test('shows hostname/version and emits settings event', async () => {
		const wrapper = mount(Heading, {
			props: {
				vm: vm(),
				openUrl: vi.fn(),
			},
		});

		expect(wrapper.text()).toContain('Alcotester');
		expect(wrapper.text()).toContain('v0.11.5');

		await wrapper.get('[aria-label="Open settings"]').trigger('click');
		expect(wrapper.emitted('openSettings')).toHaveLength(1);
	});

	test('opens the maintained fork release page when an update is available', async () => {
		const openUrl = vi.fn();
		const wrapper = mount(Heading, {
			props: {
				vm: vm({ new_version: '0.12.0' }),
				openUrl,
			},
		});

		expect(wrapper.text()).toContain('Update v0.12.0');
		await wrapper.get('.status-chip--active').trigger('click');
		await flushPromises();

		expect(openUrl).toHaveBeenCalledWith(
			'https://github.com/morozhenko-apps/rquickshare/releases/latest',
		);
	});
});
