echo "Generate Graphs"

cargo deps --all-deps | dot -Tpng | save -f graphs/dependencies.png
