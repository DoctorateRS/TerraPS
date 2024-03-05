from json import load, dump

try:
    with open("data/excel/activity_table.json", encoding="utf-8") as f:
        activity_table = load(f)

    activity_start_time_list = []

    for i in activity_table["basicInfo"]:
        if i.endswith("side") or i.endswith("sre") or i.endswith("mini") or i.endswith("bossrush"):
            startTime = activity_table["basicInfo"][i]["startTime"]
            endTime = activity_table["basicInfo"][i]["endTime"]
            activity_start_time_list.append((startTime, endTime))

    max_activity_start_time, max_activity_end_time = max(activity_start_time_list, key=lambda x: x[0])

    with open("config/config.json") as f:
        config = load(f)

    config["userConfig"]["activityMinStartTs"] = max_activity_start_time - 604800
    config["userConfig"]["activityMaxStartTs"] = max_activity_end_time + 604800

    with open("config/config.json", "w") as f:
        dump(config, f, indent=4)
except Exception:
    pass
