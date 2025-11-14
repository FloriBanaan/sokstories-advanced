<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { mousePosition } from './store.js';
    import { passive } from "svelte/legacy";

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
        initStory();
    }

    onMount(() => {
        ctx = canvas.getContext("2d", {willReadFrequently:true});
        fetchStory();
    });

    async function fetchStory() {
        storyString = await invoke("fetch_story", {id:"JNLA"});
        story = JSON.parse(storyString);
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
        console.log("even de sprites inladen");
        for (let i=0; i < story["objects"].length; i++) {
            let spriteBase64 = "data:image/png;base64," + story["objects"][i]["img"];
            var sprite = new Image();
            sprite.src = spriteBase64;
            story["objects"][i]["img"] = sprite;
            sprite.onload = function() {
                calculateHitboxes(i, sprite);
            };
        }
        console.log("klaar");
    }

    function calculateHitboxes(index, image)
    {
        ctx.clearRect(0, 0, 200, 200);
        ctx.drawImage(story["objects"][index]["img"], 0, 0);
        var imageData = ctx.getImageData(0, 0, 200, 200);
        story["objects"][index]["mask"] = {};
        story["objects"][index]["bbox_left"] = image.width;
        story["objects"][index]["bbox_top"] = image.height;
        story["objects"][index]["bbox_right"] = 0;
        story["objects"][index]["bbox_bottom"] = 0;
        
        for (var xp = 0; xp < image.width; xp++)
            for (var yp = 0; yp < image.height; yp++)
            {
                var i = xp * 4 + yp * image.width * 4;
                if (imageData.data[i + 3] > 10)
                {
                    story["objects"][index]["mask"][xp + yp * image.width] = true;
                    story["objects"][index]["bbox_left"] = Math.min(xp, story["objects"][index]["bbox_left"]);
                    story["objects"][index]["bbox_top"] = Math.min(yp, story["objects"][index]["bbox_top"]);
                    story["objects"][index]["bbox_right"] = Math.max(xp, story["objects"][index]["bbox_right"]);
                    story["objects"][index]["bbox_bottom"] = Math.max(yp, story["objects"][index]["bbox_bottom"]);
                }
                else
                    story["objects"][index]["mask"][xp + yp * image.width] = false;
            }
    }

    function drawObjectsToCanvas() {
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        for (let i=0; i < instances[currentRoom].length; i++) {
            let object = instances[currentRoom][i];
            let objectId = object["id"];
            let objectSprite = story["objects"][objectId]["img"];
            ctx.drawImage(objectSprite, object["posx"] -100, object["posy"] -100);
        }
    }

    function initStory() {
        initInstances();
        loadSprites();
        drawObjectsToCanvas();
    }

    function dragObject() {
        console.log("start dragging");
        for (let i=0; i < instances[currentRoom].length; i++) {
            let xp = instances[currentRoom][i]["posx"];
            let yp = instances[currentRoom][i]["posy"];
            let objectId = instances[currentRoom][i]["id"];
            let object = story["objects"][objectId];
            if (mouseRelativePosition()["x"] > xp - 100 + object["bbox_left"] && mouseRelativePosition()["y"] > yp - 100 + object["bbox_top"] && mouseRelativePosition()["x"] < xp - 100 + object["bbox_right"] && mouseRelativePosition()["y"] < yp - 100 + object["bbox_bottom"]) {
                // if (object["mobility"] === "static") {
                //     console.log("not draggable!");
                //     console.log(i);
                //     // return;
                // }
                // else {
                if (checkIfTransparent(objectId, i)) {
                    // return;
                    if (story["objects"][objectId]["mobility"] === "movable") {
                        draggingObject = i;
                        dragging = true;
                        xDeviation = instances[currentRoom][draggingObject]["posx"] - mouseRelativePosition()["x"];
                        yDeviation = instances[currentRoom][draggingObject]["posy"] - mouseRelativePosition()["y"];
                    }
                    else {
                        console.log("not draggable");
                        draggingObject = -1;
                        dragging = false;
                    }
                }
            }
        }
        // console.log("niet gevonden");

        // if (checkIfTransparent(object, index)) {
        //     console.log("hoi");
        //     dragging = true;
        //     draggingObject = index;

        // }
    }

    function stopDragging() {
        dragging = false;
        draggingObject = -1;
        console.log("stop dragging");
    }

    function onMove(event) {
        if(dragging) {
            instances[currentRoom][draggingObject]["posx"] = mouseRelativePosition()["x"] + xDeviation;
            instances[currentRoom][draggingObject]["posy"] = mouseRelativePosition()["y"] + yDeviation;
            drawObjectsToCanvas();
        }
    }

    function checkIfTransparent(objectId, index) {
        // console.log("object geklikt");
        // console.log(index);
        var r = 10;
        var mx = mouseRelativePosition()["x"] - (instances[currentRoom][index]["posx"] - 100);
        var my = mouseRelativePosition()["y"] - (instances[currentRoom][index]["posy"] - 100);
        for (var xxx = mx -r; xxx < mx + r; xxx++) {
            for (var yyy = my -r; yyy < my + r; yyy++)
            {
                if (check_pixel(objectId, xxx, yyy))
                {
                    console.log("draggable");
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
        console.log("not draggable");
        return false;
    }
    
    function check_pixel(index, x, y) {
        return story["objects"][Number(index)]["mask"][Math.floor(x) + Math.floor(y) * story["objects"][Number(index)]["img"].width];
    }

    function mouseRelativePosition() {
        let canvasx = canvas.getBoundingClientRect().x;
        let canvasy = canvas.getBoundingClientRect().y;
        return {"x" : $mouse.x - canvasx, "y" : $mouse.y - canvasy};
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

<svelte:window onmouseup={stopDragging} onmousemove={onMove} />

<main>
    
    <!-- <iframe src='https://sok-stories.com/?JNLA?embed' width='200' height='200'></iframe> -->
    <div id="canvasContainer">
        <canvas bind:this={canvas} id="storyCanvas" width="700" height="500" onmousedown={dragObject}></canvas>
    </div>
    <p>{storyString}</p>
    <p>{story}</p>
    <p>{story["unique_id"]}</p>
    
</main>