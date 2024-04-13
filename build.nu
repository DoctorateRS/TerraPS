date now | format date "%Y-%m-%d %H:%M:%S\n" | save -a build.log
cargo build -r -v| save -a build.log
overlay use .venv\Scripts\activate.nu
python package.py