// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod search;
//pub mod xml;
pub mod xmlv2;

use crate::search::PodResult;
use reqwest::Request;
use serde_json::{self, Error};
//use crate::xml::{root, rss};
use serde_json::Value;
use serde_xml_rs::from_str;
//use xml::podItem;
use xmltojson::to_json;

use crate::xmlv2::{root,ch};
use tauri_plugin_sql::{Migration, MigrationKind};




#[tauri::command]
async fn retreive_feed(url: String) -> Result<ch,String> {

  //let url = "https://changelog.com/podcast/feed";

  println!("{}",url);

  let request = match reqwest::get(url).await {

    Ok(resp) => resp.text().await.unwrap(),
    Err(err) => panic!("Error {}", err)
  };

  let fnstr = to_json(&request).expect("Error").to_string();

  match serde_json::from_str::<root>(&fnstr) {
    Ok(parsed) => Ok(parsed.rss.channel),
    Err(e) => Err(format!("Failed to parse JSON: {}", e)),
  }



  //let parsed: root = serde_json::from_str(&fnstr).unwrap();

  

  //return parsed.rss.channel;

  
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
async fn retreive_Search(value: String) -> Result<PodResult,String> {


  let target_url = "https://itunes.apple.com/search?term=";
  
  let total_url = format!("{}{}{}",target_url,value,"&media=podcast");

  let request = match reqwest::get(total_url).await {

      Ok(resp) => resp.text().await.unwrap(),
      Err(err) => panic!("Error {}", err)
  };


  match serde_json::from_str::<PodResult>(&request) {
    Ok(parsed) => Ok(parsed),
    Err(e) => Err(format!("Failed to parse JSON: {}", e)),
  }

  
  
  // let parsed: PodResult = serde_json::from_str(&request)?;
  


  //return parsed;


}

fn main() {



  let migrations = vec![
        // Define your migrations here
        Migration {
            version: 1,
            description: "create_initial_tables",
            sql: "CREATE TABLE podcast(id INTEGER PRIMARY KEY, title TEXT, image TEXT, feed TEXT);",
            kind: MigrationKind::Up,
        }
    ];



  tauri::Builder::default()
    .plugin(
      tauri_plugin_sql::Builder::default()
          .add_migrations("sqlite:data.db", migrations)
          .build(),
      )   
    .invoke_handler(tauri::generate_handler![retreive_Search,retreive_feed])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
