use std::collections::HashMap;

use anyhow::Error;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tera::{Context, Function, to_value};

use super::funcs::progressbar::Progressbar;

const WAKATIME_API_URL: &'static str = "https://wakatime.com/api/v1/users/lognes/stats";

pub fn get_wakatime_stats(ctx: &mut Context) -> Result<(), Error> {
    // spin up a new reqwest client
    let client = reqwest::blocking::Client::new();

    let json_data: Value = client.get(WAKATIME_API_URL).send()?.json()?;

    // we're just gonna insert all of the wakatime data into the template for the lols
    ctx.insert("wakatime", &json_data);

    // Yeah, this should've been here a longgg time ago...
    if let Some(err) = json_data.get("error") {
        panic!("Wakatime returned an error: {}", err.as_str().unwrap());
    }

    ctx.insert(
        "langs",
        &json_data["data"]["languages"]
            .as_array()
            .expect("Failed to unwrap languages as an array")
            .iter()
            .filter(|&x| {
                x["percent"].as_f64().unwrap() >= 10.0_f64
                    && x["total_seconds"].as_f64().unwrap() >= (20 * 60) as f64
            })
            .collect::<Vec<&Value>>(),
    );

    // I wanna do some extra stuff to make my life easier so I'm gonna do some string formatting in here
    wakatime_format_langs(ctx, &json_data["data"]["languages"].as_array().unwrap());
    wakatime_create_recent_work_string(ctx, &json_data["data"]["languages"].as_array().unwrap());

    ctx.insert("full", &json_data["data"]);

    // obligatory ok(())
    return Ok(());
}

#[derive(Serialize, Deserialize)]
pub struct WakatimeLangStat {
    pub name: String,
    pub text: String,
    pub percent: f64,

    pub progressbar_string: String,
}

pub fn wakatime_format_langs(ctx: &mut Context, langs: &Vec<Value>) {
    // create a new vec to hold the formatted langs
    let mut formatted_langs: Vec<WakatimeLangStat> = Vec::new();

    for lang in langs {
        let mut pgrs_hm: HashMap<String, tera::Value> = HashMap::new();

        pgrs_hm.insert("value".to_string(), lang.get("percent").unwrap().to_owned());

        formatted_langs.push(WakatimeLangStat {
            name: lang.get("name").unwrap().as_str().unwrap().to_owned(),
            text: lang.get("text").unwrap().as_str().unwrap().to_owned(),
            percent: lang.get("percent").unwrap().as_f64().unwrap(),
            progressbar_string: Progressbar::builder()
                .char_length(30)
                .call(&pgrs_hm)
                .unwrap()
                .as_str()
                .unwrap()
                .to_owned(),
        });

        // really cheap way to limit the size
        if formatted_langs.len() >= 8 {
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

/* Returns a string like "Python (1hr), Java (2hr 5m) and Vue.js (25m)"
 */
pub fn wakatime_create_recent_work_string(ctx: &mut Context, langs: &Vec<Value>) {
    let mut l = langs
        .iter()
        .filter(|&v| {
            v.get("total_seconds")
                .unwrap_or(&json!(0.0))
                .as_f64()
                .unwrap()
                >= (20 * 60) as f64
        })
        .map(|v| {
            format!(
                "**{}** `{}`",
                v.get("name").unwrap().as_str().unwrap(),
                v.get("text").unwrap().as_str().unwrap()
            )
        })
        .collect::<Vec<String>>();

    l = l[..l.len().min(8)].to_vec();

    let mut rw_str: String;

    if l.len() == 1 {
        rw_str = l[0].to_string();
    } else {
        let last = l
            .pop()
            .expect("failed to pop last string from recent_work thingy");
        rw_str = format!("{} and {}", l.join(",  "), last);
    }

    ctx.insert("recent_work", &rw_str);
}
