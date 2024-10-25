<script lang="ts">
    import { onMount } from "svelte";
    import { invoke } from "@tauri-apps/api/core";

    let task = "";
    let tasks: string[] = [];

    async function addTask(event: Event) {
        event.preventDefault();
        try {
            await invoke("add_task", { task: task });
            await fetchTasks(); // Fetch the updated list of tasks
            task = ""; // Clear the input field after adding the task
        } catch (error) {
            console.error("Failed to add task:", error);
        }
    }

    async function fetchTasks() {
        try {
            tasks = await invoke("get_tasks");
        } catch (error) {
            console.error("Failed to fetch tasks:", error);
        }
    }

    async function checkTask(index: number) {
        try {
            await invoke("check_task", { index });
            await fetchTasks(); // Fetch the updated list of tasks
        } catch (error) {
            console.error("Failed to check task:", error);
        }
    }

    onMount(() => {
        fetchTasks(); // Fetch tasks when the component mounts
    });
</script>

<main class="container">
    <h1>To-Do List</h1>

    <ul>
        {#each tasks as task, index}
            <li>
                {task}
                <button onclick={() => checkTask(index)}>✓</button>
            </li>
        {/each}
    </ul>

    <form class="row" onsubmit={addTask}>
        <input id="greet-input" placeholder="Buy eggs..." bind:value={task} />
        <button type="submit" disabled={task.trim() === ""}>Add Task</button>
    </form>
</main>

<style>
    /*do not show the dot of the list*/
    ul {
        list-style-type: none;
    }

    li {
        background-color: #0f0f0f;
        border-radius: 8px;
        width: 600px;
        margin-top: 5px;
        display: flex;
        justify-content: space-between;
        align-items: center;
        text-indent: 10px;
    }

    li button {
        text-align: right;
        background-color: #2f2f2f;
        margin: 5px;
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

    .row {
        display: flex;
        justify-content: center;
    }

    a {
        font-weight: 500;
        color: #646cff;
        text-decoration: inherit;
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
