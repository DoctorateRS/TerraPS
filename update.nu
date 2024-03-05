#!/usr/bin/env nu

echo "Starting update..."
overlay use ./.venv/Scripts/activate.nu

echo "Updating config..."
python3 update_config.py

echo "Updating game..."
python3 update_game.py

echo "Deleting old data..."
rm -r data/excel/

echo "Fetching new data..."
mkdir tmp/
cd tmp/
git clone -n --depth=1 --filter=tree:0 https://github.com/Kengxxiao/ArknightsGameData.git .
git config core.sparsecheckout true
git sparse-checkout set --no-cone zh_CN/gamedata/excel/
git checkout
mv zh_CN/gamedata/excel/ ../data/
cd ..
rm -r tmp/

echo "Updating activities..."
python3 update_activity.py

echo "Generating custom banner..."
python3 generate_gacha.py

git add -A --sparse .
git commit -m "Data Update"
git push origin main
git sparse-checkout disable

echo "Update finished. Exiting..."
cls
