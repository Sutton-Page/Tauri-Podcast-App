<script>
      import { invoke } from '@tauri-apps/api/tauri'
      import {searchContent} from '../../store.js';
      import {searchTerm} from '../../store.js';

      
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
            <h2>{item.collectionName}</h2>
            </a>
        </div>
        {/each}


    </div>
</div>


<style>
    .area {

    margin:20px;
    display: grid;
    
    
    }

    .search-header{

        place-self: center;
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