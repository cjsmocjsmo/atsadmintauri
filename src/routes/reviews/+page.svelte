<script>
	import Header from '../Header.svelte';
	import { onMount } from 'svelte';
	import { invoke } from '@tauri-apps/api/tauri';

	let reviews = "";

	onMount(async () => {
		const revs = await invoke('all_reviews', {});
		reviews = revs;
		console.log(reviews);
	}, [])

	async function accept_rev(revid) {
		const accept = await invoke('accept_review', {revid});
		console.log("accepting review");
	}

	async function reject_rev(revid) {
		const reject = await invoke('reject_review', {revid});
		console.log("rejecting review");
	}

	
	
</script>

<body>
	<Header />

	{#each reviews as review}
		<div class="revDiv">
			<div>
				<h1>{review.email}</h1>
				<h1>{review.stars} stars</h1>
				<p class="revDate">Date: {review.date}</p>
				
				<div>
					<button class="acctBtn" onclick={() => accept_rev(review.revid)}>Accept</button>
					<button class="rejBtn" onclick={() => reject_rev(review.revid)}>Reject</button>
				</div>
			</div>
			<div class="revTxt">
				<p>
					Review: {review.review}
				</p>
			</div>
		</div>
	{/each}

</body>

<style>
    .revDate {
        font-size: 1.5em;
        text-align: center;
    }
	.revTxt {
		width: 50%;
		padding: 1em;
	}

	.acctBtn {
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

    .rejBtn {
		background-color: red;
		padding: 1em;
		margin: 1em;
		border-radius: 0.5em;
		color: white;
		border-style: solid;
		border-color: grey;
		border-width: 0.25em;
		border-radius: 1.5em;
	}

	.revDiv {
		display: flex;
		justify-content: space-between;
		border: 0.25em solid blue;
        border-radius: 1em;
		padding: 1em;
		margin: 1em;
	}
</style>
