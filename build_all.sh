#!/bin/bash
set -e

echo "=== File Encryption Tool - Full Build ==="
echo ""

# Detect current platform
ARCH=$(uname -m)
OS=$(uname -s)
echo "Host: $OS $ARCH"
echo ""

STUBS_DIR="build_stubs"
RELEASE_DIR="release"
mkdir -p "$STUBS_DIR"

# ── CLI binaries (platform stubs) ──────────────────────────
echo "── Building CLI stubs ──"

# Native build
cargo build --release --bin task
if [ "$OS" = "Darwin" ]; then
    cp target/release/task "$STUBS_DIR/task_unix"
    echo "  Built: task_unix (native)"
fi
if [ "$OS" = "Linux" ] && [ "$ARCH" = "x86_64" ]; then
    cp target/release/task "$STUBS_DIR/task_linux"
    echo "  Built: task_linux (native)"
fi
if [ "$OS" = "Linux" ] && [ "$ARCH" = "aarch64" ]; then
    cp target/release/task "$STUBS_DIR/task_linux_arm64"
    echo "  Built: task_linux_arm64 (native)"
fi

# Cross-compile CLI stubs
CLI_CROSS=(
    "x86_64-pc-windows-gnu:task.exe"
    "x86_64-unknown-linux-gnu:task_linux"
    "aarch64-unknown-linux-musl:task_linux_arm64"
)

for entry in "${CLI_CROSS[@]}"; do
    TARGET="${entry%%:*}"
    OUTNAME="${entry##*:}"
    # Skip if we already have this stub from native build
    [ -f "$STUBS_DIR/$OUTNAME" ] && continue
    if rustup target list --installed 2>/dev/null | grep -q "$TARGET"; then
        echo "  Building CLI for $TARGET..."
        if cargo build --release --bin task --target "$TARGET" 2>/dev/null; then
            # Find the binary (may be in different subdirs)
            BIN="target/$TARGET/release/task"
            [ "$OUTNAME" = "task.exe" ] && BIN="target/$TARGET/release/task.exe"
            if [ -f "$BIN" ]; then
                cp "$BIN" "$STUBS_DIR/$OUTNAME"
                echo "  Built: $OUTNAME"
            fi
        else
            echo "  (skipped $TARGET - linker not available)"
        fi
    fi
done

echo ""
echo "Stubs collected in $STUBS_DIR/:"
ls -lh "$STUBS_DIR/" 2>/dev/null || true
echo ""

# ── GUI binaries ─────────────────────────────────────────────
echo "── Building GUI ──"

# Map: target triple -> release folder name -> GUI binary name
declare -A GUI_MAP
GUI_MAP["native"]="native:file-encryption-gui"

# Determine native release folder name
if [ "$OS" = "Darwin" ] && [ "$ARCH" = "arm64" ]; then
    NATIVE_DIR="macos-arm64"
elif [ "$OS" = "Darwin" ] && [ "$ARCH" = "x86_64" ]; then
    NATIVE_DIR="macos-x86_64"
elif [ "$OS" = "Linux" ] && [ "$ARCH" = "x86_64" ]; then
    NATIVE_DIR="linux-x86_64"
elif [ "$OS" = "Linux" ] && [ "$ARCH" = "aarch64" ]; then
    NATIVE_DIR="linux-arm64"
else
    NATIVE_DIR="unknown"
fi

# Build native GUI
cargo build --release --bin file-encryption-gui
echo "  Built native GUI"

# Package native GUI
DEST="$RELEASE_DIR/$NATIVE_DIR"
mkdir -p "$DEST/bin"
cp target/release/file-encryption-gui "$DEST/"
cp "$STUBS_DIR"/* "$DEST/bin/" 2>/dev/null || true
echo "  Packaged: $DEST/"

# Cross-compile GUI for other platforms
GUI_CROSS=(
    "x86_64-apple-darwin:macos-x86_64:file-encryption-gui"
    "x86_64-pc-windows-gnu:windows-x86_64:file-encryption-gui.exe"
    "x86_64-unknown-linux-gnu:linux-x86_64:file-encryption-gui"
    "aarch64-unknown-linux-musl:linux-arm64:file-encryption-gui"
)

for entry in "${GUI_CROSS[@]}"; do
    IFS=':' read -r TARGET DIR_NAME BIN_NAME <<< "$entry"
    # Skip if same as native
    [ "$DIR_NAME" = "$NATIVE_DIR" ] && continue
    if rustup target list --installed 2>/dev/null | grep -q "$TARGET"; then
        echo "  Building GUI for $TARGET..."
        if cargo build --release --bin file-encryption-gui --target "$TARGET" 2>/dev/null; then
            DEST="$RELEASE_DIR/$DIR_NAME"
            mkdir -p "$DEST/bin"
            cp "target/$TARGET/release/$BIN_NAME" "$DEST/" 2>/dev/null && {
                cp "$STUBS_DIR"/* "$DEST/bin/" 2>/dev/null || true
                echo "  Packaged: $DEST/"
            } || echo "  (skipped $TARGET - binary not found)"
        else
            echo "  (skipped $TARGET - build failed)"
        fi
    fi
done

echo ""
echo "=== Build complete ==="
echo ""
echo "Release packages:"
for dir in "$RELEASE_DIR"/*/; do
    [ -d "$dir" ] || continue
    echo "  $dir"
    ls -lh "$dir" 2>/dev/null | grep -v "^total" | head -5
    echo "    bin/:"
    ls -lh "${dir}bin/" 2>/dev/null | grep -v "^total" || echo "    (empty)"
    echo ""
done
