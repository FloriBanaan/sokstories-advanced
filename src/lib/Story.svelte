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

    let canvas;
    let ctx;
    
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

    $: if (found) {
        // console.log(story);
        initStory();
    }

    onMount(() => {
        ctx = canvas.getContext("2d", {willReadFrequently:true});
        window.addEventListener('mousemove', onMove);
        fetchStory();
        // console.log("klaar");
        // console.log(story);
    });

    async function fetchStory() {
        // console.log("even fetchen");
        storyString = await invoke("fetch_story", {id:"JNLA"});
        story = JSON.parse(storyString);
        // console.log("klaar met fetchen");
        // console.log(story);
        found = true;
    }

    function initInstances() {
        for (let i=0; i < story["rooms"].length; i++) {
            instances.push([]);
            for (let j=0; j < story["rooms"][i]["objects"].length; j++) {
                let object = story["rooms"][i]["objects"][j];
                instances[i].push({"id":object["id"], "posx":object["posx"], "posy":object.posy})
            }
        }
    }

    function loadSprites() {
        // console.log("even de sprites inladen");
        // console.log(story);
        // console.log(story["objects"]);
        for (let i=0; i < story["objects"].length; i++) {
            // console.log("we zitten in de loop");
            let spriteBase64 = "data:image/png;base64," + story["objects"][i]["img"];
            var sprite = new Image();
            sprite.src = spriteBase64;
            story["objects"][i]["img"] = sprite;
            // console.log(story["objects"][i]["img"]);
            sprite.onload = function() {
                calculateHitboxes(i, sprite);
            };
        }
    }

    function calculateHitboxes(index, image)
    {
        ctx.clearRect(0, 0, 200, 200);
        ctx.drawImage(story["objects"][index]["img"], 0, 0);
        var imageData = ctx.getImageData(0, 0, 200, 200);
        image.mask = [];
        image.bbox_left = image.width;
        image.bbox_top = image.height;
        image.bbox_right = 0;
        image.bbox_bottom = 0;
        console.log(index);
        for (var xp = 0; xp < image.width; xp++)
            for (var yp = 0; yp < image.height; yp++)
            {
                var i = xp * 4 + yp * image.width * 4;

                // console.log("varken");
                // console.log("hoi");

                // console.log(imageData.data[i + 3]);

                if (imageData.data[i + 3] > 10)
                {
                    image.mask[xp + yp * image.width] = true;
                    image.bbox_left = Math.min(xp, image.bbox_left);
                    image.bbox_top = Math.min(yp, image.bbox_top);
                    image.bbox_right = Math.max(xp, image.bbox_right);
                    image.bbox_bottom = Math.max(yp, image.bbox_bottom);
                }
                else
                    image.mask[xp + yp * image.width] = false;
            }
        

        
        // loaded_objects++;
        // if (loaded_objects == objects - failed_loads)
        // {
        //     calculate_rule_matrix();
        //     loop_interval = setInterval(loop, 1000 / fps);
        // }
    }

    function drawObjectsToCanvas() {
        console.log("hoi");
        for (let i=0; i < instances[currentRoom].length; i++) {
            console.log(i);
            let object = instances[currentRoom][i];
            let objectId = object["id"];
            let objectSprite = story["objects"][objectId]["img"];
            // console.log(objectSprite);
            // console.log(image);
            console.log(object["posx"]);
            console.log(object["posy"]);
            ctx.drawImage(objectSprite, object["posx"] -100, object["posy"] -100);
            // console.log(ctx);
        }
    }

    function initStory() {
        initInstances();
        loadSprites();
        // drawObjectsToCanvas();
    }

    function dragObject() {
        console.log("drag");
        // if (story["objects"][object["id"]]["mobility"] === "static") {
        //     console.log("not draggable!");
        //     return;
        // }
        // if (checkIfTransparent(object, index)) {
        //     console.log("hoi");
        //     dragging = true;
        //     draggingObject = index;
        //     xDeviation = instances[currentRoom][draggingObject]["posx"] - $mouse.x;
        //     yDeviation = instances[currentRoom][draggingObject]["posy"] - $mouse.y;
        // }
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
        border: 4px solid black;
        /* background-color: blue; */
    }
    #canvasContainer {
        /* background-color: red; */
    }
</style>

<main>
    <!-- <iframe src='https://sok-stories.com/?JNLA?embed' width='200' height='200'></iframe> -->
    <div id="canvasContainer">
        <!-- {#if found} -->
        <canvas bind:this={canvas} id="storyCanvas" width="700" height="500" onmousedown={dragObject}></canvas>
        <!-- {/if} -->
    </div>
    <p>{storyString}</p>
    <p>{story}</p>
    <p>{story["unique_id"]}</p>
    
</main>