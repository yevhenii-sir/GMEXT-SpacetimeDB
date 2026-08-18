///@desc Auto-generated SpacetimeDB schema registrations

function generated_shemas_spacetimedb() {


    // ==========================================
    // 2. TABLES
    // ==========================================

    spdb_register_schema(global.stdb_connection, "chat_message", [
        { name: "id", type: "u64" },
        { name: "author", type: "string" },
        { name: "text", type: "string" }
    ], "id");
    
    spdb_register_schema(global.stdb_connection, "player", [
        { name: "id", type: "u64" },
        { name: "name", type: "string" },
        { name: "hp", type: "u32" },
        { name: "x", type: "i32" },
        { name: "y", type: "i32" }
    ], "id");
    

    // ==========================================
    // 3. REDUCERS (registration)
    // ==========================================

    spdb_register_reducer("clear_players", []);
    spdb_register_reducer_error_schema(global.stdb_connection, "clear_players", "\"string\"");
    
    spdb_register_reducer("damage_player", [
        { name: "id", type: "u64" },
        { name: "amount", type: "u32" }
    ]);
    spdb_register_reducer_error_schema(global.stdb_connection, "damage_player", "\"string\"");
    
    spdb_register_reducer("delete_player", [
        { name: "id", type: "u64" }
    ]);
    spdb_register_reducer_error_schema(global.stdb_connection, "delete_player", "\"string\"");
    
    spdb_register_reducer("heal_player", [
        { name: "id", type: "u64" },
        { name: "amount", type: "u32" }
    ]);
    spdb_register_reducer_error_schema(global.stdb_connection, "heal_player", "\"string\"");
    
    spdb_register_reducer("move_player", [
        { name: "id", type: "u64" },
        { name: "dx", type: "i32" },
        { name: "dy", type: "i32" }
    ]);
    spdb_register_reducer_error_schema(global.stdb_connection, "move_player", "\"string\"");
    
    spdb_register_reducer("rename_player", [
        { name: "id", type: "u64" },
        { name: "name", type: "string" }
    ]);
    spdb_register_reducer_error_schema(global.stdb_connection, "rename_player", "\"string\"");
    
    spdb_register_reducer("say", [
        { name: "author", type: "string" },
        { name: "text", type: "string" }
    ]);
    spdb_register_reducer_error_schema(global.stdb_connection, "say", "\"string\"");
    
    spdb_register_reducer("spawn_player", [
        { name: "name", type: "string" }
    ]);
    spdb_register_reducer_error_schema(global.stdb_connection, "spawn_player", "\"string\"");
    
}

// ==========================================
// 4. REDUCER HELPER FUNCTIONS (global)
// ==========================================

function REDUCER_clear_players(_connection, _callback = undefined) {
    spdb_call_reducer(_connection, "clear_players", {}, _callback);
}

function REDUCER_damage_player(_connection, _id_u64, _amount_u32, _callback = undefined) {
    spdb_call_reducer(_connection, "damage_player", { id: _id_u64, amount: _amount_u32 }, _callback);
}

function REDUCER_delete_player(_connection, _id_u64, _callback = undefined) {
    spdb_call_reducer(_connection, "delete_player", { id: _id_u64 }, _callback);
}

function REDUCER_heal_player(_connection, _id_u64, _amount_u32, _callback = undefined) {
    spdb_call_reducer(_connection, "heal_player", { id: _id_u64, amount: _amount_u32 }, _callback);
}

function REDUCER_move_player(_connection, _id_u64, _dx_i32, _dy_i32, _callback = undefined) {
    spdb_call_reducer(_connection, "move_player", { id: _id_u64, dx: _dx_i32, dy: _dy_i32 }, _callback);
}

function REDUCER_rename_player(_connection, _id_u64, _name_string, _callback = undefined) {
    spdb_call_reducer(_connection, "rename_player", { id: _id_u64, name: _name_string }, _callback);
}

function REDUCER_say(_connection, _author_string, _text_string, _callback = undefined) {
    spdb_call_reducer(_connection, "say", { author: _author_string, text: _text_string }, _callback);
}

function REDUCER_spawn_player(_connection, _name_string, _callback = undefined) {
    spdb_call_reducer(_connection, "spawn_player", { name: _name_string }, _callback);
}

