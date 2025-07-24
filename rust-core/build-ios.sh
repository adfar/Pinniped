#!/bin/bash
set -e

# Build script for iOS static library and XCFramework
echo "🦭 Building Pinniped Core for iOS..."

# Configuration
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
BUILD_DIR="$SCRIPT_DIR/target"
INCLUDE_DIR="$SCRIPT_DIR/include"
LIB_NAME="libpinniped_core.a"

# iOS targets
IOS_TARGETS=(
    "aarch64-apple-ios"           # iOS device (ARM64)
    "aarch64-apple-ios-sim"       # iOS simulator (ARM64 - M1 Macs)
    "x86_64-apple-ios"            # iOS simulator (Intel Macs)
)

# Function to install target if not present
install_target_if_needed() {
    local target=$1
    if ! rustup target list --installed | grep -q "^$target\$"; then
        echo "📥 Installing Rust target: $target"
        rustup target add $target
    else
        echo "✅ Target already installed: $target"
    fi
}

# Function to build for a specific target
build_target() {
    local target=$1
    echo "🔨 Building for target: $target"
    
    # Install target if needed
    install_target_if_needed $target
    
    # Build
    cargo build --target $target --release
    
    # Check in parent directory target (workspace build)
    PARENT_BUILD_DIR="../target"
    if [ -f "$PARENT_BUILD_DIR/$target/release/$LIB_NAME" ]; then
        echo "✅ Build successful for $target (found in parent target directory)"
    elif [ -f "$BUILD_DIR/$target/release/$LIB_NAME" ]; then
        echo "✅ Build successful for $target"
    else
        echo "❌ Build failed for $target - library not found"
        echo "   Checked: $BUILD_DIR/$target/release/$LIB_NAME"
        echo "   Checked: $PARENT_BUILD_DIR/$target/release/$LIB_NAME"
        exit 1
    fi
}

# Clean previous builds
echo "🧹 Cleaning previous builds..."
cargo clean

# Build for all iOS targets
for target in "${IOS_TARGETS[@]}"; do
    build_target $target
done

# Create universal library for simulators
echo "🔗 Creating universal simulator library..."
PARENT_BUILD_DIR="../target"
UNIVERSAL_SIM_DIR="$PARENT_BUILD_DIR/universal-ios-sim"
mkdir -p $UNIVERSAL_SIM_DIR

# Use parent target directory where files actually are
lipo -create \
    "$PARENT_BUILD_DIR/aarch64-apple-ios-sim/release/$LIB_NAME" \
    "$PARENT_BUILD_DIR/x86_64-apple-ios/release/$LIB_NAME" \
    -output "$UNIVERSAL_SIM_DIR/$LIB_NAME"

echo "✅ Universal simulator library created"

# Verify the libraries
echo "🔍 Verifying libraries..."
for target in "${IOS_TARGETS[@]}"; do
    lib_path="$PARENT_BUILD_DIR/$target/release/$LIB_NAME"
    if [ -f "$lib_path" ]; then
        echo "📦 $target: $(file "$lib_path")"
    fi
done

echo "📦 Universal simulator: $(file "$UNIVERSAL_SIM_DIR/$LIB_NAME")"

# Create XCFramework
echo "📱 Creating XCFramework..."
XCFRAMEWORK_DIR="$PARENT_BUILD_DIR/PinnipedCore.xcframework"

# Remove existing XCFramework
rm -rf "$XCFRAMEWORK_DIR"

# Check if xcodebuild is available
if ! command -v xcodebuild &> /dev/null; then
    echo "⚠️  xcodebuild not found. Please install Xcode Command Line Tools."
    echo "   You can create the XCFramework manually using:"
    echo "   xcodebuild -create-xcframework \\"
    echo "     -library $PARENT_BUILD_DIR/aarch64-apple-ios/release/$LIB_NAME \\"
    echo "     -headers $INCLUDE_DIR \\"
    echo "     -library $UNIVERSAL_SIM_DIR/$LIB_NAME \\"
    echo "     -headers $INCLUDE_DIR \\"
    echo "     -output $XCFRAMEWORK_DIR"
    exit 0
fi

# Create XCFramework
xcodebuild -create-xcframework \
    -library "$PARENT_BUILD_DIR/aarch64-apple-ios/release/$LIB_NAME" \
    -headers "$INCLUDE_DIR" \
    -library "$UNIVERSAL_SIM_DIR/$LIB_NAME" \
    -headers "$INCLUDE_DIR" \
    -output "$XCFRAMEWORK_DIR"

echo "✅ XCFramework created at: $XCFRAMEWORK_DIR"

# Display size information
echo "📊 Library sizes:"
for target in "${IOS_TARGETS[@]}"; do
    lib_path="$PARENT_BUILD_DIR/$target/release/$LIB_NAME"
    if [ -f "$lib_path" ]; then
        size=$(stat -f%z "$lib_path" 2>/dev/null || stat -c%s "$lib_path" 2>/dev/null || echo "Unknown")
        echo "   $target: $size bytes"
    fi
done

universal_size=$(stat -f%z "$UNIVERSAL_SIM_DIR/$LIB_NAME" 2>/dev/null || stat -c%s "$UNIVERSAL_SIM_DIR/$LIB_NAME" 2>/dev/null || echo "Unknown")
echo "   Universal simulator: $universal_size bytes"

# Test the build
echo "🧪 Testing FFI functions..."
if cargo test --release; then
    echo "✅ All tests passed"
else
    echo "⚠️  Some tests failed, but build completed"
fi

echo ""
echo "🎉 iOS build complete!"
echo "📁 Outputs:"
echo "   • Static libraries: $PARENT_BUILD_DIR/*/release/$LIB_NAME"
echo "   • Universal simulator: $UNIVERSAL_SIM_DIR/$LIB_NAME"
echo "   • XCFramework: $XCFRAMEWORK_DIR"
echo "   • Headers: $INCLUDE_DIR/pinniped_core.h"
echo ""
echo "📋 Next steps:"
echo "   1. Copy the XCFramework to your iOS project"
echo "   2. Add it to your project's 'Frameworks, Libraries, and Embedded Content'"
echo "   3. Import the Swift wrapper in your iOS app"