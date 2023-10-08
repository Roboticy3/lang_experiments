<style>
/*

My idea is to layout the assigment page as a set of flexboxes.

I'm not too familiar with css, but I know flexbox is the way to go for panel-style layouts

The biggest challenge with this would probably be rendering the simulation of the logic circuit to the simulator box correctly

We'll just have to see how it goes */

/* board is the whole screen, laid out from top to bottom*/
    .board {
        display: flex;
        flex-direction: column;
        position: absolute;
        left: 0;
        top: 0;
        width: 100%;
        height: 100vh;
    }

/* the shelf is where the user picks elements, saves progress, and opens a menu to leave the level */
    .shelf {
        display: flex;
        background-color: blue;
        flex-grow: 1;
        flex-direction: row;
        justify-content: left;
    }

    .gate-spawner {
        display: flex;
        margin-left: 3%;
        align-items: center;
        align-self: center;
        background-color: yellow;
        position: inherit;
    }

/* assignment is the middle console where the user tries designs with the instructions on the side */
    .assignment {
        display: flex;
        flex-direction: row;
        flex-grow: 8.5;
    }

    .simulator {
        display: flex;
        flex-grow: 3;
        background-color: purple;
    }

    .directions {
        display: flex;
        flex-grow: 1;
        background-color: green;
        padding: 30px 30px
    }

/* simulation control goes on the bottom */
    .control-panel {
        display: flex;
        background-color: red;
        flex-grow: 1.5;
        justify-content: center;
    }

    .simulator-player {
        display: flex;
        margin: 0%;
        height: 50%;
        align-items: center;
        align-self: center;
        background-color: yellow;
    }

    .button-image {
        width: 30px;
        height: 30px;
    }

</style>

<div class="board">

    <div class="shelf">
        {#each gates as gate (gate.name)}
        <div class="gate-spawner">
            <input type="button" on:click="{() => handleShelfButton(gate.name)}" id="{gate.name}" value="{gate.name}" />
        </div>
        {/each}
    </div>

    <div class="assignment">
        <div>
            <p>
                {label}
            </p>
        </div>
        <div class="simulator">
            aaa
        </div>
        <div class="directions">
            bbb
        </div>
    </div>

    <div class="control-panel">
        <div class="simulator-player">
            <button>
                <img class="button-image" alt="asd" src="{playImage}" />
            </button>
        </div>
    </div>

</div>

<script>

    import playImage from '$lib/assets/GO.jpeg'

    let label = ""
    
    let gates = [
        {name:"Buf", sizeIn:1, sizeOut:1},
        {name:"Not", sizeIn:1, sizeOut:1},
        {name:"And", sizeIn:2, sizeOut:1},
        {name:" Or", sizeIn:2, sizeOut:1}
    ]
    
    /**
	 * @param {String} event
	 */
    function handleShelfButton(event) {
        label = event
    }

    function gateSpawnerFactory(gateName, gate) {
        
    }

</script>
