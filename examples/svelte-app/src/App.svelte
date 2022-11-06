<script lang="ts">
  import { onDestroy } from 'svelte';
	import { connectTimeout, disconnect, sendAndReceive } from 'tauri-plugin-comm-api'

  let request = "";
  let response = "";
  let isConnected = false;
  let addr = "";
  const TIMEOUT_MS = 1000;

  const _connect = () => {
    connectTimeout(addr, TIMEOUT_MS)
      .then(() => {
        isConnected = true;
      })
      .catch(e => console.log(e));
  }

  const _disconnect = () => {
    disconnect()
      .then(() => {
        isConnected = false;
      })
      .catch(e => console.log(e));
  }

  const _sendAndReceive = () => {
    sendAndReceive(request)
      .then(res => {
        response = res;
      })
      .catch(err => {
        response = err;
        console.log(err);
        isConnected = false;
      });
  }

  onDestroy(() => {
    disconnect().catch(e => console.log(e));
  });
</script>

<main class="container">

<div>
  <form on:submit|preventDefault={_connect}>
      <input type="text" bind:value={addr} placeholder="127.0.0.1:5555" disabled='{isConnected}'>
      <button type="submit" disabled='{isConnected}'>connect</button>
  </form>

	<button on:click="{_disconnect}" disabled='{!isConnected}'>disconnect</button>

  <form on:submit|preventDefault={_sendAndReceive}>
      <input type="text" bind:value={request} placeholder="request" disabled='{!isConnected}'>
      <button type="submit" disabled='{!isConnected}'>sendAndReceive</button>
      <p>response: {response}</p>
  </form>
</div>

</main>
