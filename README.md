# GMEXT-SpacetimeDB

Unofficial GameMaker extension/SDK for **SpacetimeDB** WebSocket v2: connect, subscribe, call reducers/procedures, and poll JSON events from GML.

**Not an official Clockwork Labs product.** SpacetimeDB itself: [clockworklabs/SpacetimeDB](https://github.com/clockworklabs/SpacetimeDB)

**Compatible with SpacetimeDB 2.8.0.**

Native core is **Rust** (extgen `nativeBackend: rust`), with a GML wrapper (`spdb_*`) in the sample project.

## Features

- WebSocket v2 + BSATN binary protocol
- Runtime schema registration → BSATN decode to JSON
- Brotli / Gzip compressed server messages
- Auto-reconnect with exponential backoff and subscription replay
- Schema helper CLI (`gen-gms-bindings`) to emit GML `spdb_register_*` calls from a module WASM/JSON

## Platforms

| Platform | Status |
|----------|--------|
| Windows | Supported (`.dll`) |
| macOS | Supported (`.dylib`) |
| Linux | Supported (`.so`) |
| Android | Supported (JNI + `.so` per ABI) |
| iOS | Supported (ObjC + embedded `SpacetimeDB_Rust.xcframework`) |
| tvOS | Supported (same pattern as iOS) |
| HTML5 / consoles | Not in this Rust build |

iOS/tvOS ship a **dynamic** XCFramework so this extension can sit next to other Rust GameMaker extensions without duplicate `std` symbols.

## Layout (extension)

```
extensions/SpacetimeDB/
  SpacetimeDB.yy
  source/          # extgen root (spec, rust/, scripts/)
  AndroidSource/
  iOSSource/ / tvOSSource/
  iOSSourceFromMac/ / tvOSSourceFromMac/   # built Apple frameworks
```

Build helpers: `source/scripts/build_*.{bat,sh}` (Windows, Android, macOS, Linux, iOS, tvOS).

## Quick start (Windows)

Open `source/SpacetimeDB_gml/SpacetimeDB_gml.yyp` in GameMaker. GML API:

```gml
var conn = spdb_connect("ws://localhost:3000", "my_database", "");
spdb_register_schema(conn, "player", [
    { name: "id", type: "u64" },
    { name: "name", type: "string" }
]);
spdb_poll(conn); // every Step
```

Low-level native exports are `stdb_*`. The wrapper lives in `scripts/SpacetimeDB_Wrapper`.

## Schema auto-generation

```bat
scripts\build_gen_gms_bindings.bat
source\SpacetimeDB_gml\datafiles\generate_gml_bindings.bat --wasm path\to\StdbModule.wasm
```

Then call the generated GML function after `spdb_connect()`.

## License

Apache-2.0
