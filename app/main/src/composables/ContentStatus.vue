<script setup lang="ts">
import { readText } from '@tauri-apps/plugin-clipboard-manager';
import { OutboundPayload } from '@martichou/core_lib/bindings/OutboundPayload';
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { PropType, ref } from 'vue';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emits = defineEmits(['outboundPayload', 'discoveryRunning']);
const clipboardError = ref<string>();

async function ensureDiscovery() {
	if (!props.vm.discoveryRunning) await props.vm.invoke('start_discovery');
	emits('discoveryRunning');
}

function openFilePicker() {
	props.vm.dialogOpen({
		title: "Select a file to send",
		directory: false,
		multiple: true,
	}).then(async (el) => {
		let elem;
		if (el === null) {
			return;
		}

		if (el instanceof Array) {
			if (el.length > 0 && Object.hasOwn(el[0], 'path')) {
				elem = el.map((e) => e.path);
			} else {
				elem = el;
			}
		} else {
			elem = [el];
		}

		emits('outboundPayload', {
			Files: elem
		} as OutboundPayload);
		await ensureDiscovery();
	})
}

async function shareClipboardText() {
	clipboardError.value = undefined;

	let text: string;
	try {
		text = await readText();
	} catch {
		clipboardError.value = 'Clipboard text is unavailable.';
		return;
	}

	if (text.trim().length === 0) {
		clipboardError.value = 'Clipboard does not contain text.';
		return;
	}

	emits('outboundPayload', { Text: text } as OutboundPayload);
	await ensureDiscovery();
}
</script>

<template>
	<div class="mb-5">
		<h2 class="font-semibold text-xl">
			<span v-if="props.vm.displayedIsEmpty">Ready to share</span>
			<span v-else>Nearby devices</span>
		</h2>
		<p class="text-sm text-muted mt-1">
			<span v-if="props.vm.displayedIsEmpty">Receive files or select something to send.</span>
			<span v-else>Select a device to start the transfer.</span>
		</p>
	</div>

	<div
		v-if="props.vm.displayedIsEmpty && props.vm.endpointsInfo.length === 0"
		class="m-auto status-indicator status-indicator--success status-indicator--xl"
		aria-label="Ready">
		<div class="circle circle--animated circle-main" />
		<div class="circle circle--animated circle-secondary" />
		<div class="circle circle--animated circle-tertiary" />
	</div>

	<div
		v-if="props.vm.displayedIsEmpty && props.vm.outboundPayload === undefined"
		class="drop-zone w-full rounded-2xl p-7 flex flex-col justify-center items-center mt-auto"
		:class="{'drop-zone--active': props.vm.isDragHovering}">
		<svg
			xmlns="http://www.w3.org/2000/svg"
			height="24" viewBox="0 -960 960 960" width="24"
			class="w-8 h-8">
			<!-- eslint-disable-next-line -->
			<path d="M440-320v-326L336-542l-56-58 200-200 200 200-56 58-104-104v326h-80ZM240-160q-33 0-56.5-23.5T160-240v-120h80v120h480v-120h80v120q0 33-23.5 56.5T720-160H240Z" />
		</svg>
		<h3 class="mt-3 font-semibold">
			Drop files here
		</h3>
		<p class="text-sm text-muted mt-1 mb-4">
			or choose files from disk
		</p>
		<div class="flex flex-wrap justify-center gap-2">
			<button type="button" class="btn btn-primary" @click="openFilePicker()">
				<svg xmlns="http://www.w3.org/2000/svg" height="20" viewBox="0 -960 960 960" width="20">
					<path d="M440-440H200v-80h240v-240h80v240h240v80H520v240h-80v-240Z" />
				</svg>
				<span>Select files</span>
			</button>
			<button type="button" class="btn btn-secondary" @click="shareClipboardText()">
				<svg xmlns="http://www.w3.org/2000/svg" height="20" viewBox="0 -960 960 960" width="20">
					<path d="M320-120q-33 0-56.5-23.5T240-200v-560q0-33 23.5-56.5T320-840h80q8-35 37-57.5t67-22.5q38 0 67 22.5t37 57.5h80q33 0 56.5 23.5T768-760v560q0 33-23.5 56.5T688-120H320Zm0-80h368v-560h-72v120H392v-120h-72v560Zm184-520q17 0 28.5-11.5T544-760q0-17-11.5-28.5T504-800q-17 0-28.5 11.5T464-760q0 17 11.5 28.5T504-720Z" />
				</svg>
				<span>Paste text</span>
			</button>
		</div>
		<p v-if="clipboardError" class="text-xs text-error mt-3">
			{{ clipboardError }}
		</p>
	</div>
</template>