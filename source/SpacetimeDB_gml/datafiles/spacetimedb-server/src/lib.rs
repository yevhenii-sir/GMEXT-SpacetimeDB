//! SpacetimeDB server module for GameMaker extension testing.
//!
//! Deploy:
//!   spacetime publish spdb-gmext-test-cifyi -s maincloud -p . --yes
//!
//! Tables:
//!   - player: { id, name, hp, x, y }
//!   - chat_message: { id, author, text }
//!
//! Reducers (GML keyboard demo):
//!   - spawn_player(name)           — Up
//!   - delete_player(id)            — Down
//!   - move_player(id, dx, dy)      — Left / Right / PageUp / PageDown
//!   - damage_player(id, amount)    — Space
//!   - heal_player(id, amount)      — H
//!   - rename_player(id, name)      — R
//!   - say(author, text)            — Enter
//!   - clear_players()              — Delete

use log::info;
use spacetimedb::{reducer, table, ReducerContext, Table};

// ---------------------------------------------------------------------------
// Tables
// ---------------------------------------------------------------------------

#[table(accessor = player, public)]
pub struct Player {
    #[primary_key]
    #[auto_inc]
    id: u64,
    name: String,
    hp: u32,
    x: i32,
    y: i32,
}

#[table(accessor = chat_message, public)]
pub struct ChatMessage {
    #[primary_key]
    #[auto_inc]
    id: u64,
    author: String,
    text: String,
}

// ---------------------------------------------------------------------------
// Reducers — players
// ---------------------------------------------------------------------------

/// Insert a new player with 100 HP at the origin.
#[reducer]
pub fn spawn_player(ctx: &ReducerContext, name: String) {
    info!("spawn_player: name={}", name);
    ctx.db.player().insert(Player {
        id: 0,
        name,
        hp: 100,
        x: 0,
        y: 0,
    });
}

/// Delete a player by ID.
#[reducer]
pub fn delete_player(ctx: &ReducerContext, id: u64) {
    let Some(row) = ctx.db.player().id().find(id) else {
        panic!("delete_player: id={id} not found");
    };
    ctx.db.player().delete(row);
    info!("delete_player: id={id} deleted");
}

/// Move a player by (dx, dy).
#[reducer]
pub fn move_player(ctx: &ReducerContext, id: u64, dx: i32, dy: i32) {
    let Some(mut row) = ctx.db.player().id().find(id) else {
        panic!("move_player: id={id} not found");
    };
    row.x = row.x.saturating_add(dx);
    row.y = row.y.saturating_add(dy);
    info!("move_player: id={id} -> ({}, {})", row.x, row.y);
    ctx.db.player().id().update(row);
}

/// Deal damage; clamps HP at 0 (does not auto-delete).
#[reducer]
pub fn damage_player(ctx: &ReducerContext, id: u64, amount: u32) {
    let Some(mut row) = ctx.db.player().id().find(id) else {
        panic!("damage_player: id={id} not found");
    };
    row.hp = row.hp.saturating_sub(amount);
    info!("damage_player: id={id} hp={}", row.hp);
    ctx.db.player().id().update(row);
}

/// Heal a player; clamps HP at 100.
#[reducer]
pub fn heal_player(ctx: &ReducerContext, id: u64, amount: u32) {
    let Some(mut row) = ctx.db.player().id().find(id) else {
        panic!("heal_player: id={id} not found");
    };
    row.hp = (row.hp.saturating_add(amount)).min(100);
    info!("heal_player: id={id} hp={}", row.hp);
    ctx.db.player().id().update(row);
}

/// Rename a player.
#[reducer]
pub fn rename_player(ctx: &ReducerContext, id: u64, name: String) {
    let Some(mut row) = ctx.db.player().id().find(id) else {
        panic!("rename_player: id={id} not found");
    };
    info!("rename_player: id={id} -> {name}");
    row.name = name;
    ctx.db.player().id().update(row);
}

/// Delete every player row (demo reset).
#[reducer]
pub fn clear_players(ctx: &ReducerContext) {
    let ids: Vec<u64> = ctx.db.player().iter().map(|p| p.id).collect();
    for id in ids {
        if let Some(row) = ctx.db.player().id().find(id) {
            ctx.db.player().delete(row);
        }
    }
    info!("clear_players: done");
}

// ---------------------------------------------------------------------------
// Reducers — chat
// ---------------------------------------------------------------------------

/// Append a chat line (visible to all subscribers of chat_message).
#[reducer]
pub fn say(ctx: &ReducerContext, author: String, text: String) {
    info!("say: {author}: {text}");
    ctx.db.chat_message().insert(ChatMessage {
        id: 0,
        author,
        text,
    });
}

// ---------------------------------------------------------------------------
// Lifecycle
// ---------------------------------------------------------------------------

#[reducer(init)]
pub fn init(_ctx: &ReducerContext) {
    info!("SpacetimeDB GameMaker example module initialized");
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    info!("Client connected: {:?}", ctx.sender());
}

#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    info!("Client disconnected: {:?}", ctx.sender());
}
