<script>
	import { invoke } from "@tauri-apps/api/tauri";
	import { onMount } from "svelte";

	import "carbon-components-svelte/css/white.css";

	onMount(() => {
	    invoke('my_custom_command');
	});

</script>

<h1>Dashboard</h1>

<style>
h1 {
    color: #00d9ff;
    text-transform: uppercase;
    font-size: 4em;
    font-weight: 100;
}
</style>