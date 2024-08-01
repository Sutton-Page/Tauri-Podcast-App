use serde::{Deserialize, Serialize};
use either::Either;

#[derive(Serialize, Deserialize, Debug)]
pub struct root {

    pub rss: info
}


#[derive(Serialize, Deserialize, Debug)]
pub struct info {

    pub channel: ch


}

#[derive(Serialize, Deserialize, Debug)]
pub struct ch{

    
    //description: desc,
    #[serde(with = "either::serde_untagged")]
    description: Either<String, desc>,

    item: Vec<podcast>

}




#[derive(Serialize, Deserialize, Debug)]
pub struct podcast {

    #[serde(with = "either::serde_untagged")]
    title: Either<String,desc>,
    
    #[serde(rename = "itunes:image")] 
    image: Option<inner_image>,

    enclosure: Option<en_image>,

    #[serde(with = "either::serde_untagged")]
    description: Either<String, desc>,
    
}

#[derive(Serialize, Deserialize, Debug)]
pub struct en_image {
    
    #[serde(rename = "@url")] 
    url: String

}


#[derive(Serialize, Deserialize, Debug)]
pub struct desc {
    
    #[serde(rename = "#cdata")] 
    text: String

}

#[derive(Serialize, Deserialize, Debug)]
pub struct inner_image {
    
    #[serde(rename = "@href")] 
    url: String

}

