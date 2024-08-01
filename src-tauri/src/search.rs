use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PodResult {
    
    pub results: Vec<ResultItem>
}


#[derive(Serialize, Deserialize, Debug)]
pub struct ResultItem {
    
    collectionName: String,
    feedUrl: String,
    artworkUrl600: String
}