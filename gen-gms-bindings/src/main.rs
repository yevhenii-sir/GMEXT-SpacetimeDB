//! GameMaker Studio 2 GML schema bindings generator for SpacetimeDB.
//!
//! Reads a SpacetimeDB module (WASM or pre-extracted JSON) and generates
//! a `generated_spacetimedb_schemas.gml` file with:
//! - A `generated_shemas_spacetimedb()` function containing:
//!   - `spdb_register_struct(conn, ...)` calls for custom struct types
//!   - `spdb_register_schema(global.stdb_connection, ..., primary_key)` calls for public tables
//!   - `spdb_register_reducer(...)` calls for client-callable reducers
//! - Global `REDUCER_xxx()` helper functions for each reducer, with typed
//!   argument names (e.g. `_room_id_u64`) that call `spdb_call_reducer`

use anyhow::{anyhow, Context, Result};
use clap::Parser;
use spacetimedb_lib::db::raw_def::v9::TableAccess;
use spacetimedb_lib::sats::layout::PrimitiveType;
use spacetimedb_lib::sats::serde::SerdeWrapper;
use spacetimedb_lib::sats::AlgebraicTypeRef;
use spacetimedb_lib::RawModuleDef;
use spacetimedb_schema::def::{ModuleDef, ReducerDef, TableDef};
use spacetimedb_schema::identifier::NamespacePath;
use spacetimedb_schema::type_for_generate::{AlgebraicTypeDef, AlgebraicTypeUse, ProductTypeDef};
use std::collections::HashSet;
use std::fmt::Write;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

// ---------------------------------------------------------------------------
// CLI
// ---------------------------------------------------------------------------

#[derive(Parser, Debug)]
#[command(
    name = "gen-gms-bindings",
    about = "Generate GML schema bindings for GameMaker Studio 2 from SpacetimeDB modules"
)]
struct Cli {
    /// Path to compiled WASM module file (requires spacetimedb-standalone)
    #[arg(long, group = "input")]
    wasm: Option<PathBuf>,

    /// Path to pre-extracted JSON module definition (no spacetimedb-standalone needed)
    #[arg(long, group = "input")]
    json: Option<PathBuf>,

    /// Path to spacetimedb-standalone binary.
    /// If not specified, searches next to the current executable.
    #[arg(long)]
    standalone: Option<PathBuf>,

    /// Output GML file path
    #[arg(long, short, default_value = "generated_spacetimedb_schemas.gml")]
    out: PathBuf,
}

// ---------------------------------------------------------------------------
// ModuleDef extraction
// ---------------------------------------------------------------------------

/// Extract ModuleDef from a WASM file by invoking `spacetimedb-standalone extract-schema`.
fn extract_module_def_from_wasm(wasm_path: &PathBuf, standalone_path: &Option<PathBuf>) -> Result<ModuleDef> {
    let bin_path = match standalone_path {
        Some(p) => p.clone(),
        None => find_spacetimedb_standalone()?,
    };

    let child = Command::new(&bin_path)
        .arg("extract-schema")
        .arg(wasm_path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn {}", bin_path.display()))?;

    let output = child
        .wait_with_output()
        .context("Failed to wait for spacetimedb-standalone")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("spacetimedb-standalone extract-schema failed:\n{}", stderr);
    }

    let SerdeWrapper(module): SerdeWrapper<RawModuleDef> =
        serde_json::from_slice(&output.stdout).context("Failed to deserialize RawModuleDef from JSON")?;

    ModuleDef::try_from(module).map_err(|e| anyhow!("Failed to validate module definition: {:?}", e))
}

/// Extract ModuleDef from a pre-extracted JSON file.
fn extract_module_def_from_json(json_path: &PathBuf) -> Result<ModuleDef> {
    let file = fs::File::open(json_path)
        .with_context(|| format!("Failed to open JSON file: {}", json_path.display()))?;
    let reader = std::io::BufReader::new(file);

    let SerdeWrapper(module): SerdeWrapper<RawModuleDef> =
        serde_json::from_reader(reader).context("Failed to deserialize RawModuleDef from JSON")?;

    ModuleDef::try_from(module).map_err(|e| anyhow!("Failed to validate module definition: {:?}", e))
}

