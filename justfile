default: check

check:
  cargo check
  cargo clippy

test:
  cargo test

build:
  cargo build

clean:
  echo "clean"

realclean: clean
  rm -rf target
