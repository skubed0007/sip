#!/bin/bash
set -e

echo "🔧 Starting multi-target static build for workspace binaries..."

# Define all binaries you want to build
BINARIES=("sipc" "sipfmt")  # These must match your [[bin]] targets or bin crate names
BUILD_DIR="bin"

# Define all targets
TARGETS=(
  "x86_64-unknown-linux-gnu"
  "x86_64-unknown-linux-musl"
  "i686-unknown-linux-gnu"
  "x86_64-pc-windows-gnu"
  "i686-pc-windows-gnu"
)

mkdir -p "$BUILD_DIR"

echo "📦 Adding required targets..."
for target in "${TARGETS[@]}"; do
  rustup target add "$target"
done
echo "✅ Targets ready."

# Loop through each binary and build for each target
for BIN in "${BINARIES[@]}"; do
  for target in "${TARGETS[@]}"; do
    echo "🔨 Building $BIN for $target..."

    # Reset env per target
    export RUSTFLAGS=""
    EXT=""

    case "$target" in
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

    # Build using Cargo with workspace support
    cargo build --release --bin "$BIN" --target "$target"

    # Copy result to bin/ with clean naming
    cp "target/$target/release/$BIN$EXT" "$BUILD_DIR/${BIN}-${target}${EXT}"
    echo "✅ Built $BIN for $target"
  done
done

echo "🎉 All builds complete! Check the '$BUILD_DIR/' directory."