/// Find `spacetimedb-standalone` binary next to the current executable.
fn find_spacetimedb_standalone() -> Result<PathBuf> {
    let exe_path = std::env::current_exe().context("Failed to get current executable path")?;
    let exe_dir = exe_path
        .parent()
        .ok_or_else(|| anyhow!("No parent directory for current executable"))?;

    let standalone_name = if cfg!(windows) {
        "spacetimedb-standalone.exe"
    } else {
        "spacetimedb-standalone"
    };

    let sibling = exe_dir.join(standalone_name);
    if sibling.exists() {
        return Ok(sibling);
    }

    // Also try without .exe extension on Windows (in case it's a symlink)
    if cfg!(windows) {
        let sibling_no_ext = exe_dir.join("spacetimedb-standalone");
        if sibling_no_ext.exists() {
            return Ok(sibling_no_ext);
        }
    }

    anyhow::bail!(
        "Could not find spacetimedb-standalone next to the current executable ({}). \
         Please provide --standalone path or use --json instead.",
        exe_dir.display()
    )
}

// ---------------------------------------------------------------------------
// GML type mapping
// ---------------------------------------------------------------------------

/// Map a SpacetimeDB primitive type to a GML type string.
fn primitive_to_gml(prim: PrimitiveType) -> &'static str {
    match prim {
        PrimitiveType::Bool => "bool",
        PrimitiveType::I8 => "i32",   // promoted
        PrimitiveType::U8 => "u32",   // promoted
        PrimitiveType::I16 => "i32",  // promoted
        PrimitiveType::U16 => "u32",  // promoted
        PrimitiveType::I32 => "i32",
        PrimitiveType::U32 => "u32",
        PrimitiveType::I64 => "i64",
        PrimitiveType::U64 => "u64",
        PrimitiveType::I128 => "i64", // truncated with warning
        PrimitiveType::U128 => "u64", // truncated with warning
        PrimitiveType::I256 => "i64", // truncated with warning
        PrimitiveType::U256 => "u64", // truncated with warning
        PrimitiveType::F32 => "f32",
        PrimitiveType::F64 => "f64",
    }
}

/// Resolve an `AlgebraicTypeUse` to a GML type string.
fn resolve_type(ty: &AlgebraicTypeUse, module: &ModuleDef) -> String {
    match ty {
        AlgebraicTypeUse::Primitive(prim) => {
            let gml = primitive_to_gml(*prim);
            // Warn about truncation for large integer types
            match prim {
                PrimitiveType::I128 | PrimitiveType::I256 => {
                    eprintln!(
                        "Warning: {:?} is not natively supported in GML; mapping to i64 (truncated)",
                        prim
                    );
                }
                PrimitiveType::U128 | PrimitiveType::U256 => {
                    eprintln!(
                        "Warning: {:?} is not natively supported in GML; mapping to u64 (truncated)",
                        prim
                    );
                }
                _ => {}
            }
            gml.to_string()
        }
        AlgebraicTypeUse::String => "string".to_string(),
        AlgebraicTypeUse::Identity => "identity".to_string(),
        AlgebraicTypeUse::ConnectionId => "identity".to_string(),
        AlgebraicTypeUse::Timestamp => "u64".to_string(),
        AlgebraicTypeUse::TimeDuration => "i64".to_string(),
        AlgebraicTypeUse::Uuid => "string".to_string(),
        AlgebraicTypeUse::Array(elem) => format!("vec<{}>", resolve_type(elem, module)),
        AlgebraicTypeUse::Option(inner) => {
            // Option<T> is represented as vec<T> in GML (0 or 1 elements)
            format!("vec<{}>", resolve_type(inner, module))
        }
        AlgebraicTypeUse::Ref(r) => {
            // Look up the type definition to get its name
            match module.type_def_from_ref(*r) {
                Some((name, _def)) => name.name().to_string(),
                None => {
                    eprintln!("Warning: type ref {:?} not found in module definitions; using 'string' as fallback", r);
                    "string".to_string()
                }
            }
        }
        AlgebraicTypeUse::Result { .. } => {
            eprintln!("Warning: Result type not supported in GML; using 'string' as fallback");
            "string".to_string()
        }
        AlgebraicTypeUse::Unit => {
            eprintln!("Warning: Unit type not supported in GML; using 'bool' as fallback");
            "bool".to_string()
        }
        AlgebraicTypeUse::Never => {
            eprintln!("Warning: Never type encountered; using 'string' as fallback");
            "string".to_string()
        }
        AlgebraicTypeUse::ScheduleAt => {
            eprintln!("Warning: ScheduleAt type not supported in GML; using 'u64' as fallback");
            "u64".to_string()
        }
    }
}

