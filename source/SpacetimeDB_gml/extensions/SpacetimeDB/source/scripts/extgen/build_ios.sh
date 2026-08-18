#!/usr/bin/env bash
# ##### extgen :: generated core (scripts/extgen) — customize scripts/build_ios.sh #####
# Dynamic XCFramework: cdylib → {EXT}_Rust.framework → zip → targets.ios.outputFolder
# Usage: build_ios.sh [--device-only|--sim-only] [--skip-zip]
# YY: iosThirdPartyFrameworkEntries → {EXT}_Rust.xcframework embed:1
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$ROOT/rust"

export CARGO_TARGET_DIR="$ROOT/rust/target"

CRATE="spacetimedb"
EXT="SpacetimeDB"
DEST_REL="../iOSSourceFromMac"
FRAMEWORK_NAME="SpacetimeDB_Rust"
BUNDLE_ID="com.extgen.spacetimedb.rust"
MIN_IOS="13.0"
INSTALL_NAME="@executable_path/Frameworks/${FRAMEWORK_NAME}.framework/${FRAMEWORK_NAME}"
DYLIB_BASENAME="lib${CRATE}.dylib"
ZIP_NAME="${FRAMEWORK_NAME}.zip"

BUILD_DEVICE=1
BUILD_SIM=1
SKIP_ZIP=0

while [[ $# -gt 0 ]]; do
  case "$1" in
    --device-only) BUILD_SIM=0; shift ;;
    --sim-only) BUILD_DEVICE=0; shift ;;
    --skip-zip) SKIP_ZIP=1; shift ;;
    --help|-h)
      echo "Usage: $0 [--device-only|--sim-only] [--skip-zip]"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
done

export IPHONEOS_DEPLOYMENT_TARGET="$MIN_IOS"
export CARGO_PROFILE_RELEASE_DEBUG=1
export CARGO_PROFILE_RELEASE_STRIP=none

if [[ "$(uname)" != "Darwin" ]]; then
  echo "ERROR: iOS build requires macOS with Xcode." >&2
  exit 1
fi
if ! xcode-select -p &>/dev/null; then
  echo "ERROR: Xcode CLT not found. Run: xcode-select --install" >&2
  exit 1
fi

rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios >/dev/null 2>&1 || true

write_framework_plist() {
  local plist_path="$1"
  local platform="$2" # iPhoneOS | iPhoneSimulator
  cat > "$plist_path" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
	<key>CFBundleDevelopmentRegion</key>
	<string>en</string>
	<key>CFBundleExecutable</key>
	<string>${FRAMEWORK_NAME}</string>
	<key>CFBundleIdentifier</key>
	<string>${BUNDLE_ID}</string>
	<key>CFBundleInfoDictionaryVersion</key>
	<string>6.0</string>
	<key>CFBundleName</key>
	<string>${FRAMEWORK_NAME}</string>
	<key>CFBundlePackageType</key>
	<string>FMWK</string>
	<key>CFBundleShortVersionString</key>
	<string>1.0</string>
	<key>CFBundleVersion</key>
	<string>1</string>
	<key>MinimumOSVersion</key>
	<string>${MIN_IOS}</string>
	<key>CFBundleSupportedPlatforms</key>
	<array>
		<string>${platform}</string>
	</array>
</dict>
</plist>
EOF
}

LAST_FRAMEWORK_PATH=""
LAST_DSYM_PATH=""

make_framework_from_dylib() {
  local dylib_path="$1"
  local out_dir="$2"
  local platform="$3"

  if [[ ! -f "$dylib_path" ]]; then
    echo "ERROR: dylib not found: $dylib_path" >&2
    echo "Ensure Cargo.toml has crate-type including \"cdylib\"." >&2
    exit 1
  fi

  local fw="$out_dir/${FRAMEWORK_NAME}.framework"
  rm -rf "$fw" "$out_dir/${FRAMEWORK_NAME}.framework.dSYM"
  mkdir -p "$fw"
  cp "$dylib_path" "$fw/${FRAMEWORK_NAME}"
  chmod +w "$fw/${FRAMEWORK_NAME}"
  install_name_tool -id "$INSTALL_NAME" "$fw/${FRAMEWORK_NAME}"

  local id_out
  id_out="$(otool -D "$fw/${FRAMEWORK_NAME}" 2>/dev/null | tail -n 1 || true)"
  if [[ "$id_out" == *@rpath* ]]; then
    echo "ERROR: install name still uses @rpath: $id_out" >&2
    echo "Expected: $INSTALL_NAME" >&2
    exit 1
  fi

  write_framework_plist "$fw/Info.plist" "$platform"

  local dsym="$out_dir/${FRAMEWORK_NAME}.framework.dSYM"
  if dsymutil "$fw/${FRAMEWORK_NAME}" -o "$dsym" 2>/dev/null; then
    :
  else
    echo "WARNING: dsymutil failed for $fw"
    dsym=""
  fi
  codesign --force --sign - --timestamp=none "$fw" >/dev/null 2>&1 || true

  LAST_FRAMEWORK_PATH="$fw"
  LAST_DSYM_PATH="$dsym"
}

