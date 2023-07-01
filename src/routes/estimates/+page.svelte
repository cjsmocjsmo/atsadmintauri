<script>
	import img from '../Images/test400.webp';
	import Header from '../Header.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let estimates = "";
	onMount(async () => {
		const ests = await invoke('all_estimates', {});
		estimates = ests;
		console.log(estimates);
	}, [])

	async function complete_est(estid) {
		const complete = await invoke('complete_estimate', {estid});
		console.log("completing estimate");
	}


</script>

<body>
	<Header />

	{#each estimates as estimate}
		<div class="crap">
			<div class="imgDiv">
				<img src={img} alt="" />
			</div>
			<div class="boo">
				
				<div class="boo2">
					<h1>{estimate.phone}</h1>
					<h2>{estimate.name}</h2>
					<h3>{estimate.addr}</h3>
					<h3>{estimate.city} WA</h3>
					<h3>{estimate.email}</h3>
					<div>
						<button class="completeBtn" on:click={() => complete_est(estimate.estid)}>Completed</button>
					</div>
				</div>

				<div class="comDiv">
					<h1>Comment</h1>
					<p>
						{estimate.comment}
					</p>
				</div>
			</div>
			
		</div>
		
	{/each}

</body>

<style>
	.comDiv {
		margin-right: 4em;
	}

	.completeBtn {
		background-color: green;
		padding: 1em;
		margin: 1em;
		border-radius: 0.5em;
		color: white;
		border-style: solid;
		border-color: grey;
		border-width: 0.25em;
		border-radius: 1.5em;
	}

	.crap {
		margin: 2em;
		border-color: blue;
		border-style: solid;
		border-width: 0.25em;
		border-radius: 1em;
		padding: 1em;
	}

	h3 {
		font-size: 1rem;
		text-align: center;
	}

	h2 {
		font-size: 1.5rem;
		text-align: center;
		font-weight: 600;
	}

	.boo2 {
		width: 100em;
	}

	.boo {
		display: flex;
		flex-direction: row;
		align-items: space-evenly;
		justify-content: center;
	}
	.imgDiv {
		width: 100%;
	}
	img {
		display: block;
		margin-left: auto;
		margin-right: auto;
		max-width: 300px;
		border-color: black;
		border-style: solid;
		border-width: 0.25em;
		border-radius: 1em;
	}
</style>
