use crate::{
    datapath::CONFIG_JSON_PATH,
    utils::{read_json, write_json},
};
use reqwest::get;
use serde_json::Value;

pub async fn update_config() {
    let mut config = read_json(CONFIG_JSON_PATH).unwrap();

    let old_res_ver = config["version"]["android"]["resVersion"].clone();
    let old_client_ver = config["version"]["android"]["clientVersion"].clone();

    let old_func_ver = config["networkConfig"]["cn"]["content"]["funcVer"].clone();

    let official_response = match get("https://ak-conf.hypergryph.com/config/prod/official/Android/version").await {
        Ok(res) => res.json::<Value>().await.unwrap(),
        Err(_) => panic!("Unable to parse request."),
    };

    let new_res_ver = official_response["resVersion"].clone();
    let new_client_ver = official_response["clientVersion"].clone();

    if new_res_ver != old_res_ver {
        config["version"]["android"]["resVersion"] = new_res_ver;
    }
    if new_client_ver != old_client_ver {
        config["version"]["android"]["clientVersion"] = new_client_ver;
    }

    let func_response = match get("https://ak-conf.hypergryph.com/config/prod/official/network_config").await {
        Ok(res) => res.json::<Value>().await.unwrap(),
        Err(_) => panic!("Unable to parse request."),
    };

    println!("{:#?}", func_response);

    match write_json("sniffed/network.json", func_response) {
        Ok(_) => (),
        Err(_) => panic!("Unable to write to file."),
    };

    // let new_func_ver = func_response["content"]["funcVer"].clone();

    // if new_func_ver != old_func_ver {
    //     config["networkConfig"]["cn"]["content"]["funcVer"] = new_func_ver.clone();
    //     config["networkConfig"]["cn"]["content"]["configs"][new_func_ver.to_string()] = func_response["content"]["configs"][new_func_ver.to_string()].clone();
    //     config["networkConfig"]["cn"]["content"]["configs"][old_func_ver.to_string()] = Value::Null;
    // }

    // write_json(CONFIG_JSON_PATH, config).unwrap();
}
