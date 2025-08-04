use std::collections::HashMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use tera::{to_value, Context, Function};
use serde_json::Value;

use super::funcs::progressbar::Progressbar;

const WAKATIME_API_URL : &'static str = "https://wakatime.com/api/v1/users/lognes/stats";

pub fn get_wakatime_stats(
    ctx: &mut Context
) -> Result<(), Error> {

    // spin up a new reqwest client
    let client = reqwest::blocking::Client::new();

    let json_data: Value = client.get(WAKATIME_API_URL)
        .send()?
        .json()?;

    // we're just gonna insert all of the wakatime data into the template for the lols
    ctx.insert("wakatime", &json_data);

    ctx.insert(
        "langs", 
        &json_data["data"]["languages"]
            .as_array()
            .map(|arr| &arr[..arr.len().min(8)])
    );

    // I wanna do some extra stuff to make my life easier so I'm gonna do some string formatting in here
    wakatime_format_langs(
        ctx, 
        &json_data["data"]["languages"].as_array().unwrap()
    );

    ctx.insert("full", &json_data["data"]);

    // obligatory ok(())
    return Ok(());
}

#[derive(Serialize, Deserialize)]
pub struct WakatimeLangStat {
    pub name : String,
    pub text : String,
    pub percent : f64,

    pub progressbar_string : String,
}

pub fn wakatime_format_langs(
    ctx: &mut Context,
    langs: &Vec<Value>
) {

    // create a new vec to hold the formatted langs
    let mut formatted_langs: Vec<WakatimeLangStat> = Vec::new();

    for lang in langs {

        let mut pgrs_hm: HashMap<String, tera::Value> = HashMap::new();

        pgrs_hm.insert("value".to_string(), lang.get("percent").unwrap().to_owned());

        formatted_langs.push(
            WakatimeLangStat { 
                name: lang.get("name").unwrap().as_str().unwrap().to_owned(), 
                text: lang.get("text").unwrap().as_str().unwrap().to_owned(), 
                percent: lang.get("percent").unwrap().as_f64().unwrap(), 
                progressbar_string: Progressbar::builder().char_length(30).call(&pgrs_hm).unwrap().as_str().unwrap().to_owned() 
            }
        );

        
        // really cheap way to limit the size
        if formatted_langs.len() >= 5 {
            break;
        }
        
    }

    // this is the part where we add all that schwanky padding and justification
    for index in 0..formatted_langs.len() {

        let lang = &mut formatted_langs[index];

        lang.name = format!("{:15}", lang.name);

    }
    
    ctx.insert("langs", &to_value(formatted_langs).unwrap());

}