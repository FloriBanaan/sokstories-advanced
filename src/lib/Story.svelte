<script>
// @ts-nocheck

    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    import { mousePosition } from './store.js';
    import { passive } from "svelte/legacy";
    import Page from "../routes/+page.svelte";

    let storyString = "";
    let story = [];
    let instances = [];
    let found = false;
    let currentRoom = 0;
    let dragging = false;
    let selectedObject = -1;
    let xDeviation = 0;
    let yDeviation = 0;
    let mouse = mousePosition();
    let clicking = false;
    let tapTimer = 0;
	let tapDuration = 250;
    let empty = -4;

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

    $: if (tapTimer === tapDuration) {
        // console.log("taptimer klaar");
        clicking = false;
    }

    onMount(() => {
        ctx = canvas.getContext("2d", {willReadFrequently:true});
        fetchStory();

        let last_time = performance.now();

		let frame = requestAnimationFrame(function update(time) {
			frame = requestAnimationFrame(update);

			tapTimer += Math.min(time - last_time, tapDuration - tapTimer);
			last_time = time;
		});
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

    function selectObject() {
        clicking = true;
        tapTimer = 0;
        for (let i=0; i < instances[currentRoom].length; i++) {
            let objectId = instances[currentRoom][i]["id"];
            let xp = instances[currentRoom][i]["posx"];
            let yp = instances[currentRoom][i]["posy"];
            let object = story["objects"][objectId];
            if (mouseRelativePosition()["x"] > xp - 100 + object["bbox_left"] && mouseRelativePosition()["y"] > yp - 100 + object["bbox_top"] && mouseRelativePosition()["x"] < xp - 100 + object["bbox_right"] && mouseRelativePosition()["y"] < yp - 100 + object["bbox_bottom"]) {
                if (checkIfTransparent(objectId, i)) {
                    selectedObject = i;
                    if (story["objects"][objectId]["mobility"] === "movable") {
                        dragging = true;
                        xDeviation = instances[currentRoom][selectedObject]["posx"] - mouseRelativePosition()["x"];
                        yDeviation = instances[currentRoom][selectedObject]["posy"] - mouseRelativePosition()["y"];
                    }
                }
            }
        }
    }

    function onMouseUp() {
        if (selectedObject === -1) {
            return;
        }
        if (tapTimer < tapDuration) {
            // console.log("op tijd losgelaten");
            console.log("geselecteerd object: " + selectedObject);
            checkOnClickRules();
        }
        dragging = false;
        selectedObject = -1;
    }

    function checkOnClickRules() {
        let objectId = instances[currentRoom][selectedObject]["id"];
        for (let i=0; i < story["transitions"].length; i++) {
            let transition = story["transitions"][i];
            if ((objectId === transition["pos1"] && transition["pos2"] === empty && transition["condition"] === "click") || (objectId === transition["pos2"] && transition["pos1"] === empty && transition["condition"] === "click")) {
                console.log("regel gevonden");
                doTransitionRule(i);
            }
        }
    }

    function doTransitionRule(index) {
        let transition = story["transitions"][index];
        if (transition["goto"] === "next") {
            currentRoom += 1;
            if (currentRoom === story["rooms"].length) {
                currentRoom = 0;
            }
            console.log("op naar de volgende kamer");
        }
        else if (transition["goto"] === "prev") {
            currentRoom -= 1;
            if (currentRoom < 0) {
                currentRoom = story["rooms"].length - 1;
            }
            console.log("op naar de vorige kamer");
        }
        else if (transition["goto"] != empty) {
            currentRoom = transition["goto"];
        }
        else {
            return;
        }
        
        // TODO: take left/right

        // console.log(currentRoom);
        drawObjectsToCanvas();
    }

    function onMove(event) {
        if(dragging) {
            instances[currentRoom][selectedObject]["posx"] = mouseRelativePosition()["x"] + xDeviation;
            instances[currentRoom][selectedObject]["posx"] = clamp(instances[currentRoom][selectedObject]["posx"], 100 - story["objects"][instances[currentRoom][selectedObject]["id"]]["bbox_left"], 700 + 100 - story["objects"][instances[currentRoom][selectedObject]["id"]]["bbox_right"]);
            instances[currentRoom][selectedObject]["posy"] = mouseRelativePosition()["y"] + yDeviation;
            instances[currentRoom][selectedObject]["posy"] = clamp(instances[currentRoom][selectedObject]["posy"], 100 - story["objects"][instances[currentRoom][selectedObject]["id"]]["bbox_top"], 500 + 100 - story["objects"][instances[currentRoom][selectedObject]["id"]]["bbox_bottom"]);
            drawObjectsToCanvas();
        }
    }

    function clamp(x, min, max)
    {
        return Math.min(Math.max(x, min), max);
    }

    function checkIfTransparent(objectId, index) {
        var r = 10;
        var mx = mouseRelativePosition()["x"] - (instances[currentRoom][index]["posx"] - 100);
        var my = mouseRelativePosition()["y"] - (instances[currentRoom][index]["posy"] - 100);
        for (var xxx = mx -r; xxx < mx + r; xxx++) {
            for (var yyy = my -r; yyy < my + r; yyy++)
            {
                if (check_pixel(objectId, xxx, yyy))
                {
                    return true;
                }
            }
        }
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
    }
    #canvasContainer {
    }
</style>

<svelte:window onmouseup={onMouseUp} onmousemove={onMove} />

<main>
    
    <!-- <iframe src='https://sok-stories.com/?JNLA?embed' width='200' height='200'></iframe> -->
    <div id="canvasContainer">
        <canvas bind:this={canvas} id="storyCanvas" width="700" height="500" onmousedown={selectObject}></canvas>
    </div>
    <p>{storyString}</p>
    <p>{story}</p>
    <p>{story["unique_id"]}</p>
    
</main>