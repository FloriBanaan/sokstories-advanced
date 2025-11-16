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
    let draggingObject = -1;
    let selectedObject = -1;
    let xDeviation = 0;
    let yDeviation = 0;
    let mouse = mousePosition();
    let clicking = false;
    let tapTimer = 0;
	let tapDuration = 250;
    let empty = -4;
    let loadedSprites = 0;

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

    $: if (loadedSprites > 0) {
        if (loadedSprites === story["objects"].length) {
            // console.log("klaar");
            setInterval(() => {
                doTick();
            }, 3000)
        }
    }


    $: if (tapTimer === tapDuration) {
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
                instances[i].push({"id":object["id"], "posx":object["posx"], "posy":object["posy"], "changed": false});
            }
        }
    }

    function loadSprites() {
        for (let i=0; i < story["objects"].length; i++) {
            let spriteBase64 = "data:image/png;base64," + story["objects"][i]["img"];
            var sprite = new Image();
            sprite.src = spriteBase64;
            story["objects"][i]["img"] = sprite;
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
        story["objects"][index]["mask"] = {};
        story["objects"][index]["bbox_left"] = image.width;
        story["objects"][index]["bbox_top"] = image.height;
        story["objects"][index]["bbox_right"] = 0;
        story["objects"][index]["bbox_bottom"] = 0;
        
        for (var xp = 0; xp < image.width; xp++) {
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
        loadedSprites++;
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
        // drawObjectsToCanvas();
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
                        draggingObject = i;
                        dragging = true;
                        xDeviation = instances[currentRoom][selectedObject]["posx"] - mouseRelativePosition()["x"];
                        yDeviation = instances[currentRoom][selectedObject]["posy"] - mouseRelativePosition()["y"];
                    }
                }
            }
        }
    }

    function onMouseUp() {
        dragging = false;
        if (selectedObject === -1) {
            return;
        }
        if (tapTimer < tapDuration) {
            checkOnClickRules();
        }
        if (draggingObject != -1) {
            checkOnClickCombineRules();
        }
        draggingObject = -1;
        selectedObject = -1;
    }

    function checkOnClickCombineRules() {
        let objectId = instances[currentRoom][draggingObject]["id"];
        for (let i=0; i < story["rules"].length; i++) {
            let rule = story["rules"][i];
            let otherPosId = empty;
            let selectedObjectPos = 0;
            if (rule["pos1"] === objectId && rule["pos2"] != empty && rule["condition"] === "click") {
                otherPosId = rule["pos2"];
                selectedObjectPos = 1;
            }
            else if (rule["pos1"] != empty && rule["pos2"] === objectId && rule["condition"] === "click") {
                otherPosId = rule["pos1"];
                selectedObjectPos = 2;
            }
            if (otherPosId != empty) {
                for (let j=0; j < instances[currentRoom].length; j++) {
                    let instance = instances[currentRoom][j];
                    if (instance["id"] === otherPosId && j != draggingObject) {
                        if(checkCollision(draggingObject, j)) {
                            let r = getRandomRule(rule["pos1"], rule["pos2"]);
                            if (selectedObjectPos === 1) {
                                doRule(r, draggingObject, j);
                            }
                            else {
                                doRule(r, j, draggingObject);
                            }
                            return;
                        }
                        else {
                        }
                    }
                }
            }
        }
    }

    function checkCollision(instance1Id, instance2Id) {
        let instance1 = instances[currentRoom][instance1Id];
        let instance2 = instances[currentRoom][instance2Id];
        let x1 = instance1["posx"] - 100;
        let y1 = instance1["posy"] - 100;
        let x2 = instance2["posx"] - 100;
        let y2 = instance2["posy"] - 100;
        x1 -= 100;
        y1 -= 100;
        x2 -= 100;
        y2 -= 100;

        x1 = Math.floor(x1);
        y1 = Math.floor(y1);
        x2 = Math.floor(x2);
        y2 = Math.floor(y2);

        let instance1Object = story["objects"][instance1["id"]];
        let instance2Object = story["objects"][instance2["id"]];

        var left = Math.max(x1 + instance1Object["bbox_left"], x2 + instance2Object["bbox_left"]);
        var top  = Math.max(y1 + instance1Object["bbox_top"], y2 + instance2Object["bbox_top"]);
        var right = Math.min(x1 + instance1Object["bbox_right"], x2 + instance2Object["bbox_right"]);
        var bottom  = Math.min(y1 + instance1Object["bbox_bottom"], y2 + instance2Object["bbox_bottom"]);

        for (var xp = left; xp < right; xp++)
            for (var yp = top; yp < bottom; yp++)
            {
                var i1 = (xp - x1) + (yp - y1) * instance1Object["img"].width;
                var i2 = (xp - x2) + (yp - y2) * instance2Object["img"].width;
                if (instance1Object["mask"][i1] && instance1Object["mask"][i2])
                    return true;
            }
        return false;

        // console.log("even collision checken");
        // console.log(instance1Id);
        // console.log(instance2Id);
    }

    function checkOnClickRules() {
        let objectId = instances[currentRoom][selectedObject]["id"];
        for (let i=0; i < story["transitions"].length; i++) {
            let transition = story["transitions"][i];
            if ((objectId === transition["pos1"] && transition["pos2"] === empty && transition["condition"] === "click") || (objectId === transition["pos2"] && transition["pos1"] === empty && transition["condition"] === "click")) {
                // console.log("regel gevonden");
                doTransition(i);
                selectedObject = -1;
                draggingObject = -1;
                return;
            }
        }
        for (let i=0; i < story["rules"].length; i++) {
            let rule = story["rules"][i];
            let r = getRandomRule(rule["pos1"], rule["pos2"]);
            if (objectId === rule["pos1"] && rule["pos2"] === empty && rule["condition"] === "click") {
                // console.log("regel gevonden");
                doRule(r, selectedObject, empty);
                selectedObject = -1;
                draggingObject = -1;
                return;
            }
            else if (objectId === rule["pos2"] && rule["pos1"] === empty && rule["condition"] === "click") {
                doRule(r, empty, selectedObject);
                selectedObject = -1;
                draggingObject = -1;
                return;
            }
        }
    }

    function doTransition(index) {
        let transition = story["transitions"][index];
        if (transition["goto"] === "next") {
            currentRoom += 1;
            if (currentRoom === story["rooms"].length) {
                currentRoom = 0;
            }
            // console.log("op naar de volgende kamer");
        }
        else if (transition["goto"] === "prev") {
            currentRoom -= 1;
            if (currentRoom < 0) {
                currentRoom = story["rooms"].length - 1;
            }
            // console.log("op naar de vorige kamer");
        }
        else if (transition["goto"] != empty) {
            currentRoom = transition["goto"];
        }
        else {
            return;
        }
        if (story["rooms"][currentRoom]["state"] === "restarts") {
            instances[currentRoom] = [];
            for (let i=0; i < story["rooms"][currentRoom]["objects"].length; i++) {
                let object = story["rooms"][currentRoom]["objects"][i];
                instances[currentRoom].push({"id":object["id"], "posx":object["posx"], "posy":object["posy"], "changed": false});
            }
        }
        
        // TODO: take left/right

        // console.log(currentRoom);
        drawObjectsToCanvas();
    }

    function doRule(index, pos1InstanceId, pos2InstanceId) {
        let rule = story["rules"][index];
        let removePos1 = false;
        let removePos2 = false;
        // console.log(index);
        // console.log(pos1InstanceId);
        // console.log(pos2InstanceId);
        if (rule["pos3"] != empty) {
            if (pos1InstanceId != empty) {
                instances[currentRoom][pos1InstanceId] = {"id": rule["pos3"], "posx": instances[currentRoom][pos1InstanceId]["posx"], "posy": instances[currentRoom][pos1InstanceId]["posy"], "changed": true};
            }
            else {
                instances[currentRoom].push({"id": rule["pos3"], "posx": instances[currentRoom][pos2InstanceId]["posx"], "posy": instances[currentRoom][pos2InstanceId]["posy"], "changed": true});
            }
        }
        else {
            if (pos1InstanceId != empty) {
                removePos1 = true;
            }
        }
        if (rule["pos4"] != empty) {
            if (pos2InstanceId != empty) {
                instances[currentRoom][pos2InstanceId] = {"id": rule["pos4"], "posx": instances[currentRoom][pos2InstanceId]["posx"], "posy": instances[currentRoom][pos2InstanceId]["posy"], "changed": true};
            }
            else {
                instances[currentRoom].push({"id": rule["pos4"], "posx": instances[currentRoom][pos1InstanceId]["posx"], "posy": instances[currentRoom][pos1InstanceId]["posy"], "changed": true});
            }
        }
        else {
            if (pos2InstanceId != empty) {
                removePos2 = true;
            }
        }
        if (rule["pos5"] != empty) {
            let pos1Posx = 0;
            let pos1Posy = 0;
            let pos2Posx = 0;
            let pos2Posy = 0;
            if (pos1InstanceId === empty) {
                pos2Posx = instances[currentRoom][pos2InstanceId]["posx"];
                pos2Posy = instances[currentRoom][pos2InstanceId]["posy"];
                pos1Posx = pos2Posx;
                pos1Posy = pos2Posy;
            }
            else if (pos2InstanceId === empty) {
                pos1Posx = instances[currentRoom][pos1InstanceId]["posx"];
                pos1Posy = instances[currentRoom][pos1InstanceId]["posy"];
                pos2Posx = pos1Posx;
                pos2Posy = pos1Posy;
            }
            else {
                pos2Posx = instances[currentRoom][pos2InstanceId]["posx"];
                pos2Posy = instances[currentRoom][pos2InstanceId]["posy"];
                pos1Posx = instances[currentRoom][pos1InstanceId]["posx"];
                pos1Posy = instances[currentRoom][pos1InstanceId]["posy"];
            }
            let newInstancePosx = (pos1Posx + pos2Posx) / 2;
            let newInstancePosy = (pos1Posy + pos2Posy) / 2;
            instances[currentRoom].push({"id": rule["pos5"], "posx": newInstancePosx, "posy": newInstancePosy, "changed": true});
        }
        if (removePos1 && removePos2) {
            if (pos1InstanceId > pos2InstanceId) {
                removeInstance(pos1InstanceId);
                removeInstance(pos2InstanceId);
            }
            else {
                removeInstance(pos2InstanceId);
                removeInstance(pos1InstanceId);
            }
        }
        else if (removePos1) {
            removeInstance(pos1InstanceId);
        }
        else if (removePos2) {
            removeInstance(pos2InstanceId);
        }

        drawObjectsToCanvas();
    }

    function removeInstance(instanceId) {
        instances[currentRoom].splice(instanceId, 1);
        if (draggingObject === empty) {
            return;
        }
        else if (draggingObject === instanceId) {
            dragging = false;
            draggingObject = -1;
        }
        else if (draggingObject > instanceId) {
            draggingObject -= 1;
        }
    }

    function onMove(event) {
        if(dragging) {
            selectedObject = draggingObject;
            instances[currentRoom][draggingObject]["posx"] = mouseRelativePosition()["x"] + xDeviation;
            instances[currentRoom][draggingObject]["posx"] = clamp(instances[currentRoom][draggingObject]["posx"], 100 - story["objects"][instances[currentRoom][draggingObject]["id"]]["bbox_left"], 700 + 100 - story["objects"][instances[currentRoom][draggingObject]["id"]]["bbox_right"]);
            instances[currentRoom][draggingObject]["posy"] = mouseRelativePosition()["y"] + yDeviation;
            instances[currentRoom][draggingObject]["posy"] = clamp(instances[currentRoom][draggingObject]["posy"], 100 - story["objects"][instances[currentRoom][draggingObject]["id"]]["bbox_top"], 500 + 100 - story["objects"][instances[currentRoom][draggingObject]["id"]]["bbox_bottom"]);
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

    function doTick() {
        // console.log("hoi");
        checkTickRules();
        drawObjectsToCanvas();
        for (let i=0; i < instances[currentRoom].length; i++) {
            let instance = instances[currentRoom][i];
            instances[currentRoom][i] = {...instance, "changed": false};
            // console.log("na de tick: " + instances[currentRoom][i]["changed"]);
        }
    }

    function checkTickRules() {
        for (let i=0; i < story["rules"].length; i++) {
            let rule = story["rules"][i];
            if (rule["condition"] === "tick") {
                for (let j=0; j < instances[currentRoom].length; j++) {
                    // console.log(instances[currentRoom][j]["changed"]);
                    if (!(instances[currentRoom][j]["changed"])) {
                        // console.log(instances[currentRoom].length);
                        // console.log("varken");
                        let objectId = instances[currentRoom][j]["id"];
                        // console.log(instances);
                        // console.log(objectId);
                        // console.log(rule["pos1"]);
                        // console.log("objectid:" + objectId);
                        // console.log(rule["pos2"]);
                        if (rule["pos1"] === objectId && rule["pos2"] === empty) {
                            let r = getRandomRule(rule["pos1"], rule["pos2"]);
                            // console.log("banaan");
                            doRule(r, j, empty);
                        }
                        else if (rule["pos1"] === empty && rule["pos2"] === objectId) {
                            let r = getRandomRule(rule["pos1"], rule["pos2"]);
                            doRule(r, empty, j);
                        }
                        else if (rule["pos1"] === objectId || rule["pos2"] === objectId) {
                            for (let k=0; k < instances[currentRoom].length; k++) {
                                if (j != k && ((instances[currentRoom][k]["id"] === rule["pos1"] && instances[currentRoom][j]["id"] === rule["pos2"]) || (instances[currentRoom][k]["id"] === rule["pos1"] && instances[currentRoom][j]["id"] === rule["pos2"]))) {
                                    if (checkCollision(j, k)) {
                                        let r = getRandomRule(rule["pos1"], rule["pos2"]);
                                        if (rule["pos1"] === instances[currentRoom][j]["id"]) {
                                            doRule(r, j, k);
                                        }
                                        else {
                                            doRule(r, k, j);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    function getRandomRule(pos1, pos2) {
        let ruleCollection = [];
        for (let i=0; i < story["rules"].length; i++) {
            let rule = story["rules"][i];
            if (pos1 === rule["pos1"] && pos2 === rule["pos2"]) {
                ruleCollection.push(i);
            }
        }
        let rule = ruleCollection[Math.floor(Math.random() * ruleCollection.length)]
        // console.log(rule);
        return rule;
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
    <button onclick={doTick}> dotick</button>
    <div id="canvasContainer">
        <canvas bind:this={canvas} id="storyCanvas" width="700" height="500" onmousedown={selectObject}></canvas>
    </div>
    <p>{storyString}</p>
    <p>{story}</p>
    <p>{story["unique_id"]}</p>
    
</main>