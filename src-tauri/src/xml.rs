use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Debug)]
pub struct rss {

    pub channel: root

}

#[derive(Serialize, Deserialize, Debug)]
pub struct root {

    pub description: String,
    pub item: Vec<podItem>,
    pub title: String,

    



}

#[derive(Serialize, Deserialize, Debug)]
pub struct podItem {

    title: String,
    description: String,
    pubDate: String,
    enclosure: inner,

    
    image: Image,

  
    

}




#[derive(Serialize,Debug, Deserialize)]
struct Image {
    #[serde(rename = "href", default)]
    href: String,
    title: String,
}



#[derive(Serialize, Deserialize, Debug)]
pub struct inner{

    url: String
}