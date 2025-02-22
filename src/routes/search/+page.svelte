<script>
      import { invoke } from '@tauri-apps/api/tauri'
      import {searchContent} from '../../store.js';
      import {searchTerm} from '../../store.js';
      import Database from "tauri-plugin-sql-api";
      import {onMount} from 'svelte';

      let db;
      let db_path = "sqlite:data.db";

      
      //let content = {'results':[]}
      let loaderClass = "hide-loader";
      let errorClass = "hide-loader"

      async function search() {
      
        loaderClass = "loader";
        errorClass = "hide-loader"

        // clearing content
        searchContent.set([]);

        //let content = await invoke('retreive_Search', {value: $searchTerm})
        //searchContent.set(content.results);

        await invoke('retreive_Search', {value: $searchTerm})
            .then((data) => searchContent.set(data.results))
            .catch((err) => {

                errorClass = "info-bar error";
                console.log(err);
            })
        

        loaderClass = "hide-loader";

    }

    

    async function loadData(){

            db = await Database.load(db_path);

            
    }

    onMount(() => {
        loadData();
    });

</script>



<div class="area">

    <div class="search-header"> 
        <input  placeholder="Enter a podcast to search" bind:value={$searchTerm}/>
        <button on:click="{search}">Submit</button>
    </div>

    <div class={errorClass}>
        <span class="icon">&#9888;</span> <!-- Unicode for a warning icon -->
        <span class="message"> Invalid search term: {$searchTerm}.</span>
    </div>


    <div class={loaderClass}></div>
        


    <div id="content"> 

       

        {#each $searchContent as item, index}
        <div class="cap-item">
            <a href={`/content/${index}`}>
            <img class='cap-img' src={item.artworkUrl600}/>
            <!---<h2>{item.collectionName}</h2> -->
            </a>

            <button class="remove-button">+</button>

        </div>
        {/each}


    </div>
</div>


<style>

    /* Remove Button */
.remove-button {
    position: absolute;
    bottom: 20px;
    right: 10px;
    background-color: rgb(33, 66, 33);
    border: none;
    color: white;
    font-size: 18px;
    width: 45px;
    height: 45px;
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.7); /* Stronger shadow for the button */
}

.remove-button:hover {
    background-color: green;
}


    .area {

    margin:20px;
    display: grid;
    
    
    }

    .search-header{

        place-self: center;
        margin-top:20px;
        margin-bottom: 20px;
    }

    #content{
        display: grid;
        grid-template-columns: repeat(4, 1fr);
        gap: 10px;
        justify-content: center;
        margin-bottom: 50px; 

    }

 .cap-item:hover{
    transform: scale(1.05);
 }
.cap-item{
    position: relative;
    background-color: #333; /* Dark gray for card background */
    border-radius: 10px;
    box-shadow: 0 4px 8px rgba(37, 35, 35, 0.5); /* Stronger shadow for depth */
    text-align: center;
    transition: transform 0.3s ease;
    overflow: hidden; /* Ensure no overflow */
    max-width: 350px;
    max-height: 350px;
    
}
.cap-img{

    max-width: 350px;
    max-height: 350px;
}

.hide-loader {

display: none;
}
.loader {
width: 150px;
aspect-ratio: 1;
display: grid;
border: 4px solid #0000;
border-radius: 50%;
border-right-color: #25b09b;
animation: l15 1s infinite linear;

place-self: center;


}
.loader::before,
.loader::after {    
content: "";
grid-area: 1/1;
margin: 2px;
border: inherit;
border-radius: 50%;
animation: l15 2s infinite;
}
.loader::after {
margin: 8px;
animation-duration: 3s;
}
@keyframes l15{ 
100%{transform: rotate(1turn)}
}

.info-bar {
    display: flex;
    align-items: center;
    padding: 10px 20px;
    margin: 20px;
    border-radius: 5px;
    font-size: 16px;
    transition: background-color 0.3s, color 0.3s;
}

.info-bar.error {
    background-color: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
}

.info-bar .icon {
    margin-right: 10px;
    font-size: 20px;
}

</style>