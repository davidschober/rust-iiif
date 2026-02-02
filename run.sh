#!/bin/zsh

# Check if brew is installed and get vips/glib paths
if command -v brew >/dev/null 2>&1; then
    VIPS_PREFIX=$(brew --prefix vips 2>/dev/null)
    GLIB_PREFIX=$(brew --prefix glib 2>/dev/null)
    GETTEXT_PREFIX=$(brew --prefix gettext 2>/dev/null)

    if [ -n "$VIPS_PREFIX" ]; then
        export LIBRARY_PATH="$VIPS_PREFIX/lib:$GLIB_PREFIX/lib:$GETTEXT_PREFIX/lib:$LIBRARY_PATH"
        export DYLD_LIBRARY_PATH="$VIPS_PREFIX/lib:$GLIB_PREFIX/lib:$GETTEXT_PREFIX/lib:$DYLD_LIBRARY_PATH"
    fi
fi

# Fallback to pkg-config if LIBRARY_PATH is still empty or as a backup
if command -v pkg-config >/dev/null 2>&1; then
    PKG_LIBS=$(pkg-config --libs-only-L vips glib-2.0 2>/dev/null | sed 's/-L//g' | tr ' ' ':')
    if [ -n "$PKG_LIBS" ]; then
        export LIBRARY_PATH="$PKG_LIBS:$LIBRARY_PATH"
        export DYLD_LIBRARY_PATH="$PKG_LIBS:$DYLD_LIBRARY_PATH"
    fi
fi

echo "Starting Rust IIIF Server..."
cargo run --release
