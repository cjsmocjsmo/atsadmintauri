<script>
	import img from '../Images/test400.webp';
</script>

<body>
	<div class="syncBtn">
		<button>SyncDB</button>
	</div>
	<!-- <h1>Estimates</h1> -->
	<div class="crap">
		<div class="imgDiv">
			<img src={img} alt="" />
		</div>
		<div class="boo">
			
			<div class="boo2">
				<h1>505-555-1313</h1>
				<h2>Herman Munster</h2>
				<h3>Adress: 1313 Mocking Bird Ln</h3>
				<h3>Email: herman@gmail.com</h3>
				<div>
					<button class="acctBtn">Completed</button>
					<!-- <button class="rejBtn">Reject</button> -->
				</div>
			</div>

			<div class="comDiv">
				<h1>Comment</h1>
				<p>
					this is a very long commentthis is a very long commentthis is a very long comment this is
					a very long comment this is a very long comment
				</p>
			</div>
		</div>
		
	</div>
	<div class="crap">
		<div class="imgDiv">
			<img src={img} alt="" />
		</div>
		<div class="boo">
			
			<div class="boo2">
				<h1>505-555-1313</h1>
				<h2>Herman Munster</h2>
				<h3>1313 Mocking Bird Ln</h3>
				<h3>herman@gmail.com</h3>
				<div>
					<button class="acctBtn">Completed</button>
					<!-- <button class="rejBtn">Reject</button> -->
				</div>
			</div>

			<div class="comDiv">
				<h1>Comment</h1>
				<p>
					this is a very long commentthis is a very long commentthis is a very long comment this is
					a very long comment this is a very long comment
				</p>
			</div>
		</div>
		
	</div>
</body>

<style>
	.syncBtn {
		text-align: center;
	}
	.comDiv {
		margin-right: 4em;
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
