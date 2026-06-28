# runs check and test
default: check test

# Cargo checks
check:
  cargo check
  cargo clippy

# Run rust tests
test:
  cargo test

# Build the app
build:
  @cargo build

# Bump the patch version (e.g. 0.1.0 -> 0.1.1)
bugfix:
  ./scripts/bugfix.sh

# Generate a full release
release:
  ./scripts/release.sh

# Generate all packages
packages: brew

# Generate a homebrew package
brew: build release
  brew uninstall jinja >/dev/null 2>&1 || true
  ./scripts/brew-package.sh

# Publish a homebrew package
brew-publish:
  ./scripts/brew-publish.sh

# Install the homebrew package
install:
  brew install puckdoug/tap/jinja

# Uninstall the homebrew package
uninstall:
  brew uninstall puckdoug/tap/jinja

# Clean up local temporary files
clean:
  @echo "clean"

# Remove build caches as well as regular cleaning
realclean: clean
  @rm -rf target
