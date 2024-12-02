if [[ $(git diff HEAD --name-only) ]]; then
    echo "detected uncommited files!"
    exit 1
fi
