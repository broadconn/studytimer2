<script lang="ts">
	import { createEventDispatcher, onMount } from 'svelte';
	import { Store } from 'tauri-plugin-store-api';
	import { Close, ArrowDown, ArrowForward, Add, ReorderTwo, DiceOutline } from 'svelte-ionicons';
	import { flip } from 'svelte/animate';
	import { slide } from 'svelte/transition';

	const emitEvent = createEventDispatcher();

	const store = new Store('.settings.dat');
	const storeProgrammesKey = 'study-plans';

	let categoriesInputStr = '';
	let programmesInputStr = '';
	let studyItemsInputStr = '';
	let categoryInputsCollapsed = true;

	let elementBeingDragged: HTMLElement | null = null;
	let studyItemBeingDragged: StudyItem | null = null;
	let programmeBeingDragged: Programme | null = null;
	let categoryBeingDragged: Category | null = null;
	let categoryBeingDraggedCurIdx: number = -1;

	class StudyData {
		categories: Array<Category> = new Array<Category>();
	}

	class Category {
		name: string = '';
		isCollapsed: boolean = false;
		programmes: Array<Programme> = new Array<Programme>();
		id: string = '';

		constructor(name: string, id: string) {
			this.name = name;
			this.id = id;
		}
	}

	class Programme {
		name: string = '';
		id: string = '';
		isCollapsed: boolean = false;
		studyItems: Array<StudyItem> = new Array<StudyItem>();
		categoryname: string = '';

		constructor(name: string, categoryname: string, id: string) {
			this.name = name;
			this.id = id;
			this.categoryname = categoryname;
		}
	}

	class StudyItem {
		name: string = '';
		categoryName: string = '';
		programmeName: string = '';
		id: string = '';

		constructor(name: string, categoryName: string, programmeName: string, id: string) {
			this.name = name;
			this.categoryName = categoryName;
			this.programmeName = programmeName;
			this.id = id;
		}
	}

	let studyData: StudyData = new StudyData();

	let mouseUpHandler = () => {
		onMouseUp();
	};

	function onMouseUp() {
		releaseStudyItem();
		releaseProgramme();
		releaseCategory();
	}

	onMount(async () => {
		await loadStudyData();
		window.addEventListener('mouseup', mouseUpHandler);
	});

	function refreshListUI() {
		studyData = studyData; // the assignment signals the UI to refresh anything using studyData
	}

	async function loadStudyData() {
		const val = await store.get(storeProgrammesKey);
		if (val) {
			studyData = (val as { value: StudyData }).value;
		}
	}

	async function save() {
		await store.set(storeProgrammesKey, { value: studyData });
		await store.save(); // apparently this is automatic on close but we can save excessively for peace of mindo
	}

	function onClickCollapseCategoryBtn(c: Category) {
		c.isCollapsed = !c.isCollapsed;

		// collapse all other categories
		studyData.categories.forEach((cat) => {
			if (c.name !== cat.name) {
				cat.isCollapsed = true;
			}
		});

		// collapse all programmes on open or close
		c.programmes.forEach((p) => {
			p.isCollapsed = c.programmes.length > 1;
		});

		refreshListUI();
		save();
	}

	function onClickCollapseProgrammeBtn(p: Programme) {
		p.isCollapsed = !p.isCollapsed;
		refreshListUI();
		save();
	}

	function onClickEnqueueStudyItemBtn(studyItem: StudyItem) {
		let studyItems = new Array<StudyItem>();
		studyItems.push(studyItem);

		emitEvent('enqueueStudyItemsEvent', studyItems);
	}

	function onClickEnqueueProgrammeBtn(categoryName: string, programmeName: string) {
		let category = studyData.categories.find((c) => c.name === categoryName);
		if (!category) return;
		let programme = category.programmes.find((p) => p.name === programmeName);
		if (!programme) return;

		emitEvent('enqueueStudyItemsEvent', programme.studyItems);
	}

	function onClickAddToCategoryBtn(category: string) {
		categoryInputsCollapsed = false;
		categoriesInputStr = category;
		programmesInputStr = '';
		studyItemsInputStr = '';
	}

	function onClickAddToProgrammeBtn(category: string, programme: string) {
		categoryInputsCollapsed = false;
		categoriesInputStr = category;
		programmesInputStr = programme;
		studyItemsInputStr = '';
	}

	function toggleCategoryInputsCollpased() {
		categoryInputsCollapsed = !categoryInputsCollapsed;
	}

	function onClickCreateStudyItemBtn() {
		if (categoriesInputStr === '' || programmesInputStr === '' || studyItemsInputStr === '') return;

		let newStudyItem = new StudyItem(
			studyItemsInputStr,
			categoriesInputStr,
			programmesInputStr,
			crypto.randomUUID()
		);

		let category = studyData.categories.find((c) => c.name === categoriesInputStr);
		if (!category) {
			category = new Category(categoriesInputStr, crypto.randomUUID());
			studyData.categories.push(category);
		}

		let programme = category.programmes.find((p) => p.name === programmesInputStr);
		if (!programme) {
			programme = new Programme(programmesInputStr, category.name, crypto.randomUUID());
			category.programmes.push(programme);
		}

		programme.studyItems.push(newStudyItem);

		category.isCollapsed = false;
		programme.isCollapsed = false;

		// collapse all other categories
		studyData.categories.forEach((cat) => {
			if (cat.name !== categoriesInputStr) {
				cat.isCollapsed = true;
			}
		});

		save();

		studyItemsInputStr = '';
		refreshListUI();
	}

	function onClickRemoveStudyItemBtn(studyItem: StudyItem) {
		let category = studyData.categories.find((c) => c.name === studyItem.categoryName);
		if (!category) return;
		let programme = category.programmes.find((p) => p.name === studyItem.programmeName);
		if (!programme) return;
		programme.studyItems = programme.studyItems.filter((si) => si.name !== studyItem.name);

		if (programme.studyItems.length === 0) {
			category.programmes = category.programmes.filter((p) => p.name !== studyItem.programmeName);
		}

		if (category.programmes.length === 0) {
			studyData.categories = studyData.categories.filter((c) => c.name !== studyItem.categoryName);
		}

		refreshListUI();
		save();
	}

	function onGrabStudyItemHandle(event: MouseEvent, studyItem: StudyItem): any {
		let element: HTMLElement | null = event.currentTarget as HTMLElement;

		elementBeingDragged = element;
		studyItemBeingDragged = studyItem;
	}

	function onGrabProgrammeHandle(event: MouseEvent, programme: Programme): any {
		let element: HTMLElement | null = event.currentTarget as HTMLElement;

		elementBeingDragged = element;
		programmeBeingDragged = programme;
	}

	function onGrabCategoryHandle(event: MouseEvent, category: Category): any {
		let element: HTMLElement | null = event.currentTarget as HTMLElement;

		elementBeingDragged = element;
		categoryBeingDragged = category;
		categoryBeingDraggedCurIdx = studyData.categories.findIndex((c) => c === categoryBeingDragged);
	}

	function dragEnterStudyItem(event: MouseEvent, programme: Programme, enteredIndex: number) {
		event.stopPropagation();
		if (!elementBeingDragged || studyItemBeingDragged?.programmeName != programme.name) return;

		let elementEntered: HTMLElement | null = event.currentTarget as HTMLElement;
		if (elementEntered != elementBeingDragged && elementEntered.classList.contains('study-item')) {
			let indexDraggedFrom = programme.studyItems.findIndex((si) => si === studyItemBeingDragged);
			programme.studyItems = swapItems(programme.studyItems, indexDraggedFrom, enteredIndex);
			refreshListUI();
		}
	}

	function dragEnterProgramme(event: MouseEvent, categoryToStayIn: Category, enteredIndex: number) {
		event.stopPropagation();
		if (!elementBeingDragged || programmeBeingDragged?.categoryname != categoryToStayIn.name)
			return;

		let elementEntered: HTMLElement | null = event.currentTarget as HTMLElement;
		if (
			elementEntered != elementBeingDragged &&
			elementEntered.classList.contains('programme-title')
		) {
			let indexDraggedFrom = categoryToStayIn.programmes.findIndex(
				(p) => p === programmeBeingDragged
			);
			categoryToStayIn.programmes = swapItems(
				categoryToStayIn.programmes,
				indexDraggedFrom,
				enteredIndex
			);
			refreshListUI();
		}
	}

	function dragEnterCategory(event: MouseEvent, enteredIndex: number) {
		if (!categoryBeingDragged) return;

		event.stopPropagation();
		let elementEntered: HTMLElement | null = event.currentTarget as HTMLElement;
		if (
			elementEntered != elementBeingDragged &&
			elementEntered.classList.contains('category-title')
		) {
			studyData.categories = swapItems(
				studyData.categories,
				categoryBeingDraggedCurIdx,
				enteredIndex
			);
			categoryBeingDraggedCurIdx = enteredIndex;
			refreshListUI();
		}
	}

	function swapItems(list: any[], fromIdx: number, toIdx: number) {
		let temp = list[fromIdx];
		list = [...list.slice(0, fromIdx), ...list.slice(fromIdx + 1)];
		list = [...list.slice(0, toIdx), temp, ...list.slice(toIdx)];
		return list;
	}

	function releaseStudyItem() {
		elementBeingDragged = null;
		studyItemBeingDragged = null;
	}

	function releaseProgramme() {
		elementBeingDragged = null;
		programmeBeingDragged = null;
	}

	function releaseCategory() {
		elementBeingDragged = null;
		categoryBeingDragged = null;
	}

	function enqueueRandomProgrammeItem(programme: Programme) {
		let studyItems = new Array<StudyItem>();
		let randomIndex = Math.floor(Math.random() * programme.studyItems.length);
		studyItems.push(programme.studyItems[randomIndex]);
		emitEvent('enqueueStudyItemsEvent', studyItems);
	}
