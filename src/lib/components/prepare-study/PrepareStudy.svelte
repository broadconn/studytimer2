<script lang="ts">
	import type { StudyItem } from '$lib/domain/StudyItem';
	import { createEventDispatcher } from 'svelte';
	import StudySetup from './StudySetup.svelte';
	import { EventStrings } from '$lib/utils/shared-variables';
	import { UIChangeEventWriter } from '$lib/utils/store';

	const emitEvent = createEventDispatcher();

	let showProgrammes = false;
	$: {
		UIChangeEventWriter.setShowingProgrammes(showProgrammes); // automatically gets called when showProgrammes changes
	}
	$: max_width_class = showProgrammes ? 'max-w-3xl' : 'max-w-sm';

	function handleStartStudyEvent(e: CustomEvent<Array<StudyItem>>) {
		// just pass it on again up to the main page
		emitEvent(EventStrings.startStudyEvent, e.detail);
	}

	function handleProgrammesUIChangeEvent(e: CustomEvent<boolean>) {
		showProgrammes = e.detail;
	}
</script>

<div class="mx-auto flex h-screen flex-row {max_width_class}">
	<div class="flex-grow">
		<StudySetup
			on:startStudyEvent={handleStartStudyEvent}
			on:programmesBtnClickedEvent={handleProgrammesUIChangeEvent}
		/>
	</div>
</div>
