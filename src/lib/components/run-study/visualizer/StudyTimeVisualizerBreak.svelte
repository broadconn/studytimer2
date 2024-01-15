<script lang="ts">
	export let perc = 0;
</script>

<div class="h-full w-full p-0.5">
	<div
		class="progress-bar h-full flex-grow rounded-2xl border border-red-600 border-opacity-40 bg-gradient-to-r from-yellow-500 to-red-400 text-white"
		style="width: {perc * 100}%"
	></div>
</div>

<style>
	.progress-bar {
		box-shadow: 20px 0 30px 0 rgb(236, 55, 0);
	}
</style>
