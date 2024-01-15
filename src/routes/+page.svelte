<script lang="ts">
	import { invoke } from '@tauri-apps/api/tauri';
	import { onMount } from 'svelte';
	import PrepareStudy from '../lib/components/prepare-study/PrepareStudy.svelte';
	import RunStudy from '../lib/components/run-study/RunStudy.svelte';
	import { TauriEventStrings } from '$lib/utils/shared-variables';
	import { UIChangeEventWriter, uiChangeEventStore } from '$lib/utils/store';
	import type { StudyItem } from '$lib/domain/StudyItem';

	let isStudying = false;
	$: {
		UIChangeEventWriter.setStudying(isStudying); // automatically gets called when isStudying changes
	}

	let studyItems: Array<StudyItem> = [];

	onMount(async () => {
		invoke(TauriEventStrings.onAppStart);

		const { appWindow } = await import('@tauri-apps/api/window');
		var closeBtn = document.getElementById('titlebar-close');
		closeBtn?.addEventListener('click', () => {
			appWindow.close();
		});

		uiChangeEventStore.subscribe((e) => {
			invoke(TauriEventStrings.onWindowStateChange, {
				studying: e.isStudying,
				list: e.isShowingStudyListQueue,
				programmes: e.isShowingProgrammes
			});
		});
	});

	function handlePrepareStudyEvent(e: CustomEvent<Array<StudyItem>>) {
		studyItems = e.detail;
		isStudying = true;
	}

	function handleEndStudyEvent() {
		isStudying = false;

		// HACK: slight delay to allow the UIChangeEventWriter.setStudying event to finish first
		setTimeout(() => {
			invoke(TauriEventStrings.onStudyEnded);
		}, 100);
	}
</script>

<div class="h-screen overflow-hidden bg-gray-800 bg-gradient-to-tr from-sky-950 to-gray-950">
	{#if !isStudying}
		<PrepareStudy on:startStudyEvent={handlePrepareStudyEvent} />
	{/if}
	{#if isStudying}
		<RunStudy {studyItems} on:endStudyEvent={handleEndStudyEvent} />
	{/if}
</div>
