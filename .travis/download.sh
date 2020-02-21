#!/bin/bash
set -eu

cd ..

for name in "$@"; do
	repo="https://github.com/Nu-SCPTheme/$name"
	echo "Cloning $repo..."
	git clone --depth=50 --branch=master "$repo"
	echo "Currently on commit:"
	(cd "$name" && git rev-parse HEAD)
done