// ---------------------------------------------------------------------------
// GML generation
// ---------------------------------------------------------------------------

/// Sanitize a GML type string for use in an identifier (e.g. `vec<string>` → `vec_string`).
fn sanitize_type_for_ident(gml_type: &str) -> String {
    gml_type
        .replace('<', "_")
        .replace('>', "")
        .replace(' ', "")
}

/// Sanitize a wire name (may contain `.` for submodule namespaces) for a GML identifier.
fn sanitize_name_for_ident(name: &str) -> String {
    name.replace('.', "_")
}

/// Wire name for a table: local name at root, dotted `ns.table` under submodules.
fn table_wire_name(path: &NamespacePath, table: &TableDef) -> String {
    path.join(table.name.clone()).to_string()
}

/// Depth-first visit of a module and all nested submodules.
fn visit_modules<'a>(module: &'a ModuleDef, f: &mut dyn FnMut(&'a ModuleDef)) {
    f(module);
    for submodule in module.submodules().values() {
        visit_modules(submodule, f);
    }
}

/// Generate the complete GML file content from a ModuleDef.
fn generate_gml(module: &ModuleDef) -> String {
    let mut output = String::new();

    // Header
    writeln!(output, "///@desc Auto-generated SpacetimeDB schema registrations").unwrap();
    writeln!(output).unwrap();

    // Open the wrapper function
    writeln!(output, "function generated_shemas_spacetimedb() {{").unwrap();
    writeln!(output).unwrap();

    // Collect table type refs to exclude from struct registration.
    // Include ALL tables across submodules (public + private) since private
    // table row types should also not be registered as standalone structs.
    let mut table_type_refs: HashSet<AlgebraicTypeRef> = HashSet::new();
    visit_modules(module, &mut |m| {
        for t in m.tables() {
            table_type_refs.insert(t.product_type_ref);
        }
    });

    // 1. Structs (must be first because tables/reducers reference them)
    let structs = generate_structs(module, &table_type_refs);
    if !structs.is_empty() {
        writeln!(output, "    // ==========================================").unwrap();
        writeln!(output, "    // 1. STRUCTURES (REGISTER FIRST!)").unwrap();
        writeln!(output, "    // ==========================================").unwrap();
        writeln!(output).unwrap();
        // Indent each line inside the function
        for line in structs.lines() {
            writeln!(output, "    {}", line).unwrap();
        }
    }

    // 2. Tables
    let tables = generate_tables(module);
    if !tables.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "    // ==========================================").unwrap();
        writeln!(output, "    // 2. TABLES").unwrap();
        writeln!(output, "    // ==========================================").unwrap();
        writeln!(output).unwrap();
        for line in tables.lines() {
            writeln!(output, "    {}", line).unwrap();
        }
    }

    // 3. Reducers (registration only)
    let reducers = generate_reducers(module);
    if !reducers.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "    // ==========================================").unwrap();
        writeln!(output, "    // 3. REDUCERS (registration)").unwrap();
        writeln!(output, "    // ==========================================").unwrap();
        writeln!(output).unwrap();
        for line in reducers.lines() {
            writeln!(output, "    {}", line).unwrap();
        }
    }

    // Close the wrapper function
    writeln!(output, "}}").unwrap();

    // 4. Reducer helper functions (global, outside the wrapper function)
    let reducer_helpers = generate_reducer_helpers(module);
    if !reducer_helpers.is_empty() {
        writeln!(output).unwrap();
        writeln!(output, "// ==========================================").unwrap();
        writeln!(output, "// 4. REDUCER HELPER FUNCTIONS (global)").unwrap();
        writeln!(output, "// ==========================================").unwrap();
        writeln!(output).unwrap();
        output.push_str(&reducer_helpers);
    }

    output
}

/// Generate `spdb_register_struct` calls for all non-table product types.
fn generate_structs(module: &ModuleDef, table_type_refs: &HashSet<AlgebraicTypeRef>) -> String {
    let mut output = String::new();

    visit_modules(module, &mut |m| {
        let typespace = m.typespace_for_generate();
        for type_def in m.types() {
            // Skip types that are table row types (they get spdb_register_schema instead)
            if table_type_refs.contains(&type_def.ty) {
                continue;
            }

            // Only register product types (structs) as GML structs
            let resolved = &typespace[type_def.ty];
            match resolved {
                AlgebraicTypeDef::Product(product_def) => {
                    generate_struct_entry(&mut output, &type_def.accessor_name, product_def, m);
                }
                AlgebraicTypeDef::Sum(_) => {
                    eprintln!(
                        "Note: Skipping sum type '{}' (not representable as GML struct)",
                        type_def.accessor_name
                    );
                }
                AlgebraicTypeDef::PlainEnum(_) => {
                    eprintln!(
                        "Note: Skipping plain enum '{}' (not representable as GML struct)",
                        type_def.accessor_name
                    );
                }
            }
        }
    });

    output
}

