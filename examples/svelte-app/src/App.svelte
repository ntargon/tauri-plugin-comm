<script lang="ts">
  import { onDestroy } from 'svelte';
	import { connect, disconnect, sendAndReceive } from 'tauri-plugin-comm-api'

  let request = "";
  let response = "";
  let isConnected = false;

  const _connect = () => {
    connect()
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
      });
  }

  onDestroy(() => {
    disconnect().catch(e => console.log(e));
  });
</script>

<main class="container">

<div>
	<button on:click="{_connect}" disabled='{isConnected}'>connect</button>
	<button on:click="{_disconnect}" disabled='{!isConnected}'>disconnect</button>
  <form on:submit|preventDefault={_sendAndReceive}>
      <input type="text" bind:value={request} placeholder="request" disabled='{!isConnected}'>
      <button type="submit" disabled='{!isConnected}'>sendAndReceive</button>
      <p>response: {response}</p>
  </form>
</div>

</main>
