<script lang="ts">
	import { TimeSpan } from '$lib/utils/time-span';
	import {
		CloseOutline,
		Add,
		BookOutline,
		Book,
		HourglassOutline,
		PlayOutline,
		Repeat,
		ReorderTwo
	} from 'svelte-ionicons';
	import { createEventDispatcher } from 'svelte';
	import { StudyItem } from '$lib/domain/StudyItem';
	import { UIChangeEventWriter } from '$lib/utils/store';
	import StudyProgrammes from './StudyProgrammes.svelte';
	import dayjs from 'dayjs';
	import { slide } from 'svelte/transition';

	const emitEvent = createEventDispatcher();

	let newStudyItemName = '';
	let timerTimeSpan = TimeSpan.fromMinutes(30);
	let breakTimerTimeSpan = TimeSpan.fromMinutes(5);
	let totalStudyTime = TimeSpan.fromMinutes(0);
	let timeStudyEnds = dayjs();
	let studyItems = new Array<StudyItem>();
	let repetitions = 0;

	let showingProgrammes = false;

	let showStudyListQueue = false;
	$: {
		UIChangeEventWriter.setShowingStudyListQueue(showStudyListQueue); // automatically gets called when showStudyListQueue changes
	}

	function addStudyItem(itemName: string, time: TimeSpan): void {
		if (time.totalSeconds == 0) {
			console.log('Ignoring addStudyItem request because time is 0');
			return;
		}
		studyItems.push(new StudyItem(itemName, time));
		studyItems = studyItems; // the assignment refreshes the UI
		setTotalStudyTime();
		showStudyListQueue = studyItems.length > 0;
	}

	function removeStudyItem(studyItem: StudyItem): void {
		studyItems = studyItems.filter((item) => item !== studyItem);
		setTotalStudyTime();
		showStudyListQueue = studyItems.length > 0;
	}

	function setTotalStudyTime() {
		totalStudyTime = TimeSpan.fromMinutes(0);
		for (let studyItem of studyItems) {
			totalStudyTime = totalStudyTime.add(studyItem.timeSpan);
		}
		totalStudyTime = TimeSpan.fromSeconds(totalStudyTime.totalSeconds * (repetitions + 1));

		// add breaks
		let breakLength = TimeSpan.fromSeconds(
			breakTimerTimeSpan.totalSeconds * (studyItems.length * (repetitions + 1) - 1)
		);
		totalStudyTime = totalStudyTime.add(breakLength);

		timeStudyEnds = dayjs().add(totalStudyTime.totalSeconds, 'second');
	}

	function handleStudyTimerScrollWheel(event: { deltaY: number }, timeComponent: string) {
		let scrollDir = event.deltaY * -1; // invert scroll value sign for natural feels
		let addSeconds = timeComponentToSeconds(timeComponent);
		if (timeComponent == 'm' && timerTimeSpan.totalMinutes >= 10) addSeconds = 300; // if the timer is more than 10 minutes, scroll in 5 minute increments
		let val = Math.sign(scrollDir) * addSeconds;
		modifyTimeSpan(val);
	}

	function timeComponentToSeconds(timeComponent: string): number {
		switch (timeComponent) {
			case 'h':
				return 3600;
			case 'm':
				return 60;
			case 's':
				return 10;
			default:
				return 0;
		}
	}

	function modifyTimeSpan(secondsToAdd: number) {
		if (timerTimeSpan.totalSeconds + secondsToAdd < 0) {
			secondsToAdd = -timerTimeSpan.totalSeconds; // prevent negative time
		}
		timerTimeSpan = timerTimeSpan.add(TimeSpan.fromSeconds(secondsToAdd));
	}

	function handleBreakTimerScrollWheel(event: { deltaY: number }, timeComponent: string) {
		let scrollDir = event.deltaY * -1; // invert scroll value sign for natural feels
		let addSeconds = timeComponentToSeconds(timeComponent);
		let val = Math.sign(scrollDir) * addSeconds;
		modifyBreakTimeSpan(val);
		setTotalStudyTime();
	}

	function modifyBreakTimeSpan(secondsToAdd: number) {
		if (breakTimerTimeSpan.totalSeconds + secondsToAdd < 0) {
			secondsToAdd = -breakTimerTimeSpan.totalSeconds; // prevent negative time
		}
		breakTimerTimeSpan = breakTimerTimeSpan.add(TimeSpan.fromSeconds(secondsToAdd));
	}

	function onStudyNameInput(key: KeyboardEvent) {
		if (key.key === 'Enter') {
			addStudyItem(newStudyItemName, timerTimeSpan);
			newStudyItemName = '';
		}
	}

	function canQuickStart() {
		return studyItems.length == 0 && timerTimeSpan.totalSeconds > 0;
	}

	function onClickStartStudy() {
		if (canQuickStart()) {
			addStudyItem(newStudyItemName, timerTimeSpan);
		}

		if (studyItems.length == 0) {
			console.log('Ignoring startStudy request because studyItems is empty');
			return;
		}

		// duplicate the study items by the number of repetitions
		let studyItemsWithReps = new Array<StudyItem>();
		for (let i = 0; i <= repetitions; i++) {
			for (let studyItem of studyItems) {
				studyItemsWithReps.push(studyItem);
			}
		}

		// insert breaks between each item
		console.log('splicing breaks');
		console.log(studyItemsWithReps);
		let i = 0;
		while (i < studyItemsWithReps.length - 1) {
			studyItemsWithReps.splice(i + 1, 0, new StudyItem('Break', breakTimerTimeSpan));
			i += 2; // increment by 2 because we're adding an element after each item
		}
		console.log(studyItemsWithReps);

		console.log('starting');
		emitEvent('startStudyEvent', studyItemsWithReps);
	}

	function onClickProgrammesBtn(
		_event: MouseEvent & { currentTarget: EventTarget & HTMLButtonElement }
	) {
		console.log('Sending tauri_window_state_change event~');
		showingProgrammes = !showingProgrammes;
		emitEvent('programmesBtnClickedEvent', showingProgrammes);
	}

	function handleEnqueueStudyItemsEvent(e: CustomEvent<StudyItem[]>) {
		let studyItems = e.detail;
		for (let studyItem of studyItems) {
			addStudyItem(studyItem.name, timerTimeSpan);
		}
	}

	function onQueuedItemDragStart(_: DragEvent) {
		console.log('drag start');
	}

	function onQueuedItemDragOver(_: DragEvent) {
		console.log('drag over');
	}

	function onQueuedItemDragEnd(_: DragEvent) {
		console.log('drag end');
	}
