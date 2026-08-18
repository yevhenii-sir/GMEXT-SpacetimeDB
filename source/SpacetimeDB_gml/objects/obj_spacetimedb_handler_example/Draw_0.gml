/// @description Visual overlay — reads players from native cache

draw_set_color(c_white);
draw_set_halign(fa_left);
draw_set_valign(fa_top);
draw_text(16, 16, "SpacetimeDB demo → spdb-gmext-test-cifyi");
draw_text(16, 36, "1 spawn | 2 delete | WASD move | Space dmg | H heal | R rename | Enter chat | Del clear");

var connected = is_struct(global.spdb_connection) && global.spdb_connection.connected;
draw_text(16, 56, connected ? "Status: CONNECTED" : "Status: connecting…");

var players = connected ? spdb_table_iter(global.spdb_connection, "player") : [];
draw_text(16, 76, "Players (cache): " + string(array_length(players)));

var ox = 16;
var oy = 110;
for (var i = 0; i < array_length(players); i++) {
    var p = players[i];
    var selected = (!is_undefined(global.spdb_last_player_id)
        && int64(global.spdb_last_player_id) == int64(p.id));
    var px = ox + (real(p.x) * 2);
    var py = oy + (real(p.y) * 2);
    draw_set_color(selected ? c_lime : c_aqua);
    draw_rectangle(px, py, px + 48, py + 48, false);
    draw_set_color(c_black);
    draw_text(px + 4, py + 4, string_copy(p.name, 1, 8));
    draw_text(px + 4, py + 22, "hp " + string(p.hp));
}

draw_set_color(c_white);
