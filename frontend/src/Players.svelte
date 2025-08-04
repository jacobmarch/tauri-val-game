<script lang="ts">
  import { onMount } from 'svelte';

  interface Player {
    name: string;
  }

  let players: Player[] = [];

  onMount(async () => {
    const res = await fetch("http://localhost:8080/players");
    players = await res.json();
  });
</script>

<h2>Players</h2>
<ul>
  {#each players as player}
    <li>{player.name}</li>
  {/each}
</ul>

<a href="/">Back to Home</a>
