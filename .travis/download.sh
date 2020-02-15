#!/bin/bash
set -eu

cd ..

for name in "$@"; do
	repo="https://github.com/Nu-SCPTheme/$name"
	echo "Cloning $repo..."
	git clone --depth=50 --branch=master "$repo"
done
