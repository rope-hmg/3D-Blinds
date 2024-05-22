<script lang="ts">
    import { onMount } from "svelte";
    import { Message, Colour, start_app, push_message, set_colour, set_layout_code } from "app";
    import { initialised } from "main";

    import Box from "lib/Box.svelte";
    import Cube from "lib/Cube.svelte";

    onMount(() => {
        initialised.then(start_app);
    });

    function debounce<A extends any[]>(f: (...args: A) => void, ms: number): (...args: A) => void {
        let timeout: number;

        return (...args: A) => {
            clearTimeout(timeout);
            timeout = setTimeout(() => f(...args), ms);
        };
    }
    const set_layout_code_debounced = debounce(set_layout_code, 500);

    function on_change(event: any): void {
        set_layout_code_debounced(event.target.value);
    }

    function toggle_louvers() {
        push_message(Message.Toggle_Louvers);
    }
    function toggle_angles() {
        push_message(Message.Toggle_Angles);
    }
    function toggle_doors() {
        push_message(Message.Toggle_Doors);
    }
    function top_view() {
        push_message(Message.Top_View);
    }
    function front_view() {
        push_message(Message.Front_View);
    }
</script>

<main class="container">
    <Box>
        <h2>Shutter Layout</h2>
        <hr class="header_hr" />
        <label for="layout_code">Layout Code <span class="required">*</span></label>
        <input id="layout_code" type="text" placeholder="e.g. LBLBRBR" on:input={on_change} value="LR" />
    </Box>

    <Box>
        <h2>Colour</h2>
        <hr class="header_hr" />
        <button on:click={() => set_colour(Colour.White)}>White</button>
        <button on:click={() => set_colour(Colour.Red)}>Red</button>
        <button on:click={() => set_colour(Colour.Green)}>Green</button>
        <button on:click={() => set_colour(Colour.Blue)}>Blue</button>
    </Box>

    <Box>
        {#await initialised}
            <Cube />
        {:then}
            <article class="content">
                <aside class="controls">
                    <button on:click={toggle_louvers}>Toggle Louvers</button>
                    <button on:click={toggle_doors}>Toggle Doors</button>
                    <button on:click={top_view}>Top View</button>
                    <button on:click={front_view}>Front View</button>
                </aside>

                <canvas id="app-canvas"></canvas>
            </article>
        {/await}
    </Box>
</main>

<style>
    .container {
        height: 100%;
    }

    h2,
    .header_hr {
        color: rgb(103, 152, 179);
    }

    .required {
        color: rgb(187, 92, 103);
    }

    .content {
        width: 100%;
        height: 600px;
        display: flex;
        justify-content: center;
        align-items: stretch;
    }

    .controls {
        position: absolute;
        left: 0;
        width: 200px;
        height: 25%;
        display: flex;
        flex-direction: column;
        justify-content: space-around;
        align-items: center;
    }
</style>
