<script setup lang="ts">
import { TauriVM } from '../vue_lib/helper/ParamsHelper';
import { PropType } from 'vue';

const props = defineProps({
	vm: {
		type: Object as PropType<TauriVM>,
		required: true
	}
});

const emits = defineEmits(['invertVisibility', 'clearSending']);

const pluralize = (n: number, s: string) => n === 1 ? s : `${s}s`;
</script>

<template>
	<aside v-if="props.vm.outboundPayload === undefined" class="sidebar-panel w-72 px-1 py-3">
		<div class="px-3">
			<p class="text-xs font-semibold uppercase tracking-[0.12em] text-muted mb-3">
				Receiving
			</p>
			<button type="button" class="btn btn-secondary w-full !justify-between" @click="emits('invertVisibility')">
				<span v-if="props.vm.visibility === 'Visible'">Visible to everyone</span>
				<span v-else-if="props.vm.visibility === 'Invisible'">Hidden</span>
				<span v-else>Visible for 1 minute</span>

				<svg
					xmlns="http://www.w3.org/2000/svg" height="22" viewBox="0 -960 960 960" width="22"
					:class="{'rotate-180': props.vm.visibility === 'Invisible'}">
					<path d="M504-480 320-664l56-56 240 240-240 240-56-56 184-184Z" />
				</svg>
			</button>

			<p class="text-xs text-muted leading-5 mt-3">
				<span v-if="props.vm.visibility === 'Visible'">
					Nearby devices can find this computer. Every incoming transfer still requires your approval.
				</span>
				<span v-else-if="props.vm.visibility === 'Invisible'">
					This computer is not advertised to nearby devices. Previously paired devices may still attempt to connect.
				</span>
				<span v-else>
					This computer is temporarily discoverable to nearby devices.
				</span>
			</p>
		</div>
	</aside>

	<aside v-else class="sidebar-panel w-72 px-4 py-3 flex flex-col justify-between">
		<div>
			<p class="text-xs font-semibold uppercase tracking-[0.12em] text-muted mb-3">
				Sending
			</p>
			<template v-if="'Files' in props.vm.outboundPayload">
				<p class="font-semibold">
					{{ props.vm.outboundPayload.Files.length }} {{ pluralize(props.vm.outboundPayload.Files.length, "file") }}
				</p>
			</template>
			<template v-else>
				<p class="font-semibold">Sharing text</p>
			</template>

			<div class="icon-surface w-24 h-24 rounded-2xl my-4 flex justify-center items-center">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					height="24" viewBox="0 -960 960 960" width="24"
					class="w-8 h-8">
					<!-- eslint-disable-next-line -->
					<path d="M240-80q-33 0-56.5-23.5T160-160v-640q0-33 23.5-56.5T240-880h320l240 240v480q0 33-23.5 56.5T720-80H240Zm280-520v-200H240v640h480v-440H520ZM240-800v200-200 640-640Z" />
				</svg>
			</div>

			<template v-if="'Files' in props.vm.outboundPayload">
				<p
					v-for="f in props.vm.outboundPayload.Files"
					:key="f"
					class="overflow-hidden whitespace-nowrap text-ellipsis text-sm">
					{{ f.split('/').pop() }}
				</p>
			</template>
			<p
				v-else
				class="text-sm text-muted whitespace-pre-wrap break-words line-clamp-6">
				{{ props.vm.outboundPayload.Text }}
			</p>

			<p class="text-xs text-muted leading-5 mt-4">
				Keep both devices unlocked and nearby with Bluetooth enabled. The receiving device must have Quick Share enabled.
			</p>
		</div>

		<button type="button" class="btn btn-secondary w-fit" @click="emits('clearSending')">
			Cancel
		</button>
	</aside>
</template>