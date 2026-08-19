/// @description Poll SpacetimeDB + keyboard reducer demo
/// Controls: 1 spawn | 2 delete | WASD hold-move | Space dmg | H heal | R rename | Enter chat | Del clear

spdb_poll(global.spdb_connection);

// 1 — spawn
if (keyboard_check_pressed(ord("1"))) {
    var _name = "Player_" + string(irandom(9999));
    spdb_call_reducer(global.spdb_connection, "spawn_player", { name: _name }, {
        on_result: function(ev) { show_debug_message("spawn_player: " + json_stringify(ev)); },
        on_error: function(ev) { show_debug_message("spawn_player ERROR: " + json_stringify(ev)); }
    });
    show_debug_message("→ spawn_player name=" + _name);
}

// 2 — delete last
if (keyboard_check_pressed(ord("2"))) {
    var _id = __spdb_example_require_player();
    if (!is_undefined(_id)) {
        spdb_call_reducer(global.spdb_connection, "delete_player", { id: _id }, {
            on_result: function(ev) { show_debug_message("delete_player: " + json_stringify(ev)); },
            on_error: function(ev) { show_debug_message("delete_player ERROR: " + json_stringify(ev)); }
        });
        show_debug_message("→ delete_player id=" + string(int64(_id)));
    }
}

// WASD — hold to move; reducer only when velocity vector changes (incl. release stop)
__spdb_example_sync_move_velocity();

// Space — damage 15
if (keyboard_check_pressed(vk_space)) {
    var _id = __spdb_example_require_player();
    if (!is_undefined(_id)) {
        spdb_call_reducer(global.spdb_connection, "damage_player", { id: _id, amount: 15 }, {
            on_error: function(ev) { show_debug_message("damage ERROR: " + json_stringify(ev)); }
        });
        show_debug_message("→ damage_player 15");
    }
}

// H — heal 20
if (keyboard_check_pressed(ord("H"))) {
    var _id = __spdb_example_require_player();
    if (!is_undefined(_id)) {
        spdb_call_reducer(global.spdb_connection, "heal_player", { id: _id, amount: 20 }, undefined);
        show_debug_message("→ heal_player 20");
    }
}

// R — rename
if (keyboard_check_pressed(ord("R"))) {
    var _id = __spdb_example_require_player();
    if (!is_undefined(_id)) {
        var _name = "Hero_" + string(irandom(999));
        spdb_call_reducer(global.spdb_connection, "rename_player", { id: _id, name: _name }, undefined);
        show_debug_message("→ rename_player " + _name);
    }
}

// Enter — chat
if (keyboard_check_pressed(vk_enter)) {
    var _author = "GM";
    if (!is_undefined(global.spdb_last_player_id)) {
        var row = spdb_table_find(global.spdb_connection, "player", global.spdb_last_player_id);
        if (is_struct(row) && variable_struct_exists(row, "name")) {
            _author = row.name;
        }
    }
    var _lines = ["Hello from GameMaker!", "Need heals!", "Moving out.", "gg"];
    var _text = _lines[irandom(array_length(_lines) - 1)];
    spdb_call_reducer(global.spdb_connection, "say", { author: _author, text: _text }, undefined);
    show_debug_message("→ say " + _author + ": " + _text);
}

// Delete — clear all players
if (keyboard_check_pressed(vk_delete)) {
    spdb_call_reducer(global.spdb_connection, "clear_players", {}, {
        on_result: function(ev) {
            global.spdb_last_player_id = undefined;
            global.spdb_move_vx = 0;
            global.spdb_move_vy = 0;
            show_debug_message("clear_players done | cache=" + string(spdb_table_count(global.spdb_connection, "player")));
        }
    });
    show_debug_message("→ clear_players");
}
