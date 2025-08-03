<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import {searchContent} from '../store.js';

    import Database from "tauri-plugin-sql-api";
    import {onMount} from 'svelte';

    let db;

    let savedPodcasts = [];
    let db_path = "sqlite:data.db";

   

    let contextId = "";

  async function removePod(id){

      if(db != null){

          let query = "DELETE FROM podcast where id = $1";
          let result = await db.execute(query,[id]);

          let getAll = "select * from podcast";
          let request = await db.select(getAll)

          savedPodcasts = request;
      }
  }

    
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
    
    <h1 style="text-align:center"> Saved Podcasts </h1>
    <div class="saved">

      
      {#if savedPodcasts.length > 0}

      


        {#each savedPodcasts as pod}

          <div class="pod-card">
            
              <button class="remove-button" on:click={removePod(pod.id)}>
                x
                
              </button>
            
            <a href={`/saved/${pod.id}`} >
            <img  src={pod.image} width="350px" height="350px" class="podcast-cover"/>
            </a>
            
          </div>
        {/each}
      
      {:else}
          <div class="no-saved-pod">
            <h3> No saved podcasts click the button below to search for podcast to add</h3>
            <a href='/search'> Search </a>
          </div>
      {/if}

    </div>
    

  </div>



  <style>
  
  .remove-button {
    position: absolute;
    top: 10px;
    right: 10px;
    background-color: #ff4d4d;
    border: none;
    color: white;
    font-size: 18px;
    width: 35px;
    height: 35px;
    border-radius: 50%;
    cursor: pointer;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.7); /* Stronger shadow for the button */
}

.remove-button:hover {
    background-color: #ff1a1a;
}

.pod-card:hover {
    transform: scale(1.05);
}

.podcast-cover {
    width: 100%;
    object-fit: cover;
    border-radius: 10px;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.3); /* Subtle shadow for cover */
    height: 100%; /* Image takes up the full height of the card */
}

.no-saved-pod{

  text-align: center;

}

.pod-card {
            
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

  display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 20px;
    justify-content: center;
    margin-bottom: 50px; 

    
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

.context-menu {
    position: absolute;
    top: 0;
    left: 0;
    background: #333; /* Dark background for context menu */
    border: 1px solid #555; /* Dark border */
    box-shadow: 0 2px 5px rgba(0, 0, 0, 0.5); /* Darker shadow */
    display: none;
    z-index: 1000;
  }

  .context-menu.visible {
    display: block;
  }

  .context-menu-item {
    padding: 8px 12px;
    cursor: pointer;
    display: flex;
    align-items: center;
    color: #e0e0e0; /* Light text for items */
  }

  .context-menu-item:hover {
    background: #444; /* Slightly lighter background on hover */
  }

  .context-menu-item .icon {
    margin-right: 8px;
  }


  </style>


