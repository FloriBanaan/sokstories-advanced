<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let storyString = "";
    let story = [];
    let found = false;
    
    //
    // icon: object_idx
    // story_name: string
    // unique_id: string
    // maker_name: string
    // maker_id: string
    // objects:
    //      mobility: "static"/"movable"
    //      img: base64
    // rules:
    //      pos1: object_idx
    //      pos2: object_idx
    //      pos3: object_idx
    //      pos4: object_idx
    //      pos5: obhect_idx
    //      condition: "click"/"tick"
    //      sound: sfx_idx
    // rooms:
    //      state: "persistent"/"restarts"
    //      objects:
    //          id: object_idx
    //          posx: float
    //          posy: float
    // transitions:
    //      pos1: object_idx
    //      pos2: object_idx
    //      condition: "click"/"tick"
    //      take: "left"/"right"/"none"
    //      goto: "next"/"prev"/room_idx
    //

    onMount(async () => {
        storyString = await invoke("fetch_story", {id:"JNLA"});
        story = JSON.parse(storyString);
        found = true;
    });
</script>

<style>
    #storyCanvas {
        position: relative;
        width: 700px;
        height: 500px;
        border: 4px solid black;
        /* background-color: red; */
    }
    #topleft {
        left: 0;
        top: 0;
    }
    .object {
        position: absolute;
    }
</style>

<main>
    <iframe src='https://sok-stories.com/?JNLA?embed' width='200' height='200'></iframe>
    <div id="storyCanvas">
        {#if found}
        {#each story["rooms"][0]["objects"] as object}
        <img class="object" style="left: {object["posx"] - 100}px; top: {object["posy"] - 100}px;" src={`data:image/png;base64,${story["objects"][object["id"]]["img"]}`} alt="object">
        <!-- <img class="object" style="left: 0px; top: 0px;" src={`data:image/png;base64,${story["objects"][object["id"]]["img"]}`} alt="object"> -->
        {/each}
        {/if}
    </div>
    <p>{storyString}</p>
    <p>{story}</p>
    <p>{story["unique_id"]}</p>
    
</main>