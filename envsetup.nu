python -m ensurepip --upgrade
pip install uv
uv venv
uv pip install -r requirements.txt
rustup toolchain install stable

echo "Setup build environment completed."