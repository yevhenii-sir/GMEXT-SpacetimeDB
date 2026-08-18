/// @description SpacetimeDB example — connects, registers schemas, subscribes
/// Row store of truth: native cache via spdb_table_iter / find / count.

global.spdb_connection = spdb_connect("wss://maincloud.spacetimedb.com", "spdb-gmext-test-cifyi", "");
global.spdb_last_player_id = undefined;

spdb_on(global.spdb_connection, "identity_token", function(data) {
    global.spdb_token = data.payload.token;
    show_debug_message("Identity token received and saved.");
});

// Schemas (primary_key defaults to "id")
spdb_register_schema(global.spdb_connection, "player", [
    { name: "id", type: "u64" },
    { name: "name", type: "string" },
    { name: "hp", type: "u32" },
    { name: "x", type: "i32" },
    { name: "y", type: "i32" }
], "id");

spdb_register_schema(global.spdb_connection, "chat_message", [
    { name: "id", type: "u64" },
    { name: "author", type: "string" },
    { name: "text", type: "string" }
], "id");

spdb_register_reducer("spawn_player", [{ name: "name", type: "string" }]);
spdb_register_reducer("delete_player", [{ name: "id", type: "u64" }]);
spdb_register_reducer("move_player", [
    { name: "id", type: "u64" },
    { name: "dx", type: "i32" },
    { name: "dy", type: "i32" }
]);
spdb_register_reducer("damage_player", [
    { name: "id", type: "u64" },
    { name: "amount", type: "u32" }
]);
spdb_register_reducer("heal_player", [
    { name: "id", type: "u64" },
    { name: "amount", type: "u32" }
]);
spdb_register_reducer("rename_player", [
    { name: "id", type: "u64" },
    { name: "name", type: "string" }
]);
spdb_register_reducer("clear_players", []);
spdb_register_reducer("say", [
    { name: "author", type: "string" },
    { name: "text", type: "string" }
]);

spdb_on(global.spdb_connection, "connected", function(data) {
    show_debug_message("Connected to SpacetimeDB!");
    show_debug_message("Controls: 1=spawn 2=delete WASD=move Space=dmg H=heal R=rename Enter=chat Del=clear");

    // bind_table: debug logs + last_player_id only — rows live in native cache
    spdb_bind_table(global.spdb_connection, "SELECT * FROM player", "player",
        function(rows) {
            show_debug_message("Initial players (event): " + string(array_length(rows))
                + " | cache count=" + string(spdb_table_count(global.spdb_connection, "player")));
            for (var i = 0; i < array_length(rows); i++) {
                var p = rows[i];
                show_debug_message($"  id={int64(p.id)} name={p.name} hp={p.hp} pos=({p.x},{p.y})");
                global.spdb_last_player_id = p.id;
            }
        },
        function(inserts, deletes) {
            var inserted_ids = {};
            for (var i = 0; i < array_length(inserts); i++) {
                var p = inserts[i];
                inserted_ids[$ string(int64(p.id))] = true;
                show_debug_message($"Player upsert: id={int64(p.id)} name={p.name} hp={p.hp} pos=({p.x},{p.y})");
                global.spdb_last_player_id = p.id;
            }
            for (var i = 0; i < array_length(deletes); i++) {
                var d = deletes[i];
                if (variable_struct_exists(inserted_ids, string(int64(d.id)))) continue;
                show_debug_message($"Player deleted: id={int64(d.id)}");
                if (!is_undefined(global.spdb_last_player_id) && int64(global.spdb_last_player_id) == int64(d.id)) {
                    var remaining = spdb_table_iter(global.spdb_connection, "player");
                    global.spdb_last_player_id = (array_length(remaining) > 0)
                        ? remaining[array_length(remaining) - 1].id
                        : undefined;
                }
            }
        }
    );

    spdb_bind_table(global.spdb_connection, "SELECT * FROM chat_message", "chat_message",
        function(rows) {
            show_debug_message("Chat history: " + string(array_length(rows)) + " messages");
        },
        function(inserts, deletes) {
            for (var i = 0; i < array_length(inserts); i++) {
                show_debug_message($"CHAT {inserts[i].author}: {inserts[i].text}");
            }
        }
    );
});

spdb_on(global.spdb_connection, "disconnected", function(data) {
    global.spdb_last_player_id = undefined;
    show_debug_message("Disconnected from SpacetimeDB.");
});

function __spdb_example_require_player() {
    if (is_undefined(global.spdb_last_player_id)) {
        // Fall back to any row in native cache
        var rows = spdb_table_iter(global.spdb_connection, "player");
        if (array_length(rows) > 0) {
            global.spdb_last_player_id = rows[array_length(rows) - 1].id;
            return global.spdb_last_player_id;
        }
        show_debug_message("No player selected — press 1 to spawn one.");
        return undefined;
    }
    return global.spdb_last_player_id;
}
