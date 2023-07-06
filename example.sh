runs_on="macos-latest"

if [ $runs_on == "macos-latest" ]; then
    echo "mac"
elif [ $runs_on == "linux-latest" ]; then
    echo "linux"
else
    echo "else"
fi