<script lang="ts">
	import '../app.postcss';
	import '@fortawesome/fontawesome-free/css/all.css';

	import {
		AppShell,
		type ModalComponent,
		Modal,
		Toast,
		popup,
		type PopupSettings
	} from '@skeletonlabs/skeleton';
	import { LightSwitch } from '@skeletonlabs/skeleton';
	import AddModal from './AddModal.svelte';

	// stores
	import { initializeStores } from '@skeletonlabs/skeleton';
	initializeStores();

	import { computePosition, autoUpdate, offset, shift, flip, arrow } from '@floating-ui/dom';
	import { storePopup } from '@skeletonlabs/skeleton';
	import SettingsPopup from './SettingsPopup.svelte';
	storePopup.set({ computePosition, autoUpdate, offset, shift, flip, arrow });

	const modalRegistry: Record<string, ModalComponent> = {
		addModal: { ref: AddModal }
	};

	const settings: PopupSettings = {
		event: 'click',
		target: 'settingsContents',
		placement: 'bottom',
		closeQuery: ''
	};
</script>

<Toast position="br" zIndex="z-[1000]" />
<Modal components={modalRegistry} />

<!-- App Shell -->
<AppShell>
	<svelte:fragment slot="pageHeader">
		<!-- Page Container -->
		<div class="page-container !max-w-7xl mx-auto grid grid-cols-[1fr_auto] items-center gap-4 p-4">
			<button type="button" class="btn-icon" use:popup={settings}>
				<img class="max-w-full rounded-sm aspect-square shadow-xl" src="./favicon.png" alt="shit" />
			</button>

			<div class="card p-4 w-72 shadow-xl z-[2]" data-popup="settingsContents">
				<SettingsPopup />
			</div>

			<LightSwitch
				class="bg-surface-50/50 dark:bg-surface-900/50 backdrop-blur-xl shadow-xl"
				ring="ring-none"
			/>
		</div>
	</svelte:fragment>
	<!-- Page Route Content -->
	<slot />
</AppShell>