DEVICE_DYLIB=""
SIM_DYLIB=""

if [[ "$BUILD_DEVICE" -eq 1 ]]; then
  echo "--- Building cdylib aarch64-apple-ios ---"
  cargo build --release --target aarch64-apple-ios
  DEVICE_DYLIB="${CARGO_TARGET_DIR}/aarch64-apple-ios/release/${DYLIB_BASENAME}"
fi

if [[ "$BUILD_SIM" -eq 1 ]]; then
  echo "--- Building cdylib aarch64-apple-ios-sim ---"
  cargo build --release --target aarch64-apple-ios-sim
  echo "--- Building cdylib x86_64-apple-ios ---"
  cargo build --release --target x86_64-apple-ios
  mkdir -p "${CARGO_TARGET_DIR}/ios-sim-release"
  SIM_DYLIB="${CARGO_TARGET_DIR}/ios-sim-release/${DYLIB_BASENAME}"
  lipo -create \
    "${CARGO_TARGET_DIR}/aarch64-apple-ios-sim/release/${DYLIB_BASENAME}" \
    "${CARGO_TARGET_DIR}/x86_64-apple-ios/release/${DYLIB_BASENAME}" \
    -output "$SIM_DYLIB"
fi

FRAMEWORK_TMPDIR="$(mktemp -d)"
trap 'rm -rf "$FRAMEWORK_TMPDIR"' EXIT
DEVICE_FW_DIR="$FRAMEWORK_TMPDIR/device"
SIM_FW_DIR="$FRAMEWORK_TMPDIR/sim"
mkdir -p "$DEVICE_FW_DIR" "$SIM_FW_DIR"

XCFRAMEWORK_ARGS=()
if [[ "$BUILD_DEVICE" -eq 1 ]]; then
  make_framework_from_dylib "$DEVICE_DYLIB" "$DEVICE_FW_DIR" "iPhoneOS"
  XCFRAMEWORK_ARGS+=(-framework "$LAST_FRAMEWORK_PATH")
  if [[ -n "${LAST_DSYM_PATH}" && -d "${LAST_DSYM_PATH}" ]]; then
    XCFRAMEWORK_ARGS+=(-debug-symbols "$LAST_DSYM_PATH")
  fi
fi
if [[ "$BUILD_SIM" -eq 1 ]]; then
  make_framework_from_dylib "$SIM_DYLIB" "$SIM_FW_DIR" "iPhoneSimulator"
  XCFRAMEWORK_ARGS+=(-framework "$LAST_FRAMEWORK_PATH")
  if [[ -n "${LAST_DSYM_PATH}" && -d "${LAST_DSYM_PATH}" ]]; then
    XCFRAMEWORK_ARGS+=(-debug-symbols "$LAST_DSYM_PATH")
  fi
fi

xcodebuild -create-xcframework \
  "${XCFRAMEWORK_ARGS[@]}" \
  -output "$FRAMEWORK_TMPDIR/${FRAMEWORK_NAME}.xcframework"

if find "$FRAMEWORK_TMPDIR/${FRAMEWORK_NAME}.xcframework" -name '*.a' | grep -q .; then
  echo "WARNING: XCFramework contains .a — expected dynamic frameworks only."
fi

case "$DEST_REL" in
  /*|[A-Za-z]:*) DEST="$DEST_REL" ;;
  *) DEST="$ROOT/$DEST_REL" ;;
esac

if [[ "$SKIP_ZIP" -eq 1 ]]; then
  echo "Skipping zip. XCFramework: $FRAMEWORK_TMPDIR/${FRAMEWORK_NAME}.xcframework"
  trap - EXIT
else
  mkdir -p "$DEST"
  rm -f "$DEST/$ZIP_NAME"
  (cd "$FRAMEWORK_TMPDIR" && zip -r -q "$DEST/$ZIP_NAME" "${FRAMEWORK_NAME}.xcframework")
  echo "Deployed iOS -> $DEST/$ZIP_NAME"
fi
