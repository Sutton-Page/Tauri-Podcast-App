<script>
import { page } from '$app/stores';
import { invoke } from '@tauri-apps/api/tauri'
import {onMount} from 'svelte';
import Database from 'tauri-plugin-sql-api';

let db = null;
let db_path = "sqlite:data.db";
let pageIndex = parseInt($page.url.pathname.split('/')[2]);

let data = [];
let content = {'item':[]};
let podDescription = "";
let podImage = "";

// variables to handle errors and loading css
let errorClass = "hide-loader"
let loaderClass = "loader";
let infoClass = "hide-loader";


function addPodcast() {

    
}

async function loadContent() {

    db = await Database.load(db_path);

    const result = await db.select("SELECT * from podcast WHERE id = $1",[pageIndex]);

    if(result.length > 0){

        data = result
        podImage = data[0].image;
        let url = data[0].feed;

        await invoke('retreive_feed',{url})
        .then((value) => {

            content = value

            if(typeof(content.description) == 'string'){

                podDescription = content.description;
                } else{

                podDescription = content.description["#cdata"];
                }

        })
        .catch((err) => {

            errorClass = "info-bar-fail error"
            console.log(err);
        })

    
    
        loaderClass = "hide-loader";

    }


}


onMount(() => {

    loadContent()

});

</script>

<div id="root">

    
    {#if data.length > 0}
   
    <div class="title-content">
        
        <img  src={data[0].image} width="300px" height="300px"/>
        

        <div class="title-options">
            <h1> {data[0].title}</h1>
            {#if podDescription != ""}
                 <p class="content-desc"> {podDescription}</p>
            {/if}
            <button on:click={addPodcast}> Add Podcast </button>
        </div>

        
        
    </div>

    {/if}

    
    <div class={errorClass}>
        <span class="icon">&#9888;</span> <!-- Unicode for a warning icon -->
        <span class="message"> Unable to load RSS Feed. Try again later.</span>
    </div>
    <div class={loaderClass}></div>

    <div id="content-v2">
        {#each content.item as podItem, index}
        <div class="content-card">

            <img  src={podImage} width="300px" height="300px"/>

            <!----
            undecided about if I should jsut use podImage
            {#if podItem["itunes:image"] != null}

                <img class='cap-img' src={podItem["itunes:image"]["@href"]}/>
                
            {:else}
                <img class='cap-img' src={podImage}/>

            {/if}-->
           
            <div class="title-options"> 
            {#if typeof(podItem.title) == 'string'}
            <h3> {podItem.title}</h3>

            {:else}
                <h3> {podItem.title["#cdata"]}</h3>
            {/if} 

            {#if typeof(podItem.description) == 'string'}
                <p class="content-desc"> {podItem.description}</p>
            {:else}
                <p class="content-desc"> {podItem.description["#cdata"]}</p>
            {/if}

            {#if podItem.hasOwnProperty("enclosure")}

                <audio controls src={podItem.enclosure["@url"]} preload="none"></audio>
           
            {:else}
                <p> Check rss feed</p>
            {/if}
        
            </div> 
            

        </div>
        {/each}
    </div>
   
    
</div>


<style>

.info-bar {
            display: flex;
            justify-content: space-between;
            align-items: center;
            padding: 10px 20px;
            background-color: #28a745; /* Green background */
            color: #f1f1f1;
            border-radius: 5px;
            margin: 10px;
            box-shadow: 0 4px 6px rgba(0, 0, 0, 0.5);
        }
        .info-bar button {
            background-color: transparent;
            border: none;
            color: #f1f1f1;
            font-size: 16px;
            cursor: pointer;
        }
        .info-bar button:hover {
            color: #ddd;
        }



    #root{

        display: grid;
        
        
        

    }

    .title-content{

        margin: 10px;
        margin-bottom: 20px;
        display: grid;
        grid-template-columns: 300px 600px;
        
        

    }

    .title-options {

        grid-column: 2;
        grid-row: 1;
        margin:10px;
        font-family: 'Times New Roman';
    }

    

    .title-options p {

        font-size: 1.1em;
        
        
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

    .mid-image{

        width:20%;
        height: 20%;
    }

    #content{

    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(250px, 2fr));
    margin-left:5%;
    margin-bottom:2.5%;
    }

    

    #content-v2{

        display: grid;
   
        margin-left:5%;
        margin-bottom:2.5%;
       
    }

    .content-desc {

        font-size: 1.1em;
        width:50vw;
        height:20vh;
        text-wrap:balance;
        overflow:hidden;
        
    }

    .content-card{

    margin: 10px;
    display: grid;
    grid-template-columns: 300px 0.5fr;
    padding:10px;
    
    border: 0.5px inset;
    border-left:0;
    border-bottom: 0;
    border-right: 0;
    
    border-image-slice: 1;

    


    border-image-source: linear-gradient(
        to right, #e0e0e0, #3f3d3d, #2d2b2b);
    

    
    }
    

    
    
.cap-item{
padding-top:20px;
border-bottom: 2.5px solid black;
}
.cap-img{

width: 80%;
height:80%;
}

.info-bar-fail {
    display: flex;
    align-items: center;
    padding: 10px 20px;
    margin: 20px;
    border-radius: 5px;
    font-size: 16px;
    transition: background-color 0.3s, color 0.3s;
}

.info-bar-fail.error {
    background-color: #f8d7da;
    color: #721c24;
    border: 1px solid #f5c6cb;
}

.info-bar-fail .icon {
    margin-right: 10px;
    font-size: 20px;
}



</style>