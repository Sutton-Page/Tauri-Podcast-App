<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import {searchContent} from '../store.js';

    import Database from "tauri-plugin-sql-api";
    import {onMount} from 'svelte';

    let db;

    let savedPodcasts = [];

    let db_path = "sqlite:data.db";

    
    onMount(() => {

        loadData();
    });

    async function loadData(){

        db = await Database.load(db_path);
        
        const result = await db.select("SELECT * from podcast");

        savedPodcasts = result;

        
    }

  
    let value = ''
    let content = {'results':[]}

    let loaderClass = "hide-loader";
  
    async function search() {
      
      loaderClass = "loader";

      // clearing content
      content = {'results':[]};

      content = await invoke('retreive_Search', { value })
      searchContent.set(content.results);

      loaderClass = "hide-loader";

    }
  </script>


  
  <div class="area">
    
    <h2 style="text-align:center"> Saved Podcasts </h2>
    <div class="saved">

      
      {#if savedPodcasts.length > 0}

      

        {#each savedPodcasts as pod}
          <div>
            <a href={`/saved/${pod.id}`}>
            <img  src={pod.image} width="350px" height="350px"/>
            </a>
          </div>
        {/each}
      
      {:else}
          <div>
            <h3> No saved podcasts click the button below to search for podcast to add</h3>
            <button> Search </button>
          </div>
      {/if}

    </div>
    

  </div>



  <style>
  
  

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

.area {

    margin:20px;
    
}

.saved {

   display:grid;
   grid-template-columns: 1fr 1fr 1fr 1fr;
}

#content{
display: grid;
grid-template-columns: repeat(auto-fit, minmax(250px, 2fr));
margin-left:5%;
margin-bottom:2.5%;
}
.cap-item{
padding-top:20px;
border-bottom: 2.5px solid black;
}
.cap-img{

width: 70%;
height:70%;
}
  </style>


