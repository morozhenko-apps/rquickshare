<script setup lang="ts">
import { ref, watch } from 'vue';
import { parseListeningPort, utils } from '../vue_lib';
import { PropType } from 'vue';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emit = defineEmits(['close']);
const portInput = ref('');
const portError = ref('');
const portSaved = ref(false);

watch(
	() => props.vm.settingsOpen,
	async (isOpen) => {
		if (!isOpen) return;

		const savedPort = await props.vm.store.get<number>('port');
		portInput.value = savedPort?.toString() ?? '';
		portError.value = '';
		portSaved.value = false;
	},
	{ immediate: true }
);

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

async function onAutostartChange(event: Event) {
	const checked = (event.target as HTMLInputElement).checked;
	await utils.setAutoStart(props.vm, checked);
}

async function onKeepRunningChange(event: Event) {
	const checked = (event.target as HTMLInputElement).checked;
	await utils.setRealClose(props.vm, !checked);
}

async function onStartMinimizedChange(event: Event) {
	const checked = (event.target as HTMLInputElement).checked;
	await utils.setStartMinimized(props.vm, checked);
}

async function savePort() {
	const parsed = parseListeningPort(portInput.value);
	portError.value = parsed.error ?? '';
	portSaved.value = false;

	if (parsed.error) {
		return;
	}

	if (parsed.port === null) {
		await props.vm.store.delete('port');
	} else {
		await props.vm.store.set('port', parsed.port);
	}

	await props.vm.store.save();
	portSaved.value = true;
}
</script>

<template>
	<div v-if="vm.settingsOpen" class="modal-backdrop absolute inset-0 z-10 flex justify-center items-center p-6">
		<div class="modal-card rounded-2xl p-5 w-[30rem] max-w-full max-h-full overflow-y-auto">
			<div class="flex flex-row justify-between items-center gap-4">
				<div>
					<h2 class="font-semibold text-xl">\n\t\t\t\t\t\tSettings\n\t\t\t\t\t</h2>
					<p class="text-sm text-muted mt-1">\n\t\t\t\t\t\tGeneral behavior, networking and received files.\n\t\t\t\t\t</p>
				</div>
				<button type="button" class="btn btn-secondary" @click="emit('close')">
					Close
				</button>
			</div>

			<div class="pt-5 flex flex-col gap-2">
				<div class="setting-row rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center gap-4">
						<div>
							<p class="font-medium">\n\t\t\t\t\t\t\tStart on boot\n\t\t\t\t\t\t</p>
							<p class="text-xs text-muted mt-1">\n\t\t\t\t\t\t\tLaunch Quick Share when you sign in.\n\t\t\t\t\t\t</p>
						</div>
						<input
							type="checkbox"
							:checked="vm.autostart"
							class="checkbox focus:outline-none"
							@change="onAutostartChange">
					</label>
				</div>

				<div class="setting-row rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center gap-4">
						<div>
							<p class="font-medium">\n\t\t\t\t\t\t\tKeep running on close\n\t\t\t\t\t\t</p>
							<p class="text-xs text-muted mt-1">\n\t\t\t\t\t\t\tHide the window instead of stopping the service.\n\t\t\t\t\t\t</p>
						</div>
						<input
							type="checkbox"
							:checked="!vm.realclose"
							class="checkbox focus:outline-none"
							@change="onKeepRunningChange">
					</label>
				</div>

				<div class="setting-row rounded-xl p-3">
					<label class="cursor-pointer flex flex-row justify-between items-center gap-4">
						<div>
							<p class="font-medium">\n\t\t\t\t\t\t\tStart minimized\n\t\t\t\t\t\t</p>
							<p class="text-xs text-muted mt-1">\n\t\t\t\t\t\t\tOpen directly in the background.\n\t\t\t\t\t\t</p>
						</div>
						<input
							type="checkbox"
							:checked="vm.startminimized"
							class="checkbox focus:outline-none"
							@change="onStartMinimizedChange">
					</label>
				</div>

				<button type="button" class="setting-row rounded-xl p-3 text-left" @click="openDownloadPicker()">
					<p class="font-medium">\n\t\t\t\t\t\tDownload folder\n\t\t\t\t\t</p>
					<p class="overflow-hidden whitespace-nowrap text-ellipsis text-xs text-muted mt-1">
						{{ vm.downloadPath ?? 'OS user download folder' }}
					</p>
				</button>

				<div class="setting-row rounded-xl p-3">
					<div class="flex items-start justify-between gap-4">
						<div class="min-w-0 flex-1">
							<p class="font-medium">\n\t\t\t\t\t\t\tListening port\n\t\t\t\t\t\t</p>
							<p class="text-xs text-muted mt-1">
								Leave empty for an automatic random port. A fixed port is useful when a firewall is enabled.
							</p>
						</div>
						<input
							v-model="portInput"
							type="number"
							min="1024"
							max="65535"
							placeholder="Auto"
							class="text-input w-28"
							@input="portSaved = false"
							@keyup.enter="savePort">
					</div>
					<div class="flex items-center justify-between gap-3 mt-3">
						<p class="text-xs" :style="{ color: portError ? 'var(--rqs-danger)' : 'var(--rqs-text-muted)' }">
							<span v-if="portError">{{ portError }}</span>
							<span v-else-if="portSaved">Saved. Restart the app to apply.</span>
							<span v-else>Changes apply after restart.</span>
						</p>
						<button type="button" class="btn btn-secondary" @click="savePort">
							Save
						</button>
					</div>
				</div>
			</div>
		</div>
	</div>
</template>