/// Generate a single `spdb_register_struct` call.
fn generate_struct_entry(
    output: &mut String,
    name: &spacetimedb_schema::def::ScopedTypeName,
    product_def: &ProductTypeDef,
    module: &ModuleDef,
) {
    writeln!(output, "spdb_register_struct(global.stdb_connection, \"{}\", [", name.name()).unwrap();
    for (i, (field_name, field_type)) in product_def.elements.iter().enumerate() {
        let gml_type = resolve_type(field_type, module);
        let comma = if i < product_def.elements.len() - 1 { "," } else { "" };
        writeln!(output, "    {{ name: \"{}\", type: \"{}\" }}{}", field_name, gml_type, comma).unwrap();
    }
    writeln!(output, "]);").unwrap();
    writeln!(output).unwrap();
}

/// Resolve wire primary-key field name for `spdb_register_schema` (matches decoded row keys).
fn table_primary_key_name(table: &TableDef) -> String {
    table
        .primary_key
        .and_then(|col_id| table.get_column(col_id))
        .map(|col| col.name.to_string())
        .unwrap_or_else(|| "id".to_string())
}

/// Generate `spdb_register_schema` calls for all public tables (including submodules).
fn generate_tables(module: &ModuleDef) -> String {
    let mut output = String::new();

    // Sort by wire name for deterministic output
    let mut tables: Vec<(NamespacePath, &ModuleDef, &TableDef)> = module
        .all_tables_with_prefix()
        .into_iter()
        .filter(|(_, _, t)| t.table_access == TableAccess::Public)
        .collect();
    tables.sort_by(|(path_a, _, table_a), (path_b, _, table_b)| {
        table_wire_name(path_a, table_a).cmp(&table_wire_name(path_b, table_b))
    });

    for (path, owning_def, table) in tables {
        let typespace = owning_def.typespace_for_generate();
        let product_def = &typespace[table.product_type_ref]
            .as_product()
            .expect("Table product type ref should resolve to a ProductTypeDef");

        let wire_name = table_wire_name(&path, table);
        let pk_name = table_primary_key_name(table);
        writeln!(
            output,
            "spdb_register_schema(global.stdb_connection, \"{}\", [",
            wire_name
        )
        .unwrap();

        for (i, (field_name, field_type)) in product_def.elements.iter().enumerate() {
            let gml_type = resolve_type(field_type, owning_def);
            let comma = if i < product_def.elements.len() - 1 {
                ","
            } else {
                ""
            };
            writeln!(
                output,
                "    {{ name: \"{}\", type: \"{}\" }}{}",
                field_name, gml_type, comma
            )
            .unwrap();
        }

        writeln!(output, "], \"{}\");", pk_name).unwrap();
        writeln!(output).unwrap();
    }

    output
}

/// Generate `spdb_register_reducer` calls for all client-callable reducers,
/// plus `spdb_register_reducer_error_schema` calls for reducers with error types.
fn generate_reducers(module: &ModuleDef) -> String {
    let mut output = String::new();

    // Include submodule reducers (wire names are already qualified, e.g. auth.verify).
    for (_path, owning_def, reducer) in module.all_reducers_with_prefix() {
        // Skip lifecycle reducers (init, connect, disconnect)
        if reducer.lifecycle.is_some() {
            continue;
        }
        // Skip private reducers
        if reducer.visibility.is_private() {
            continue;
        }

        generate_reducer_entry(&mut output, reducer, owning_def);
    }

    output
}

