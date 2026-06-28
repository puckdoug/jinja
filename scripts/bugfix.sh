#!/bin/sh
# scripts/bugfix.sh — bump the patch version and commit so release tags the right code.
set -eu
cd "$(dirname "$0")/.."

ver=$(awk -F'"' '/^version[[:space:]]*=/ { print $2; exit }' Cargo.toml)
new_ver=$(echo "$ver" | awk -F. '{print $1"."$2"."$3+1}')
sed -i '' "s/^version = \"$ver\"/version = \"$new_ver\"/" Cargo.toml

# Update Cargo.lock to match
cargo update --package jinja 2>/dev/null

git add Cargo.toml Cargo.lock
git commit -m "bugfix: bump version $ver -> $new_ver"
echo "Bumped $ver -> $new_ver"
