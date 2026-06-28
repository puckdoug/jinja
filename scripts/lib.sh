# scripts/lib.sh — shared config + helpers for the packaging scripts (sourced, not run).
# shellcheck shell=sh

REPO="puckdoug/jinja"  # GitHub repo holding the source and releases
TAP="puckdoug/tap"     # Homebrew tap holding the published formulae

# Version from Cargo.toml's [package] table, e.g. "0.1.0".
repo_version() {
  awk -F'"' '/^version[[:space:]]*=/ { print $2; exit }' Cargo.toml
}

# GitHub source-tarball URL for a version (arg 1).
tarball_url() {
  echo "https://github.com/${REPO}/archive/refs/tags/v${1}.tar.gz"
}

# On-disk tap checkout (portable: Intel/ARM/Linuxbrew prefixes differ).
tap_checkout() {
  HOMEBREW=$(brew --repository)
  echo "${HOMEBREW}/Library/Taps/puckdoug/homebrew-tap"
}
