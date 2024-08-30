use std::{collections::HashMap, io::BufWriter};

use serde::{Deserialize, Serialize};
use serde_json::{from_value, Serializer as JsonSer};

use crate::SERVER_CONFIG;
use common_utils::read_json;

#[derive(Serialize, Deserialize)]
struct ProdAndroidNetworkConfig {
    sign: String,
    pub content: NetworkConfigContent,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct NetworkConfigContent {
    config_ver: String,
    pub configs: HashMap<String, NwCfg>,
    func_ver: String,
}

impl NetworkConfigContent {
    fn process(mut self, server: &str) -> Self {
        let cfgv = self.config_ver;
        let fv = self.func_ver;
        let cfg = self.configs.remove(&fv);
        if let Some(cfg) = cfg {
            let ord = cfg.ord;
            let secure = cfg.network.secure;
            let sl = cfg.network.sl;
            let of = cfg.network.of;
            let gs = cfg.network.gs.replace("{server}", server);
            let ak_server = cfg.network.ak_server.replace("{server}", server);
            let ue = cfg.network.ue.replace("{server}", server);
            let hu = cfg.network.hu.replace("{server}", server);
            let hv = cfg.network.hv.replace("{server}", server);
            let rc = cfg.network.rc.replace("{server}", server);
            let an = cfg.network.an.replace("{server}", server);
            let prean = cfg.network.prean.replace("{server}", server);
            NetworkConfigContent {
                config_ver: cfgv,
                configs: {
                    let mut cfgs = HashMap::new();
                    cfgs.insert(
                        fv.clone(),
                        NwCfg {
                            ord,
                            network: NwCfgContent {
                                gs,
                                ak_server,
                                ue,
                                hu,
                                hv,
                                rc,
                                an,
                                prean,
                                sl,
                                of,
                                pkg_ad: (),
                                pkg_ios: (),
                                secure,
                            },
                        },
                    );
                    cfgs
                },
                func_ver: fv,
            }
        } else {
            NetworkConfigContent {
                config_ver: String::from("5"),
                configs: HashMap::new(),
                func_ver: String::from("052"),
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
struct NwCfg {
    pub network: NwCfgContent,
    #[serde(rename = "override")]
    ord: bool,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
struct NwCfgContent {
    pub gs: String,
    #[serde(rename = "as")]
    pub ak_server: String,
    #[serde(rename = "u8")]
    pub ue: String,
    pub hu: String,
    pub hv: String,
    pub rc: String,
    pub an: String,
    pub prean: String,
    pub sl: String,
    pub of: String,
    pkg_ad: (),
    pkg_ios: (),
    secure: bool,
}

#[derive(Serialize, Deserialize)]
pub struct ProdAndroidNetwork {
    sign: String,
    content: String,
}

impl ProdAndroidNetwork {
    pub fn load() -> Self {
        let mode = SERVER_CONFIG.mode.to_lowercase();
        let host = SERVER_CONFIG.host.as_str();
        let port = SERVER_CONFIG.port;

        let server = format!("http://{}:{}", host, port);

        let cfg = from_value::<HashMap<String, ProdAndroidNetworkConfig>>(read_json("./config/network.json")).unwrap().remove(&mode).unwrap();

        let content = cfg.content.process(&server);

        let content = {
            let mut buf = Vec::<u8>::new();

            let mut ser = JsonSer::new(&mut buf);
            content.serialize(&mut ser).unwrap();

            String::from_utf8(buf).unwrap()
        };

        Self { sign: String::from("sign"), content }
    }
}
