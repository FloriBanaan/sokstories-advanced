<script>
  import { event } from "@tauri-apps/api";
  import { invoke } from "@tauri-apps/api/core";

  import Login from "$lib/Login.svelte";

  let currentPage = $state("login");
  let user = $state("");
  
  // let name = $state("");
  // let greetMsg = $state("");
  // let testStoryId = $state("");

  let storyId = $state("");
  let story = $state("");
  // let testStory = $state("");

  let gameCode = $state("QBVL");

  let name = $state("");
  let blubber = $state("");

  let maker_id = $state("");
  let code = $state("");

  let save = $state("");

  let verified = $state("");

  let max = $state("");
  let stories = $state("");

  let category_name = $state("");
  let category_success = $state("");

  let story_to_category = $state("");
  let add_to_story_success = $state("");

  let story_from_category = $state("");
  let remove_story_success = $state("");

  let category_to_remove = $state("");
  let category_remove_success = $state("");

  let story_name = $state("");
  let story_name_result = $state("");

  let maker_name = $state("");
  let maker_name_result = $state("");

  let comment = $state("");
  let comment_result = $state("");

  let story_to_get_comments_from = $state("");
  let comments = $state("");

  // async function greet(event) {
  //   event.preventDefault();
  //   // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  //   greetMsg = await invoke("greet", { name });
  // }

  // async function test_story(event) {
  //   event.preventDefault();
  //   testStory = await invoke("test_story", { story:testStoryId });
  // }

  function setPage(newPage) {
    currentPage = newPage;
  }

  function setUser(verifiedUser) {
    user = verifiedUser;
  }

  async function fetch_story(event) {
    event.preventDefault();
    story = await invoke("fetch_story", { id:storyId });
  }

  async function get_sokstories_name_and_id(event) {
    event.preventDefault();
    name = await invoke("get_sokstories_name_and_id", { blubber});
  }

  async function request_token(event) {
    event.preventDefault();
    code = await invoke("request_token", { requestNew:"no" });
  }

  async function request_new_token(event) {
    event.preventDefault();
    code = await invoke("request_token", { requestNew:"yes" });
  }

  async function init_save_folder(event) {
    event.preventDefault();
    save = await invoke("init_save_folder");
  }

  async function request_verification(event) {
    event.preventDefault();
    verified = await invoke("request_verification");
  }

  async function fetch_stories_by_recency(event) {
    event.preventDefault();
    stories = await invoke("fetch_stories_by_recency", {min:1, max:Number(max)});
  }

  async function create_category(event) {
    event.preventDefault();
    category_success = await invoke("create_category", {name:category_name})
  }

  async function add_story_to_category(event) {
    event.preventDefault();
    add_to_story_success = await invoke("add_story_to_category", {categoryName:"varken", story:story_to_category});
  }

  async function remove_story_from_category(event) {
    event.preventDefault();
    remove_story_success = await invoke("remove_story_from_category", {categoryName:"varken", story:story_from_category});
  }

  async function remove_category(event) {
    event.preventDefault();
    category_remove_success = await invoke("remove_category", {name:category_to_remove});
  }

  async function fetch_stories_by_name(event) {
    event.preventDefault();
    story_name_result = await invoke("fetch_stories_by_name", {name:story_name});
  }

  async function fetch_makers_by_name(event) {
    event.preventDefault();
    maker_name_result = await invoke("fetch_makers_by_name", {name:maker_name})
  }

  async function post_comment(event) {
    event.preventDefault();
    comment_result = await invoke("post_comment", {comment:comment, storyId:"QBVL"})
  }

  async function fetch_comments(event) {
    event.preventDefault();
    comments = await invoke("fetch_comments", {storyId:story_to_get_comments_from})
  }
</script>

