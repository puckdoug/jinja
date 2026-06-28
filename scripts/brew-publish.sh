#!/bin/sh
# scripts/brew-publish.sh — commit and push the rendered formula in the tap repo.
set -eu
PATH=~/.cargo/bin:$PATH
export PATH
cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib.sh
. scripts/lib.sh

ver="$(repo_version)"
tapdir="$(tap_checkout)"
cd "$tapdir" || { echo "Tap missing — run: brew tap-new $TAP" >&2; exit 1; }

git add Formula/jinja.rb
if git diff --cached --quiet; then
  echo "No formula changes to publish."
else
  git commit -m "jinja $ver"
  git push
fi
