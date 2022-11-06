<script lang="ts">
  import Greet from './lib/Greet.svelte'
	import { execute } from 'tauri-plugin-comm-api'
	import { invoke } from '@tauri-apps/api'

	let response = ''

	function updateResponse(returnValue) {
    console.log(returnValue);
		response += `[${new Date().toLocaleTimeString()}]` + (typeof returnValue === 'string' ? returnValue : JSON.stringify(returnValue)) + '<br>'
	}

	async function _execute() {
		execute().then(updateResponse).catch(updateResponse)
    let a = await invoke('plugin:comm|execute')
		console.log(a);

	}
</script>

<main class="container">

<div>
	<button on:click="{_execute}">Execute</button>
	<div>{@html response}</div>
</div>

</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>