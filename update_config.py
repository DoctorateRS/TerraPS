import requests
import json

with open("config/config.json") as f:
    config = json.load(f)

old_resVersion = config["version"]["android"]["resVersion"]
old_clientVersion = config["version"]["android"]["clientVersion"]

old_funcVer = config["networkConfig"]["cn"]["content"]["funcVer"]

timeout = config["server"]["timeout"]

try:
    version = requests.get("https://ak-conf.hypergryph.com/config/prod/official/Android/version", timeout=timeout).json()
    resVersion = version["resVersion"]
    clientVersion = version["clientVersion"]
    if resVersion != old_resVersion:
        config["version"]["android"]["resVersion"] = resVersion
    if clientVersion != old_clientVersion:
        config["version"]["android"]["clientVersion"] = clientVersion

except Exception:
    pass

try:
    network_config = requests.get("https://ak-conf.hypergryph.com/config/prod/official/network_config", timeout=timeout).json()
    content = json.loads(network_config["content"])
    funcVer = content["funcVer"]
    if funcVer != old_funcVer:
        config["networkConfig"]["cn"]["content"]["funcVer"] = funcVer
        config["networkConfig"]["cn"]["content"]["configs"][funcVer] = config["networkConfig"]["cn"]["content"]["configs"][old_funcVer]  # noqa: E501
        del config["networkConfig"]["cn"]["content"]["configs"][old_funcVer]

except Exception:
    pass

with open("config/config.json", "w") as f:
    json.dump(config, f, indent=4)
