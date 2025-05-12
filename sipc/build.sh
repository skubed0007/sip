#!/bin/bash
set -e

echo "🔧 Starting multi-target static build..."

# Your binary name
BIN_NAME="sip"
BUILD_DIR="bin"

mkdir -p "$BUILD_DIR"

TARGETS=(
  "x86_64-unknown-linux-gnu"
  "x86_64-unknown-linux-musl"
  "i686-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
  "i686-pc-windows-gnu"
)

# Add targets
for target in "${TARGETS[@]}"; do
  rustup target add "$target"
done

echo "✅ All targets added."

for target in "${TARGETS[@]}"; do
  echo "🔨 Building for $target..."

  # Reset environment
  export RUSTFLAGS=""
  EXT=""

  # Target-specific adjustments
  case $target in
    *-musl)
      # MUSL targets are statically linked by default
      ;;
    *-windows-gnu)
      export RUSTFLAGS="-C target-feature=+crt-static"
      EXT=".exe"
      ;;
    *-linux-gnu)
      export RUSTFLAGS="-C target-feature=+crt-static"
      ;;
  esac

  # Build
  cargo build --release --target "$target"

  # Output
  cp "target/$target/release/$BIN_NAME$EXT" "$BUILD_DIR/${BIN_NAME}-${target}$EXT"
  echo "✅ Built $BIN_NAME for $target"
done

echo "🎉 All builds complete! Binaries in '$BUILD_DIR/'"
