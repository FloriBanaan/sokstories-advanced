<script>
    import { event } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let storyName = "";
    let makerName = "";
    let storyRange = 0;
    let stories = "";
    let makers = "";
    let newStories = "";

    onMount(async () => {
        fetchStoriesByRecency();
    });

    async function fetchStoriesByName() {
        stories = JSON.parse(await invoke("fetch_stories_by_name", {name:storyName}));
    }

    async function fetchMakersByName() {
        makers = JSON.parse(await invoke("fetch_makers_by_name", {name:makerName}))
    }

    async function fetchStoriesByRecency() {
        newStories = JSON.parse(await invoke("fetch_stories_by_recency", {min:storyRange, max:storyRange + 10}))
    }

    async function increaseStoryRange() {
        storyRange += 10;
        fetchStoriesByRecency();
    }

    async function decreaseStoryRange() {
        if (storyRange < 10) {
            console.log(storyRange);
            return;
        }
        storyRange -= 10;
        fetchStoriesByRecency();
    }

</script>

<style>

</style>

<main>
    <form class="row" onsubmit={fetchStoriesByName}>
        <input placeholder="Story name..." bind:value={storyName} />
        <button type="submit">Search</button>
    </form>

    <form class="row" onsubmit={fetchMakersByName}>
        <input placeholder="Maker name..." bind:value={makerName} />
        <button type="submit">Search</button>
    </form>

    <button onclick={decreaseStoryRange}> Older</button>
    <button onclick={increaseStoryRange}> Newer</button>
    

    {#each stories as story}
    <p>{story["story_name"]}</p>
    <img src={`data:image/png;base64,${story["icon"]}`} />
    {/each}

    {#each makers as maker}
    <p>{maker["maker_name"]}</p>
    {/each}

    {#each newStories as story}
    <p>{story["story_name"]}</p>
    <img src={`data:image/png;base64,${story["icon"]}`} />
    {/each}

</main>