<script>
	import { invoke } from '@tauri-apps/api/tauri';
	


	export let data;
	let email = '';
	let password = '';
	let signedIn = '';

	async function signin() {
		let bar = data["foo"]
		console.log(bar)
		console.log(email)
		console.log(password)
		
		signedIn = await invoke('sign_in', {email, password, bar})
		console.log("this is signedIn")
		console.log(signedIn)
		if (signedIn) {
			window.location.href = '/estimates'
		} else {
			alert('Invalid email or password');
		}
		email = '';
		password = '';
	}


</script>

<body>
	<div class="card">
		<div class="form">
			<h3>Admin Login</h3>
			<div class="input-field">
				<i class="fa fa-envelope" />
				<input type="text" placeholder="Enter your email" bind:value="{email}"/>
			</div>
			<div class="input-field">
				<i class="fa fa-lock" />
				<input type="password" placeholder="Enter your password" bind:value="{password}"/>
			</div>
			<p />
			<p />
			<button on:click="{signin}">Login</button>
			<p />
			<p />
		</div>
		<div class="image">
			<div class="overlay">
				<h3>We Cut Trees<br />Alpha Tree Service</h3>
				<!-- <p>let us cut one for you</p> -->
			</div>
		</div>
	</div>
</body>

<style>

	body {
		font-family: 'Poppins', sans-serif;
		height: 100vh;
		width: 100vw;
		background-color: #7227d5;
		display: flex;
		justify-content: center;
		align-items: center;
	}

	.card {
		background: #fff;
		width: 900px;
		min-height: 400px;
		box-shadow: rgba(50, 50, 93, 0.25) 0px 6px 12px -2px, rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
		display: flex;
	}

	.form,
	.image {
		width: 50%;
	}

	.image {
		background-image: url('./Images/test400.webp');
		background-size: cover;
		background-position: center;
	}

	.overlay {
		width: 100%;
		height: 100%;
		background-color: rgba(114, 39, 213, 0.31);
		display: flex;
		justify-content: center;
		align-items: center;
		flex-direction: column;
		text-align: center;
	}

	.overlay h3 {
		color: white;
		letter-spacing: 1px;
		font-size: 25px;
		font-weight: 900;
		opacity: 0.6;
	}
	/* .overlay p {
		color: white;
		font-size: 16px;
		font-weight: bold;
		opacity: 0.6;
	} */

	.form {
		padding: 60px 25px;
		display: flex;
		flex-direction: column;
	}
	.form h3 {
		font-size: 28px;
		font-weight: 500;
		position: relative;
		margin-bottom: 30px;
	}
	.form h3::after {
		content: '';
		width: 30px;
		height: 3px;
		background: #7227d5;
		position: absolute;
		left: 0;
		bottom: 2px;
		border-radius: 5px;
	}

	.fa {
		color: #7227d5;
		position: absolute;
		top: 10px;
		font-size: 18px;
	}

	.input-field {
		width: 100%;
		margin-bottom: 10px;
		position: relative;
	}
	.input-field input {
		display: block;
		width: 100%;
		padding: 10px 30px;
		outline: none;
		border: none;
		border-bottom: 2px solid rgb(182, 180, 180);
		font-size: 15px;
	}

	button {
		height: 45px;
		background: #7227d5;
		border: none;
		color: white;
		border-radius: 5px;
		font-size: 18px;
	}

	button + p {
		text-align: center;
		padding-top: 30px;
		font-size: 14px;
	}

	input::placeholder {
		font-family: 'Poppins', sans-serif;
	}
</style>
