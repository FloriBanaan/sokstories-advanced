<script>
    // @ts-nocheck
    import { event } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";

    let comments = [];
    let newComment = "";

    onMount(() => {
        comments = invoke("fetch_comments", {storyId:"JNLA"});
    });

    function postComment() {
        invoke("post_comment", {comment:newComment, storyId:"JNLA"});
        comments = invoke("fetch_comments", {storyId:"JNLA"});
    }


</script>

<style>

</style>

<main>
    <p> Write a comment:</p>
    <form class="row" onsubmit={postComment}>
        <input placeholder="Comment..." bind:value={newComment} />
        <button type="submit">Post comment</button>
    </form>
    <p>Comments:</p>
    {#each comments as comment}
    <p>{comment["maker_name"]}</p>
    <p>{comment["comment"]}</p>
    {/each}
</main>