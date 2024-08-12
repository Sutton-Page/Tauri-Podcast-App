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
            
              <button class="pod-card-but" on:click={removePod(pod.id)}>
                <svg class="trash-icon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24">
                  <path d="M3 6h18v2H3V6zm2 4h14v12H5V10zm3-8h8l1 1H7l1-1zM9 12v8h2v-8H9zm4 0v8h2v-8h-2z"/>
                </svg>
              </button>
            
            <a href={`/saved/${pod.id}`} >
            <img  src={pod.image} width="350px" height="350px"/>
            </a>
            
          </div>
        {/each}
      
      {:else}
          <div class="no-saved-pod">
            <h3> No saved podcasts click the button below to search for podcast to add</h3>
            <button> Search </button>
          </div>
      {/if}

    </div>
    

  </div>



  <style>

.no-saved-pod{

  text-align: center;

}

.pod-card {
            
  position: relative;
  margin:10px;
            
}

.trash-icon {
            width: 30px;
            height: 30px;
            fill: rgb(18, 19, 19);
        }

.pod-card-but {
  position: absolute;
            top: 0px;
            right: 0px;  
            background:lightgray;
            border: none;
            cursor: pointer;
            font-size: 24px;
            color: #0f0e0e;
            transition: color 0.3s;
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

   display:grid;
   grid-template-columns: 1fr 1fr 1fr 1fr;
   margin-bottom: 10px;
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