<style>
.logo.vite:hover {
  filter: drop-shadow(0 0 2em #747bff);
}

.logo.svelte-kit:hover {
  filter: drop-shadow(0 0 2em #ff3e00);
}

:root {
  font-family: Inter, Avenir, Helvetica, Arial, sans-serif;
  font-size: 16px;
  line-height: 24px;
  font-weight: 400;

  color: #0f0f0f;
  background-color: #f6f6f6;

  font-synthesis: none;
  text-rendering: optimizeLegibility;
  -webkit-font-smoothing: antialiased;
  -moz-osx-font-smoothing: grayscale;
  -webkit-text-size-adjust: 100%;
}

.container {
  margin: 0;
  padding-top: 10vh;
  display: flex;
  flex-direction: column;
  justify-content: center;
  text-align: center;
}

.logo {
  height: 6em;
  padding: 1.5em;
  will-change: filter;
  transition: 0.75s;
}

.logo.tauri:hover {
  filter: drop-shadow(0 0 2em #24c8db);
}

.row {
  display: flex;
  justify-content: center;
}

a {
  font-weight: 500;
  color: #646cff;
  text-decoration: inherit;
}

a:hover {
  color: #535bf2;
}

h1 {
  text-align: center;
}

input,
button {
  border-radius: 8px;
  border: 1px solid transparent;
  padding: 0.6em 1.2em;
  font-size: 1em;
  font-weight: 500;
  font-family: inherit;
  color: #0f0f0f;
  background-color: #ffffff;
  transition: border-color 0.25s;
  box-shadow: 0 2px 2px rgba(0, 0, 0, 0.2);
}

button {
  cursor: pointer;
}

button:hover {
  border-color: #396cd8;
}
button:active {
  border-color: #396cd8;
  background-color: #e8e8e8;
}

input,
button {
  outline: none;
}

#greet-input {
  margin-right: 5px;
}

@media (prefers-color-scheme: dark) {
  :root {
    color: #f6f6f6;
    background-color: #2f2f2f;
  }

  a:hover {
    color: #24c8db;
  }

  input,
  button {
    color: #ffffff;
    background-color: #0f0f0f98;
  }
  button:active {
    background-color: #0f0f0f69;
  }
}

</style>


<main class="container">
  <p> user:{user}</p>
  {#if currentPage === "login"}
  <Login onSetPage={setPage} onSetUser={setUser}></Login>
  {:else if currentPage === "test"}
  <!-- <iframe src="https://sok-stories.com/?QBVL?embed?r?c" width='560' height='420'></iframe> -->

  <!-- <form class="row" onsubmit={greet}>
    <input id="greet-input" placeholder="Enter a name..." bind:value={name} />
    <button type="submit">Greet</button>
  </form>
  <p>{greetMsg}</p> -->
  <form class="row" onsubmit={fetch_story}>
    <input placeholder="zoek een story..." bind:value={storyId} />
    <button type="submit">zoeken</button>
  </form>
  <p>story: {story}</p>

  <!-- <form class="row" onsubmit={test_story}>
    <input placeholder="zoek een story..." bind:value={testStoryId} />
    <button type="submit">zoeken</button>
  </form>
  <p>deze story is gemaakt door: {testStory}</p> -->

  <form class="row" onsubmit={get_sokstories_name_and_id}>
    <input placeholder="blubber" bind:value={blubber} />
    <button type="submit">pak naam</button>
  </form>
  <p>jouw naam is: {name}</p>

  <form class="row" onsubmit={request_token}>
    <input placeholder="vul maker_id in..." bind:value={maker_id} />
    <button type="submit">krijg token</button>
  </form>
  <p>jouw verification code is: {code}</p>

  <form class="row" onsubmit={request_new_token}>
    <input placeholder="vul maker_id in..." bind:value={maker_id} />
    <button type="submit">krijg nieuwe token</button>
  </form>
  <p>jouw verification code is: {code}</p>

  <form class="row" onsubmit={init_save_folder}>
    <input placeholder="maak save folder..." bind:value={blubber} />
    <button type="submit">maak</button>
  </form>
  <p>is het gelukt? {save}</p>

  <form class="row" onsubmit={request_verification}>
    <input placeholder="verifieer..." bind:value={blubber} />
    <button type="submit">verifieer</button>
  </form>
  <p>geverifieerd: {verified}</p>

  <form class="row" onsubmit={fetch_stories_by_recency}>
    <input placeholder="voer max in..." bind:value={max} />
    <button type="submit">haal stories op</button>
  </form>
  <p>stories: {stories}</p>
  
  <form class="row" onsubmit={create_category}>
    <input placeholder="geef categorie een naam..." bind:value={category_name} />
    <button type="submit">maak categorie</button>
  </form>
  <p>gelukt? {category_success}</p>

  <form class="row" onsubmit={add_story_to_category}>
    <input placeholder="vul story in..." bind:value={story_to_category} />
    <button type="submit">voeg story toe aan categorie</button>
  </form>
  <p>gelukt? {add_to_story_success}</p>
  
  <form class="row" onsubmit={remove_story_from_category}>
    <input placeholder="vul story in..." bind:value={story_from_category} />
    <button type="submit">verwijder story uit categorie</button>
  </form>
  <p>gelukt? {remove_story_success}</p>

  <form class="row" onsubmit={remove_category}>
    <input placeholder="vul categorie in..." bind:value={category_to_remove} />
    <button type="submit">verwijder categorie</button>
  </form>
  <p>gelukt? {category_remove_success}</p>

  <form class="row" onsubmit={fetch_stories_by_name}>
    <input placeholder="vul story naam in..." bind:value={story_name} />
    <button type="submit">zoek story</button>
  </form>
  <p>stories: {story_name_result}</p>

  <form class="row" onsubmit={fetch_makers_by_name}>
    <input placeholder="vul maker naam in..." bind:value={maker_name} />
    <button type="submit">zoek maker</button>
  </form>
  <p>makers: {maker_name_result}</p>

  <form class="row" onsubmit={post_comment}>
    <input placeholder="vul comment in..." bind:value={comment} />
    <button type="submit">plaats comment</button>
  </form>
  <p>gelukt? {comment_result}</p>

  <form class="row" onsubmit={fetch_comments}>
    <input placeholder="krijg comments van..." bind:value={story_to_get_comments_from} />
    <button type="submit">haal comments op</button>
  </form>
  <p>comments: {comments}</p>
  {/if}
</main>