#!/bin/sh -x
# compute the sha256, render the formula into the tap, validate it.
set -eu
PATH=~/.cargo/bin:$PATH
export PATH
cd "$(dirname "$0")/.."
. scripts/lib.sh

ver="$(repo_version)"
url="$(tarball_url "$ver")"
tapdir="$(tap_checkout)"
[ -d "$tapdir" ] || { echo "Tap missing — run: brew tap-new $TAP" >&2; exit 1; }

tarball="$(mktemp "${TMPDIR:-/tmp}/jinja-tarball.XXXXXX")"
trap 'rm -f "$tarball"' EXIT
echo "Fetching $url"
# ponytail: retry loop — GitHub archive CDN can lag a few seconds after a tag push
i=0
until curl -fsSL "$url" -o "$tarball" 2>/dev/null; do
  i=$((i + 1))
  [ $i -ge 10 ] && { echo "Could not fetch tarball after retries — does the release exist?" >&2; exit 1; }
  echo "Waiting for GitHub CDN… (attempt $i/10)"
  sleep 6
done
sha="$(shasum -a 256 "$tarball" | awk '{ print $1 }')"
echo "sha256 = $sha"

sed \
  -e "s|{{ *version *}}|$ver|g" \
  -e "s|{{ *sha256 *}}|$sha|g" \
  packaging/jinja.rb.tmpl > "$tapdir/Formula/jinja.rb"
echo "Wrote $tapdir/Formula/jinja.rb"

brew style "$tapdir/Formula/jinja.rb"
brew audit --strict --online "$TAP/jinja"
brew install --build-from-source "$TAP/jinja"
brew test "$TAP/jinja"
