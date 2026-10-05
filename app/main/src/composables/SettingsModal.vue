<script setup lang="ts">
import { utils } from '../vue_lib';
import { PropType } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emit = defineEmits(['close']);

function openDownloadPicker() {
	props.vm.dialogOpen({
		title: "Select the destination for files",
		directory: true,
		multiple: false,
	}).then(async (el) => {
		if (el === null) {
			return;
		}

		await utils.setDownloadPath(props.vm, el as string);
	});
}
</script>

<template>
	<div v-if="vm.settingsOpen" class="modal-backdrop absolute inset-0 z-10 flex justify-center items-center p-6">
		<div class="modal-card rounded-2xl p-5 w-[28rem] max-w-full">
			<div class="flex flex-row justify-between items-center gap-4">
				<div>
					<h2 class="font-semibold text-xl">Settings</h2>
					<p class="text-sm text-muted mt-1">General behavior and received files.</p>
				</div>
				<button type="button" class="btn btn-secondary" @click="emit('close')">
					Close
				</button>
			</div>

			<div class="pt-5 flex flex-col gap-2">
				<div class="setting-row rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center gap-4" @click="utils.setAutoStart(vm, !vm.autostart)">
						<div>
							<p class="font-medium">Start on boot</p>
							<p class="text-xs text-muted mt-1">Launch Quick Share when you sign in.</p>
						</div>
						<input type="checkbox" :checked="vm.autostart" class="checkbox focus:outline-none">
					</label>
				</div>

				<div class="setting-row rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center gap-4" @click="utils.setRealClose(vm, !vm.realclose)">
						<div>
							<p class="font-medium">Keep running on close</p>
							<p class="text-xs text-muted mt-1">Hide the window instead of stopping the service.</p>
						</div>
						<input type="checkbox" :checked="!vm.realclose" class="checkbox focus:outline-none">
					</label>
				</div>

				<div class="setting-row rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center gap-4" @click="utils.setStartMinimized(vm, !vm.startminimized)">
						<div>
							<p class="font-medium">Start minimized</p>
							<p class="text-xs text-muted mt-1">Open directly in the background.</p>
						</div>
						<input type="checkbox" :checked="vm.startminimized" class="checkbox focus:outline-none">
					</label>
				</div>

				<button type="button" class="setting-row rounded-xl p-3 text-left" @click="openDownloadPicker()">
					<p class="font-medium">Download folder</p>
					<p class="overflow-hidden whitespace-nowrap text-ellipsis text-xs text-muted mt-1">
						{{ vm.downloadPath ?? 'OS user download folder' }}
					</p>
				</button>
			</div>
		</div>
	</div>
</template>