<script>
    import { event } from "@tauri-apps/api";
    import { invoke } from "@tauri-apps/api/core";
    import { onMount } from "svelte";
    // import { createEventDispatcher } from "svelte";

    // const dispatch = createEventDispatcher();

    export let onSetPage;
    export let onSetUser = "";

    let verification = "";
    let token = "key not found";
    let name_and_id = "";
    let name = "";
    let code = "";

    onMount(async () => {
        verify();
    });

    function continueAsGuest() {
        name = "";
        onSetPage("main");
        onSetUser(name);
    }

    function continueAsVerifiedAccount() {
        onSetPage("main");
        onSetUser(name);
    }

    async function generateNewToken(event) {
        event.preventDefault();
        code = await invoke("request_token", {request_new:"no"});
        await getTokenAndName();
    }

    async function forceNewToken(event) {
        event.preventDefault();
        code = await invoke("request_token", {request_new:"yes"});
        await getTokenAndName();
    }

    async function verify() {
        verification = "";
        await getTokenAndName();
        if (token === "key not found") {
            return;
        }
        verification = await invoke("request_verification");
        code = "";
    }

    async function getTokenAndName() {
        name_and_id = await invoke("get_sokstories_name_and_id");
        name = name_and_id[0];
        if (name === "sokstories not found") {
            return;
        }
        token = await invoke("get_data_from_save", {key:"token"})
    }

</script>

<style>

</style>

{#if name === ""}
<p> Welcome to sokstories advanced! It looks like you don't have a sokstories account. You need one to be able to create a sokstories advanced account. If you don't own sokstories, you can also continue as guest.</p>
<button on:click={getTokenAndName}> Try again</button>

{:else if verification === "unable to connect to server" || code === "unable to generate token"}
<p> Unable to connect to the server. Check your internet connection. If the server seems to be offline, please report this! In the meantime, you can continue as guest.</p>
<button on:click={verify}> Try again</button>

{:else if code === "token already exist"}
<p> It looks like you have already created an account. Have you used sokstories advanced on a different computer before? If you want to use sokstories advanced on this computer, please copy your old save to the sokstories folder. If that is not possible, you can generate a new token. But you will have to verify again!</p>
<button on:click={forceNewToken}> Generate new token</button>

{:else if verification === "save not found" || code === "unable to save token"}
<p> No sokstories save folder was found. If you don't have a sokstories account, you can continue as guest.</p>
<button on:click={verify}> Try again</button>

{:else if code !== ""}
<p> Thanks for creating an account, {name}! To continue, you need to verify your account to prove that the {name} sokstories account really belongs to you. To do this, please upload a story with the name: !bb verify {code}. Then click the button below to verify. Note: you may have to wait 10 minutes.</p>
<button on:click={verify}> Verify</button>

{:else if token === "key not found"}
<p> Welcome to sokstories advanced, {name}! It looks like you don't have an account yet. Click the button below to create one.</p>
<button on:click={generateNewToken}> Create account</button>

{:else if verification === "wrong token"}
<p> Your token and the one on the server don't match. Have you used sokstories advanced on a different computer before? If you want to use sokstories advanced on this computer, please copy your old save to the sokstories folder. If that is not possible, you can generate a new token. But you will have to verify again!</p>
<button on:click={forceNewToken}> Generate new token</button>

{:else if verification === "failed"}
<p> Your sokstories account wasn't able to get verified. Did you upload the story with the verification code yet? If you did, you may have to wait 10 minutes until the verification is complete. If you didn't and you forgot your verification code, click the button below to get a new one. </p>
<button on:click={forceNewToken}> Generate new token</button>

{:else if verification === "success"}
<p> Welcome back, {name}!</p>
<button on:click={continueAsVerifiedAccount}> Let's go!</button>

{:else}
<p> Loading... </p>

{/if}

<button on:click={continueAsGuest}> Continue as guest</button>

<!-- <p>verification: {verification}</p>
<p>token: {token}</p>
<p>name and id: {name_and_id}</p>
<p>name: {name}</p>
<p>code: {code}</p> -->