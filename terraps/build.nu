

cargo build -r -v | save -f build.log
overlay use .venv\Scripts\activate.nu
python package.py