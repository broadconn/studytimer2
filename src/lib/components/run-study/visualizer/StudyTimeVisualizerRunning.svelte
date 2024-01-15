<script lang="ts">
	export let perc = 0;
	export let hideProgress = false;
</script>

<div class="h-full w-full p-0.5">
	<div
		id="gradient"
		class="progress-bar　h-full flex-grow rounded-2xl border border-sky-400 border-opacity-50 bg-gradient-to-r from-green-400 to-blue-700 text-white"
		style="width: {hideProgress ? 100 : perc * 100}%"
	></div>
</div>

<style>
	#gradient {
		background-size: 200% 200%;
		animation: Animation 3s ease infinite;
	}

	@keyframes Animation {
		0% {
			background-position: 10% 0%;
		}
		50% {
			background-position: 91% 100%;
		}
		100% {
			background-position: 10% 0%;
		}
	}

	.progress-bar {
		box-shadow: 20px 0 30px 0 rgb(0, 142, 236);
	}
</style>
