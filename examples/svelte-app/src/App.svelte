<script lang="ts">
	import { connect, disconnect, sendAndReceive } from 'tauri-plugin-comm-api'

  let request = "";
  let response = "";

  const _connect = () => {
    connect()
      .catch(e => console.log(e));
  }

  const _disconnect = () => {
    disconnect()
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

</script>

<main class="container">

<div>
	<button on:click="{_connect}">connect</button>
	<button on:click="{_disconnect}">disconnect</button>
  <form on:submit|preventDefault={_sendAndReceive}>
      <input type="text" bind:value={request} placeholder="request">
      <button type="submit">sendAndReceive</button>
      <p>response: {response}</p>
  </form>
</div>

</main>
