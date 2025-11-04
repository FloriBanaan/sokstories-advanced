<script>
    import { event } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let categories = [];
    let newCategoryName = "";
    let result = "hoi";
    let removeResult = "blubber";

    onMount(async () => {
        getCategories();
    });

    async function getCategories() {
        categories = JSON.parse(await invoke("get_data_from_save", {key:"categories"}));
    }

    async function createCategory() {
        result = await invoke("create_category", {name:newCategoryName});
        await getCategories();
    }

    async function removeCategory(name) {
        removeResult = await invoke("remove_category", {name});
        await getCategories();
    }

    async function RemoveStoryFromCategory(categoryName, story) {
        removeStoryResult = await invoke("remove_story_from_category", {category_name:categoryName, story});
        await getCategories();
    }


</script>

<style>

</style>

<main>
    <p> categories</p>
    <p> Create new category</p>
    <form class="row" onsubmit={createCategory}>
        <input placeholder="New category name..." bind:value={newCategoryName} />
        <button type="submit">Create</button>
    </form>

    <!-- <p>{categories}</p> -->

    {#each categories as category}
    <p>{category["name"]}</p>
    <button onclick={removeCategory(category["name"])}> Remove category</button>
    <hr>

    {#each category["stories"] as story}
    <p>{story}</p>
    <button onclick={RemoveStoryFromCategory(category["name"], story)}> Remove story</button>

    {/each}
    {/each}
    <!-- <p>{result}</p>
    <p>{removeResult}</p> -->
</main>