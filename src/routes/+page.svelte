<script lang="ts">
    let time = 'test';
        // When using the Tauri API npm package:
    import { invoke } from '@tauri-apps/api/tauri';

    async function get_stuff() {
        try {
        time = await invoke('display_time')
        console.log(time)
        }
        catch(error) {
            console.error('Error occurred:', error);
        }
    }
    get_stuff()
</script>

<mytitle>
    <h1>Welcome to the St Kateri Hack Club</h1>
</mytitle>

<time_display>
    <h2>Current time:  {time}</h2>
</time_display>

<h1> fghfghg</h1>
<h1>hello world</h1>
<h1>Welcome to the hack club</h1>

<style>
    mytitle{
        display: grid;
        text-align: center;
        color:violet;
    }

    time_display{
        display:grid;
        padding: 5%;
        background-color: aquamarine;
    }
</style>
<a href="https://www.w3schools.com">This is a link</a> 