/// Generate a single `spdb_register_reducer` call, and if the reducer has an
/// error return type, also generate a `spdb_register_reducer_error_schema` call.
fn generate_reducer_entry(output: &mut String, reducer: &ReducerDef, module: &ModuleDef) {
    let params = &reducer.params_for_generate;

    if params.elements.is_empty() {
        writeln!(output, "spdb_register_reducer(\"{}\", []);", reducer.name).unwrap();
    } else {
        writeln!(output, "spdb_register_reducer(\"{}\", [", reducer.name).unwrap();
        for (i, (field_name, field_type)) in params.elements.iter().enumerate() {
            let gml_type = resolve_type(field_type, module);
            let comma = if i < params.elements.len() - 1 { "," } else { "" };
            writeln!(output, "    {{ name: \"{}\", type: \"{}\" }}{}", field_name, gml_type, comma).unwrap();
        }
        writeln!(output, "]);").unwrap();
    }

    // Generate error schema registration if the reducer has a non-unit error type
    if let Some(error_schema) = generate_error_schema(reducer, module) {
        // Escape double quotes for GML string literal (GML doesn't support single-quoted strings)
        let escaped_schema = error_schema.replace('"', "\\\"");
        writeln!(
            output,
            "spdb_register_reducer_error_schema(global.stdb_connection, \"{}\", \"{}\");",
            reducer.name, escaped_schema
        ).unwrap();
    }

    writeln!(output).unwrap();
}

/// Generate the error schema JSON string for a reducer's error return type.
/// Returns None if the error type is unit (no meaningful error data).
fn generate_error_schema(reducer: &ReducerDef, module: &ModuleDef) -> Option<String> {
    use spacetimedb_lib::sats::algebraic_type::AlgebraicType;

    match &reducer.err_return_type {
        AlgebraicType::String => Some("\"string\"".to_string()),
        AlgebraicType::Ref(r) => {
            // Look up the type definition to get its name
            match module.type_def_from_ref(*r) {
                Some((name, def)) => {
                    let typespace = module.typespace_for_generate();
                    match &typespace[def.ty] {
                        AlgebraicTypeDef::Sum(sum_def) => {
                            // Enum type: generate variant list
                            let variants: Vec<String> = sum_def.variants.iter().map(|(vname, vtype)| {
                                let type_str = if matches!(vtype, AlgebraicTypeUse::Unit) {
                                    "Unit".to_string()
                                } else {
                                    resolve_type(vtype, module)
                                };
                                format!("{{\"name\":\"{}\",\"type\":\"{}\"}}", vname, type_str)
                            }).collect();
                            Some(format!("[{}]", variants.join(",")))
                        }
                        AlgebraicTypeDef::PlainEnum(enum_def) => {
                            // Plain enum (all unit variants)
                            let variants: Vec<String> = enum_def.variants.iter().map(|vname| {
                                format!("{{\"name\":\"{}\",\"type\":\"Unit\"}}", vname)
                            }).collect();
                            Some(format!("[{}]", variants.join(",")))
                        }
                        AlgebraicTypeDef::Product(_) => {
                            // Struct type: reference by name
                            Some(format!("{{\"type\":\"{}\"}}", name.name()))
                        }
                    }
                }
                None => {
                    eprintln!("Warning: Could not resolve error type ref {:?} for reducer {}", r, reducer.name);
                    None
                }
            }
        }
        AlgebraicType::Sum(sum) => {
            // Anonymous sum/enum — generate inline variant list
            let variants: Vec<String> = sum.variants.iter().map(|variant| {
                let vname = variant.name.as_deref().unwrap_or("Unknown");
                let vtype_str = algebraic_type_to_schema_str(&variant.algebraic_type, module);
                format!("{{\"name\":\"{}\",\"type\":\"{}\"}}", vname, vtype_str)
            }).collect();
            Some(format!("[{}]", variants.join(",")))
        }
        _ => {
            // Unusual error types (primitives, arrays, etc.), skip error schema
            eprintln!("Note: Reducer '{}' has unusual error type, skipping error schema", reducer.name);
            None
        }
    }
}

/// Convert an AlgebraicType to a schema type string for error schema JSON.
fn algebraic_type_to_schema_str(ty: &spacetimedb_lib::sats::AlgebraicType, module: &ModuleDef) -> String {
    use spacetimedb_lib::sats::algebraic_type::AlgebraicType;

    match ty {
        AlgebraicType::String => "String".to_string(),
        AlgebraicType::Ref(r) => {
            match module.type_def_from_ref(*r) {
                Some((name, _)) => name.name().to_string(),
                None => "string".to_string(),
            }
        }
        AlgebraicType::Sum(_) => "string".to_string(), // nested sum, fallback
        AlgebraicType::Product(_) => "string".to_string(), // anonymous product, fallback
        _ => "string".to_string(),
    }
}