</script>

<div class="flex h-screen flex-col py-2 pl-2">
	<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
	<div
		data-tauri-drag-region
		class="flex-grow items-center justify-center overflow-y-auto text-white"
		on:mouseup={function (event) {
			event.stopPropagation();
			onMouseUp();
		}}
		role="list"
	>
		<!-- list parent -->
		<div data-tauri-drag-region class="flex h-full w-full flex-grow flex-col gap-1">
			{#each studyData.categories as category, index (category.id)}
				<!-- categories -->
				<div class="rounded-lg border border-slate-700">
					<!-- category titlebar -->
					<div
						class="category-title mr-1 flex flex-row items-center justify-center"
						on:mouseenter={(event) => dragEnterCategory(event, index)}
						role="list"
					>
						<button
							class="flex w-full cursor-pointer items-center justify-center rounded-lg rounded-t-lg bg-white bg-opacity-0 text-left text-xl font-bold"
							on:click={() => {
								onClickCollapseCategoryBtn(category);
							}}
						>
							<div
								class="ml-2 cursor-grab text-white opacity-50 transition-opacity duration-200"
								on:mousedown={(event) => onGrabCategoryHandle(event, category)}
								role="button"
								tabindex="0"
							>
								<ReorderTwo />
							</div>
							<div class="flex-grow px-2 py-0.5 text-lg">{category.name}</div>
						</button>
						<button
							class="category-btn float-right rounded-full border border-opacity-80 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
							on:click={() => {
								onClickAddToCategoryBtn(category.name);
							}}
						>
							<Add />
						</button>
					</div>
					<!-- category content -->
					{#if !category.isCollapsed}
						<div transition:slide={{ duration: 200 }} class="flex flex-col gap-1">
							{#each category.programmes as programme, index (programme.id)}
								<div
									data-tauri-drag-region
									class="programme-list ml-3 mr-1 flex flex-col rounded-md border border-white border-opacity-5"
								>
									<!-- programme title bar -->
									<div
										class="programme-title flex flex-row items-center justify-center gap-2 rounded{!programme.isCollapsed
											? '-t'
											: ''}-md bg-white bg-opacity-10 py-0"
										on:mouseenter={(event) => dragEnterProgramme(event, category, index)}
										role="list"
									>
										<button
											class="flex w-full cursor-pointer flex-row rounded-lg rounded-t-lg bg-white bg-opacity-0 text-left text-xl font-bold"
											on:click={() => {
												onClickCollapseProgrammeBtn(programme);
											}}
										>
											<div
												class="ml-2 cursor-grab text-white opacity-50 transition-opacity duration-200"
												on:mousedown={(event) => onGrabProgrammeHandle(event, programme)}
												role="button"
												tabindex="0"
											>
												<ReorderTwo />
											</div>
											<div class="flex-grow px-2 py-0.5 text-base">
												{programme.name}
											</div>
										</button>
										<button
											class="programme-btn float-right rounded-full border border-opacity-80 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
											on:click={() => {
												enqueueRandomProgrammeItem(programme);
											}}
										>
											<DiceOutline />
										</button>
										<button
											class="programme-btn float-right rounded-full border border-opacity-80 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
											on:click={() => {
												onClickAddToProgrammeBtn(category.name, programme.name);
											}}
										>
											<Add />
										</button>
										<button
											class="programme-btn float-right rounded-full border border-opacity-80 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
											on:click={() => {
												onClickEnqueueProgrammeBtn(category.name, programme.name);
											}}
										>
											<ArrowForward />
										</button>
									</div>
									<!-- programme content -->
									{#if !programme.isCollapsed}
										<div transition:slide={{ duration: 200 }}>
											{#each programme.studyItems as studyItem, index (studyItem.id)}
												<!-- svelte-ignore a11y-no-static-element-interactions -->
												<div
													class="study-item flex flex-row"
													on:mouseenter={(event) => dragEnterStudyItem(event, programme, index)}
													animate:flip={{ duration: 200 }}
												>
													<button
														class="study-item-content flex w-full flex-row rounded-full border border-white border-opacity-0 bg-white bg-opacity-0 text-base transition-all duration-300 hover:border-opacity-20 focus:outline-none focus:ring-0"
														on:dblclick={() => {
															onClickEnqueueStudyItemBtn(studyItem);
														}}
													>
														<div
															class="study-item-reorder-handle ml-2 cursor-grab text-white text-opacity-50"
															on:mousedown={(event) => onGrabStudyItemHandle(event, studyItem)}
															role="button"
															tabindex="0"
														>
															<ReorderTwo />
														</div>
														<button class="flex-grow text-left text-white">
															<div class="items-center pl-2 pt-0.5">{studyItem.name}</div>
														</button>
														<button
															class=" float-right rounded-full border text-white opacity-0 transition-all duration-300 hover:opacity-50"
															on:click={() => onClickRemoveStudyItemBtn(studyItem)}
														>
															<Close />
														</button>
													</button>
												</div>
											{/each}
										</div>
									{/if}
								</div>
							{/each}
						</div>
					{/if}
				</div>
			{/each}
		</div>
	</div>

	<!-- Input boxes -->
	{#if !categoryInputsCollapsed}
		<div
			class="rounded-t-xl border-t-2 border-white border-opacity-50 bg-white bg-opacity-5 p-2"
			transition:slide={{ duration: 200 }}
		>
			<div class="flex h-full flex-col gap-1 overflow-hidden">
				<div class="flex flex-col gap-1">
					<div class="flex flex-row gap-1">
						<!-- cateogory input -->
						<input
							class="h-full w-full rounded-lg bg-white bg-opacity-10 px-2 py-1 text-lg text-white"
							placeholder="Category"
							bind:value={categoriesInputStr}
						/>
						<!-- programme input -->
						<input
							class="h-full w-full rounded-lg bg-white bg-opacity-10 px-2 py-1 text-lg text-white"
							placeholder="Programme"
							bind:value={programmesInputStr}
						/>
					</div>
					<div class="flex flex-row gap-1">
						<!-- study item input -->
						<input
							class="h-full w-full rounded-lg bg-white bg-opacity-10 px-2 py-1 text-lg text-white"
							placeholder="Study Name"
							bind:value={studyItemsInputStr}
						/>
						<div class="flex flex-col items-end">
							<!-- add button -->
							<button
								class="h-full flex-grow rounded-full border border-opacity-80 p-1 px-5 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
								on:click={onClickCreateStudyItemBtn}
							>
								Create
							</button>
						</div>
					</div>
				</div>
			</div>
		</div>
	{/if}

	<!-- New category inputs toggler -->
	<button
		class="w-full rounded-lg border border-white border-opacity-20 bg-white bg-opacity-10 text-white transition-all duration-300 hover:bg-white hover:bg-opacity-50 focus:outline-none focus:ring-0"
		on:click={toggleCategoryInputsCollpased}
	>
		<div class="flex-grow items-center justify-center align-middle">
			{#if categoryInputsCollapsed}
				<Add />
			{:else}
				<ArrowDown />
			{/if}
		</div>
	</button>
</div>

<style>
	.category-title .category-btn {
		opacity: 0;
		transition-duration: 0s;
	}

	.category-title:hover .category-btn {
		opacity: 1;
	}

	.programme-title > .programme-btn {
		opacity: 0;
		transition-duration: 0s;
	}

	.programme-title:hover .programme-btn {
		opacity: 1;
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
