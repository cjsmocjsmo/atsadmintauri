<script>
</script>

<body>
	<!-- <h1>Reviews</h1> -->

	<div class="revDiv">
		<div>
            <h1>boo@gmail.com</h1>
            <h1>5 stars</h1>
			<p class="revDate">Date: 02/02/2003</p>
			
			<div>
				<button class="acctBtn">Accept</button>
				<button class="rejBtn">Reject</button>
			</div>
		</div>
		<div class="revTxt">
			<p>
				Review: this is a Review: this is a Review: this is a really long reiviewReview: this is a
				really long reiviewReview: this is a really long reiview really long reiview really long
				reiview
			</p>
		</div>
	</div>
    <div class="revDiv">
		<div>
            <h1>boo@gmail.com</h1>
            <h1>5 stars</h1>
			<p class="revDate">Date: 02/02/2003</p>
			
			<div>
				<button class="acctBtn">Accept</button>
				<button class="rejBtn">Reject</button>
			</div>
		</div>
		<div class="revTxt">
			<p>
				Review: this is a Review: this is a Review: this is a really long reiviewReview: this is a
				really long reiviewReview: this is a really long reiview really long reiview really long
				reiview
			</p>
		</div>
	</div>
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