/// Generate global `REDUCER_xxx()` helper functions for all client-callable reducers.
///
/// Each function takes `_connection` as the first argument followed by typed
/// parameter names like `_room_id_u64`, and calls `spdb_call_reducer` internally.
fn generate_reducer_helpers(module: &ModuleDef) -> String {
    let mut output = String::new();

    for (_path, owning_def, reducer) in module.all_reducers_with_prefix() {
        // Skip lifecycle reducers (init, connect, disconnect)
        if reducer.lifecycle.is_some() {
            continue;
        }
        // Skip private reducers
        if reducer.visibility.is_private() {
            continue;
        }

        generate_reducer_helper_entry(&mut output, reducer, owning_def);
    }

    output
}

/// Generate a single `REDUCER_xxx()` helper function.
fn generate_reducer_helper_entry(output: &mut String, reducer: &ReducerDef, module: &ModuleDef) {
    let params = &reducer.params_for_generate;
    let wire_name = reducer.name.to_string();
    let func_name = format!("REDUCER_{}", sanitize_name_for_ident(&wire_name));

    // Build the parameter list: always starts with _connection, ends with _callback = undefined
    let mut param_list = vec!["_connection".to_string()];
    // Build the args struct fields: { field_name: _field_name_type, ... }
    let mut args_fields = Vec::new();

    for (field_name, field_type) in &params.elements {
        let gml_type = resolve_type(field_type, module);
        let type_suffix = sanitize_type_for_ident(&gml_type);
        let param_name = format!("_{}_{}", field_name, type_suffix);
        param_list.push(param_name.clone());
        args_fields.push(format!("{}: {}", field_name, param_name));
    }

    // Always append _callback = undefined at the end
    param_list.push("_callback = undefined".to_string());

    let params_str = param_list.join(", ");

    if args_fields.is_empty() {
        writeln!(
            output,
            "function {}({}) {{",
            func_name, params_str
        )
        .unwrap();
        writeln!(
            output,
            "    spdb_call_reducer(_connection, \"{}\", {{}}, _callback);",
            wire_name
        )
        .unwrap();
    } else {
        let args_struct = args_fields.join(", ");
        writeln!(
            output,
            "function {}({}) {{",
            func_name, params_str
        )
        .unwrap();
        writeln!(
            output,
            "    spdb_call_reducer(_connection, \"{}\", {{ {} }}, _callback);",
            wire_name, args_struct
        )
        .unwrap();
    }

    writeln!(output, "}}").unwrap();
    writeln!(output).unwrap();
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() -> Result<()> {
    let cli = Cli::parse();

    let module = match (cli.wasm.as_ref(), cli.json.as_ref()) {
        (Some(wasm_path), None) => extract_module_def_from_wasm(wasm_path, &cli.standalone)?,
        (None, Some(json_path)) => extract_module_def_from_json(json_path)?,
        _ => anyhow::bail!("Exactly one of --wasm or --json must be specified"),
    };

    let gml = generate_gml(&module);

    // Ensure output directory exists
    if let Some(parent) = cli.out.parent() {
        if !parent.as_os_str().is_empty() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
        }
    }

    fs::write(&cli.out, &gml).with_context(|| format!("Failed to write output file: {}", cli.out.display()))?;

    eprintln!("Successfully generated GML bindings to {}", cli.out.display());

    // Print summary
    let mut struct_count = 0usize;
    let mut table_type_refs: HashSet<AlgebraicTypeRef> = HashSet::new();
    visit_modules(&module, &mut |m| {
        for t in m.tables() {
            table_type_refs.insert(t.product_type_ref);
        }
    });
    visit_modules(&module, &mut |m| {
        let typespace = m.typespace_for_generate();
        struct_count += m
            .types()
            .filter(|td| !table_type_refs.contains(&td.ty))
            .filter(|td| matches!(&typespace[td.ty], AlgebraicTypeDef::Product(_)))
            .count();
    });
    let table_count = module
        .all_tables_with_prefix()
        .into_iter()
        .filter(|(_, _, t)| t.table_access == TableAccess::Public)
        .count();
    let reducer_count = module
        .all_reducers_with_prefix()
        .into_iter()
        .filter(|(_, _, r)| r.lifecycle.is_none() && !r.visibility.is_private())
        .count();

    eprintln!("  Structs:  {}", struct_count);
    eprintln!("  Tables:   {}", table_count);
    eprintln!("  Reducers: {}", reducer_count);

    Ok(())
}