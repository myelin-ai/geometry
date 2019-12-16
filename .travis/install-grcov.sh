#!/usr/bin/env bash

set -e

version=0.5.5
archive_name="grcov-linux-x86_64"
archive_file="$archive_name.tar.bz2"
download_url="https://github.com/mozilla/grcov/releases/download/$version/$archive_file"
temp_directory=$(mktemp -d)
executable_name=grcov

cd "$temp_directory"

wget "$download_url"
tar -xf "$archive_file"
sudo cp "$executable_name" /usr/local/bin/
