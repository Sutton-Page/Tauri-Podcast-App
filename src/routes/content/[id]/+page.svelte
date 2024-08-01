<script>
import { page } from '$app/stores';
  import { type } from '@tauri-apps/api/os';
import {searchContent} from '../../../store';
import { invoke } from '@tauri-apps/api/tauri'
import {onMount} from 'svelte';

let pageIndex = parseInt($page.url.pathname.split('/')[2]);

console.log($page.url.pathname.split('/'));

let  content = {'item':[]};

let podImage = $searchContent[pageIndex].artworkUrl600;
let podTitle = $searchContent[pageIndex].collectionName

let podDescription = "";

let loaderClass = "loader";


function checkImage(index){

    if(content.item.length > 0){

        let item = content.item[index];

        if(item.hasOwnProperty('itunes:image')){

            if(item['itunes:image'] != null){

                return item['itunes:image']['@href'];
            }
            
        } 

        return podImage


    }
}

async function loadContent(){

    
    let value = $searchContent;

    let data = value[pageIndex];

    let url = data.feedUrl;

    content = await invoke('retreive_feed',{url})

    //podDescription = content.description;

    
    if(typeof(content.description) == 'string'){

        podDescription = content.description;
    } else{

        podDescription = content.description["#cdata"];
    }

    
    loaderClass = "hide-loader";

    console.log(content);




}


onMount(() => {

    loadContent();
});




</script>

<div id="root">

   
    <div class="title-content">
        
        <img  src={podImage} width="300px" height="300px"/>
        

        <div class="title-options">
            <h1> {podTitle}</h1>
            {#if podDescription != ""}
                 <p class="content-desc"> {podDescription}</p>
            {/if}
            <button> Add Podcast </button>
        </div>

        
        
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
    grid-template-columns: 300px 0.7fr;
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
</style>