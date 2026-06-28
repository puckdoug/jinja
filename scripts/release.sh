#!/bin/sh
# scripts/release.sh — tag the version, push it, and cut a GitHub release.
set -eu
cd "$(dirname "$0")/.."
# shellcheck source=scripts/lib.sh
. scripts/lib.sh

ver="$(repo_version)"
tag="v$ver"

if git rev-parse -q --verify "refs/tags/$tag" >/dev/null; then
  echo "Tag $tag already exists; not re-tagging."
else
  git tag -a "$tag" -m "Release $tag"
  git push origin "$tag"
fi

if gh release view "$tag" >/dev/null 2>&1; then
  echo "Release $tag already exists."
else
  gh release create "$tag" --title "$tag" --generate-notes
fi
