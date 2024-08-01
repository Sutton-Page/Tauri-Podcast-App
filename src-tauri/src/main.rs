// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod search;
//pub mod xml;
pub mod xmlv2;

use crate::search::PodResult;
//use crate::xml::{root, rss};
use serde_json::Value;
use serde_xml_rs::from_str;
//use xml::podItem;
use xmltojson::to_json;

use crate::xmlv2::{root,ch};


#[tauri::command]
async fn retreive_feed(url: String) -> ch {

  //let url = "https://changelog.com/podcast/feed";

  println!("{}",url);

  let request = match reqwest::get(url).await {

    Ok(resp) => resp.text().await.unwrap(),
    Err(err) => panic!("Error {}", err)
  };

  let fnstr = to_json(&request).expect("Error").to_string();

  let parsed: root = serde_json::from_str(&fnstr).unwrap();

  

  return parsed.rss.channel;

  
  /* 
  let finalResult: Result<rss,_> = from_str(&request);

  

  
  
  let temp = root {

      description: String::from(""),
      item: Vec::new(),
      title: String::from(""),
      

  };

  match finalResult {

    Ok(finalResult) => {

      

      return finalResult.channel;
        
    }

    Err(e) => {
        // Handle error
        eprintln!("Error parsing XML: {:?}", e);

        return temp;
    },
}*/


}

#[tauri::command]
async fn retreive_Search(value: String) -> PodResult{

  

  let target_url = "https://itunes.apple.com/search?term=";
  
  let total_url = format!("{}{}{}",target_url,value,"&media=podcast");

  let request = match reqwest::get(total_url).await {

      Ok(resp) => resp.text().await.unwrap(),
      Err(err) => panic!("Error {}", err)
  };


  let parsed: PodResult = serde_json::from_str(&request).unwrap();

  

  return parsed



}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![retreive_Search,retreive_feed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
