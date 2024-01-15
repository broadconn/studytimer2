<script lang="ts">
	import type { StudyItem } from '$lib/domain/StudyItem';
	import { createEventDispatcher, onMount } from 'svelte';
	import { studyPauseEventStore } from '$lib/utils/store';
	import { EventStrings } from '$lib/utils/shared-variables';
	import StudyTimeVisualizerRunning from './visualizer/StudyTimeVisualizerRunning.svelte';
	import StudyTimeVisualizerBreak from './visualizer/StudyTimeVisualizerBreak.svelte';

	export let studyItems: Array<StudyItem>;

	const dispatch = createEventDispatcher();

	let isPaused = false;
	let currentStudyItem = studyItems[0];
	let i = 0;
	let isBreak = false;
	let upNextString = '';
	let hideProgress = true;

	let timeInThisStudyItem = 0;
	let percThroughThisStudyItem = 0;
	let progressUpdateIntervalId: NodeJS.Timeout;

	onMount(() => {
		if (!studyItems || studyItems.length === 0) {
			endStudy();
			return;
		}

		startStudyItem(0);
	});

	function startStudyItem(val: number): void {
		i = val;
		currentStudyItem = studyItems[val];
		isBreak = currentStudyItem.name == 'Break';

		if (val + 1 < studyItems.length) {
			upNextString = studyItems[val + 1].name;
		} else {
			upNextString = '';
		}

		timeInThisStudyItem = 0;
		percThroughThisStudyItem = 0;
		restartProgressTracking();
	}

	function onEndStudyItem() {
		i++;
		if (i < studyItems.length) {
			startStudyItem(i);
		} else {
			endStudy();
		}
	}

	function restartProgressTracking() {
		const updateIntervalMillis = 50;
		clearInterval(progressUpdateIntervalId);
		progressUpdateIntervalId = setInterval(() => {
			if (isPaused) {
				return; // don't update time if paused
			} else {
				timeInThisStudyItem += updateIntervalMillis;
				percThroughThisStudyItem =
					timeInThisStudyItem / currentStudyItem.timeSpan.totalMilliseconds;
				if (percThroughThisStudyItem >= 1) {
					clearInterval(progressUpdateIntervalId);
					onEndStudyItem();
				}
			}
		}, updateIntervalMillis);
	}

	function endStudy() {
		clearInterval(progressUpdateIntervalId);
		dispatch(EventStrings.endStudyEvent);
	}

	function togglePause() {
		isPaused = !isPaused;

		console.log('sending pause event');
		studyPauseEventStore.set(isPaused);
	}

	function toggleProgHidden() {
		hideProgress = !hideProgress;
		console.log('toggleProgHidden' + hideProgress);
	}
</script>

<div class="flex h-full w-full flex-col bg-black">
	<div class="h-full w-full">
		{#if isBreak}
			<StudyTimeVisualizerBreak perc={percThroughThisStudyItem} />
		{:else}
			<button class="h-full w-full" on:click={toggleProgHidden}>
				<StudyTimeVisualizerRunning perc={percThroughThisStudyItem} {hideProgress} />
			</button>
		{/if}
	</div>

	<div
		class="pointer-events-none absolute inset-0 flex w-full items-center justify-center text-center"
	>
		<p class="pointer-events-none text-xs text-white">
			{currentStudyItem?.name + '' + (isBreak ? '! Next: ' + upNextString : '')}
		</p>
	</div>
</div>