</script>

<div class="flex flex-row">
	{#if showingProgrammes}
		<div class="w-full flex-grow">
			<StudyProgrammes on:enqueueStudyItemsEvent={handleEnqueueStudyItemsEvent} />
		</div>
	{/if}

	<div
		data-tauri-drag-region
		class="mx-auto flex h-screen w-full max-w-sm flex-grow flex-col gap-2 p-2 transition-transform duration-1000"
	>
		<!-- List of added study items -->
		{#if showStudyListQueue || showingProgrammes}
			<div class="flex flex-grow flex-col overflow-y-auto">
				<div
					data-tauri-drag-region
					class="relative mt-1 flex-grow overflow-y-auto rounded-l-3xl border-2 border-dashed border-gray-500 bg-slate-200 bg-opacity-5 p-1"
				>
					<ul class="flex flex-col gap-1">
						{#each studyItems as studyItem}
							<!-- Each individual study item -->
							<li
								class="queued-study-item flex items-center gap-2 rounded-l-2xl border border-solid border-white border-opacity-20 bg-black bg-opacity-25 py-2 pl-1 pr-4"
								draggable="true"
								on:dragstart={(e) => onQueuedItemDragStart(e)}
								on:dragover={(e) => onQueuedItemDragOver(e)}
								on:dragend={(e) => onQueuedItemDragEnd(e)}
							>
								<div class="ml-1 cursor-grab text-white text-opacity-50"><ReorderTwo /></div>
								<div class="text-white">
									<div class="text-lg">{studyItem.name}</div>
									<div class="text-xs opacity-50">{studyItem.timeSpan.toNiceTimerString()}</div>
								</div>
								<div class="ml-auto">
									<button
										class="transition-bg-opacity flex h-8 w-8 items-center justify-center rounded-md text-white opacity-100 duration-75 ease-in hover:bg-slate-600 focus:outline-none focus:ring-0"
										on:click={() => removeStudyItem(studyItem)}
									>
										<CloseOutline />
									</button>
								</div>
							</li>
						{/each}
					</ul>
				</div>
			</div>
		{/if}

		<div class="rounded-xl border border-white border-opacity-10">
			<!-- Multi-study settings -->
			{#if studyItems.length > 1}
				<div class="flex h-12 w-full flex-row" transition:slide={{ duration: 200 }}>
					<!-- Breaks time bar -->
					<div data-tauri-drag-region class="flex flex-grow justify-start">
						<div data-tauri-drag-region class="flex items-center">
							<div class="ml-2 flex flex-grow items-center">
								<p class="flex flex-col text-white">Breaks</p>
							</div>
							<div class="ml-1 text-white"><HourglassOutline /></div>
							<div class="my-1 ml-1 flex text-center">
								<div
									class="rounded-l-full bg-white bg-opacity-10 py-1 pl-3 pr-1 text-xl text-white"
									on:wheel={(event) => handleBreakTimerScrollWheel(event, 'm')}
								>
									<p>{breakTimerTimeSpan.minutes}</p>
								</div>
								<div
									class="rounded-r-full bg-white bg-opacity-10 py-1 pl-1 pr-3 text-xl text-white"
									on:wheel={(event) => handleBreakTimerScrollWheel(event, 's')}
								>
									<p>{breakTimerTimeSpan.secondsStr}</p>
								</div>
							</div>
						</div>
					</div>
					<!-- Repetitions input -->
					<div class=" flex w-24 flex-row items-center gap-1 rounded-tl-xl px-1">
						<div class="w-12 flex-grow py-1 pl-1 text-2xl">
							<input
								class="w-full rounded-xl bg-white bg-opacity-10 px-2 text-2xl text-white"
								type="number"
								min="0"
								bind:value={repetitions}
								on:change={() => setTotalStudyTime()}
							/>
						</div>
						<div
							class="bottom-2 right-0 flex flex-row items-center justify-center gap-1 text-lg text-white opacity-50"
						>
							<Repeat class="pointer-events-none  h-8 w-8" />
						</div>
					</div>
				</div>
			{/if}
			<!-- New study item setup box -->
			<div
				data-tauri-drag-region
				class="flex flex-col items-center justify-center gap-2 rounded-xl bg-white bg-opacity-5 p-2 shadow-xl shadow-slate-900"
			>
				<!-- Top half - study item name input -->
				{#if !showingProgrammes}
					<div class="flex w-full flex-grow items-center justify-end gap-2">
						<input
							class="h-auto w-full rounded-lg bg-white bg-opacity-10 px-5 py-3 text-lg text-white"
							placeholder="Enter study name"
							bind:value={newStudyItemName}
							on:keydown={onStudyNameInput}
						/>
						<button
							class="my-2 rounded-full border border-opacity-80 p-1 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:outline-0 focus:ring-0"
							on:click={() => addStudyItem(newStudyItemName, timerTimeSpan)}
						>
							<Add />
						</button>
					</div>
				{/if}
				<!-- Bottom half -->
				<div class="flex h-full w-full">
					<!-- Timer input -->
					<div data-tauri-drag-region class="flex items-center">
						<div class="ml-1 text-white"><HourglassOutline /></div>
						<div class="ml-1 flex text-center">
							<div
								class="h-12 w-11 rounded-l-full bg-white bg-opacity-10 pt-2 text-2xl text-white"
								on:wheel={(event) => handleStudyTimerScrollWheel(event, 'h')}
							>
								<p>{timerTimeSpan.hoursStr}</p>
							</div>
							<div
								class="h-12 w-11 rounded-md bg-white bg-opacity-10 pt-2 text-2xl text-white"
								on:wheel={(event) => handleStudyTimerScrollWheel(event, 'm')}
							>
								<p>{timerTimeSpan.minutesStr}</p>
							</div>
							<div
								class="h-12 w-11 rounded-r-full bg-white bg-opacity-10 pt-2 text-2xl text-white"
								on:wheel={(event) => handleStudyTimerScrollWheel(event, 's')}
							>
								<p>{timerTimeSpan.secondsStr}</p>
							</div>
						</div>
					</div>
					<!-- Empty space -->
					<div data-tauri-drag-region class="w-5 flex-1"></div>
					<!-- Buttons -->
					<div data-tauri-drag-region class="flex justify-end gap-2">
						<div class="flex aspect-square items-center justify-center">
							<button
								class="rounded-full border border-opacity-80 p-2 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
								on:click={onClickProgrammesBtn}
							>
								<span class="flex items-center justify-center">
									{#if showingProgrammes}
										<Book />
									{:else}
										<BookOutline />
									{/if}
								</span>
							</button>
						</div>
						<div class="aspect-square">
							<button
								class="group relative h-full w-full overflow-hidden rounded-full border border-opacity-80 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
								on:click={() => onClickStartStudy()}
							>
								<span class="flex items-center justify-center">
									<PlayOutline />
								</span>
							</button>
						</div>
					</div>
				</div>
			</div>
			{#if studyItems.length > 1}
				<div
					class="flex w-full flex-row justify-center gap-1 px-2 py-1"
					transition:slide={{ duration: 200 }}
				>
					<div data-tauri-drag-region class="text-xs text-white text-opacity-50">
						Time study ends: {timeStudyEnds.format('h:mm A')}
					</div>
					<div
						data-tauri-drag-region
						class="flex-grow items-end justify-end text-end text-xs text-white text-opacity-50"
					>
						{studyItems.length === 0
							? timerTimeSpan.toNiceTimerString()
							: totalStudyTime.toNiceTimerString()}
					</div>
				</div>
			{/if}
		</div>
	</div>
</div>

<style>
	.queued-study-item {
		animation: studyItemAddAnimation 300ms ease-out forwards;

		/* Prestate */
		opacity: 0;
		transform: translateY(-20px);
	}

	@keyframes studyItemAddAnimation {
		100% {
			opacity: 1;
			transform: none;
		}
	}

	::-webkit-scrollbar {
		width: 10px;
	}

	/* Track */
	::-webkit-scrollbar-track {
		background: #00000000;
	}

	/* Handle */
	::-webkit-scrollbar-thumb {
		background: #ffffff83;
		border-radius: 15px;
		border: 2px solid #26295f;
	}

	/* Handle on hover */
	::-webkit-scrollbar-thumb:hover {
		background: #a3e0f0;
	}
</style>
