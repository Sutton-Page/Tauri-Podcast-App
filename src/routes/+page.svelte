<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import {searchContent} from '../store.js';

    

  
    let value = ''
    let content = {'results':[]}
  
    async function search() {
      
      content = await invoke('retreive_Search', { value })
      searchContent.set(content.results);


    }
  </script>

  
  
  <div class="area">
    <input  placeholder="Enter a podcast to search" bind:value={value}/>
    <button on:click="{search}">Submit</button>

    
    
    <div id="content"> 

        {#each content.results as item, index}
        <div class="cap-item">
            <a href={`/content/${index}`}>
            <img class='cap-img' src={item.artworkUrl600}/>
            <h2>{item.collectionName}</h2>
            </a>
        </div>
        {/each}


    </div>

  </div>



  <style>

.area {

    margin:20px;
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


