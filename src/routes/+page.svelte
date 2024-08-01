<script>
    import { invoke } from '@tauri-apps/api/tauri'
    import {searchContent} from '../store.js';

    

  
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
    <input  placeholder="Enter a podcast to search" bind:value={value}/>
    <button on:click="{search}">Submit</button>

    <div class={loaderClass}>

    </div>
    
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


