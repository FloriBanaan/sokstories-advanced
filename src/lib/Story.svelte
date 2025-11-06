<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { mousePosition } from './store.js';

    let storyString = "";
    let story = [];
    let instances = [];
    let found = false;
    let currentRoom = 0;
    let dragging = false;
    let draggingObject = -1;
    let xDeviation = 0;
    let yDeviation = 0;
    let mouse = mousePosition();
    
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
        window.addEventListener('mousemove', onMove);
        storyString = await invoke("fetch_story", {id:"JNLA"});
        story = JSON.parse(storyString);
        await initInstances();
        found = true;
    });

    function initInstances() {
        for (let i=0; i < story["rooms"].length; i++) {
            instances.push([]);
            for (let j=0; j < story["rooms"][i]["objects"].length; j++) {
                let object = story["rooms"][i]["objects"][j];
                instances[i].push({"id":object["id"], "posx":object["posx"], "posy":object.posy})
            }
        }
    }

    function dragObject(object, index) {
        if (story["objects"][object["id"]]["mobility"] === "static") {
            console.log("not draggable!");
            return;
        }
        if (checkIfTransparent(object, index)) {
            console.log("hoi");
            dragging = true;
            draggingObject = index;
            xDeviation = instances[currentRoom][draggingObject]["posx"] - $mouse.x;
            yDeviation = instances[currentRoom][draggingObject]["posy"] - $mouse.y;
        }
    }

    function stopDragging(object, index) {
        dragging = false;
        draggingObject = -1;
    }

    function onMove(event) {
        if(dragging) {
            instances[currentRoom][draggingObject]["posx"] = $mouse.x + xDeviation;
            instances[currentRoom][draggingObject]["posy"] = $mouse.y + yDeviation;
        }
    }

    function checkIfTransparent(object, index) {
        // pixel-perfect check (w/ radius)
        var r = 5;
        var mx = $mouse.x - (instances[currentRoom][index]["posx"] - 85);
        var my = $mouse.y - (instances[currentRoom][index]["posy"] - 35);
        console.log("mousex: " + $mouse.x);
        console.log("mousey: " + $mouse.y);
        console.log("spritex: " + instances[currentRoom][index]["posx"] - 100);
        console.log("spritey: " + instances[currentRoom][index]["posy"] - 100);
        console.log("mx: " + mx);
        console.log("my: " + my);
        // console.log(mx);
        // console.log(my);
        for (var xxx = mx -r; xxx < mx + r; xxx++) {
            for (var yyy = my -r; yyy < my + r; yyy++)
            {
                // console.log("banaan");
                // console.log(story["objects"][object["id"]]["img"]);
                // console.log(xxx);
                // console.log(yyy);
                if (check_pixel(story["objects"][object["id"]]["img"], xxx, yyy))
                {
                    console.log("nog steeds niet transparant");
                    return true;
                    // if (!object_is_static[object[current_room][i]])
                    //     hover_target = i;
                    // else
                    //     hover_target_static = i;
                    // xxx = 99999;
                    // yyy = 99999;
                }
            }
        }
        return false;
    }
    
    function check_pixel(spriteBase64, x, y) {

        const image = new Image();
        image.src = "data:image/png;base64," + spriteBase64;

        const canvas = document.createElement("canvas");
        const ctx = canvas.getContext("2d");
        canvas.width = image.width;
        canvas.height = image.height;

        ctx.drawImage(image, 0, 0);

        const pixel = ctx.getImageData(Math.floor(x), Math.floor(y), 1, 1).data;
        // pixel = [r, g, b, a]
        // resolve(pixel);
        const [r, g, b, a] = pixel;
        console.log(`R:${r} G:${g} B:${b} A:${a}`);
        if (a > 1) {
            console.log("niet transparant");
            return true;
        }
        return false;

    }

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
    <!-- <iframe src='https://sok-stories.com/?JNLA?embed' width='200' height='200'></iframe> -->
    <div id="storyCanvas">
        {#if found}
        {#each instances[currentRoom] as object, index}
        <img onmouseup={() => stopDragging(object, index)} onmousedown={() => dragObject(object, index)} class="object" style="left: {object["posx"] - 100}px; top: {object["posy"] - 100}px;" src={`data:image/png;base64,${story["objects"][object["id"]]["img"]}`} alt="object" draggable="false">
        <!-- <img class="object" style="left: 0px; top: 0px;" src={`data:image/png;base64,${story["objects"][object["id"]]["img"]}`} alt="object"> -->
        {/each}
        {/if}
    </div>
    <p>{storyString}</p>
    <p>{story}</p>
    <p>{story["unique_id"]}</p>
    
</main>