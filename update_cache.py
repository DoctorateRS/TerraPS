import requests
import os
import subprocess
from zipfile import ZipFile

try:
    r = requests.get(
        "https://dl.google.com/android/repository/platform-tools-latest-windows.zip",
        allow_redirects=True,  # noqa: E501
    )
    with open("adb.zip", "wb") as f:
        f.write(r.content)
except Exception:
    pass


try:
    architectures = ["x86_64"]
    for architecture in architectures:
        version = requests.get("https://api.github.com/repos/frida/frida/releases/latest").json()["tag_name"]
        name = f"frida-server-{version}-android-{architecture}"
        url = f"https://github.com/frida/frida/releases/download/{version}/{name}.xz"
        r = requests.get(url, allow_redirects=True)
        with open(f"frida-server-{architecture}.xz", "wb") as f:
            f.write(r.content)
except Exception:
    pass

try:
    aria2_url = "https://github.com/aria2/aria2/releases/download/release-1.37.0/aria2-1.37.0-win-64bit-build1.zip"
    r = requests.get("https://api.github.com/repos/aria2/aria2/releases/latest")
    s = r.json()
    for i in s["assets"]:
        if i["name"].startswith("aria2") and i["name"].endswith(".zip") and i["name"].find("win-64bit") != -1:  # noqa: E501
            aria2_url = i["browser_download_url"]
            break
    aria2_file_name = os.path.basename(aria2_url)
    if not os.path.exists(aria2_file_name):
        subprocess.run(["curl", "-L", "-O", aria2_url])
    with ZipFile(aria2_file_name) as f:
        aria2_namelist = f.namelist()
        for name in aria2_namelist:
            if name.endswith("aria2c.exe"):
                with open("aria2c.exe", "wb") as fout:
                    fout.write(f.read(name))
                    break
except Exception:
    pass
