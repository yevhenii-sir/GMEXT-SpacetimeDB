//! SpacetimeDB server module for GameMaker extension testing.
//!
//! Deploy:
//!   spacetime publish spdb-gmext-test-cifyi -s maincloud -p . --yes
//!
//! Tables:
//!   - player: { id, name, hp, x, y, vx, vy }
//!   - chat_message: { id, author, text }
//!   - move_tick_timer: scheduled interval (~50ms) drives position from velocity
//!
//! Movement model (realtime-style):
//!   Client holds WASD → calls `set_player_velocity` only when the input vector **changes**
//!   (including a final `0,0` on release). Server tick integrates `x += vx`, `y += vy`.
//!
//! Other reducers (GML keyboard demo):
//!   - spawn_player(name)           — 1
//!   - delete_player(id)            — 2
//!   - set_player_velocity(id, vx, vy) — WASD hold (edge-triggered)
//!   - damage_player(id, amount)    — Space
//!   - heal_player(id, amount)      — H
//!   - rename_player(id, name)      — R
//!   - say(author, text)            — Enter
//!   - clear_players()              — Delete

use log::info;
use spacetimedb::{reducer, table, ReducerContext, ScheduleAt, Table};
use std::time::Duration;

/// Units of (x,y) applied per server move tick while velocity is non-zero.
const MOVE_SPEED: i32 = 4;
/// How often the server integrates positions from velocity.
const MOVE_TICK_MS: u64 = 50;

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
    /// Velocity in world units per move tick (set by client input; applied in `move_tick`).
    vx: i32,
    vy: i32,
}

#[table(accessor = chat_message, public)]
pub struct ChatMessage {
    #[primary_key]
    #[auto_inc]
    id: u64,
    author: String,
    text: String,
}

/// Interval schedule that runs [`move_tick`].
#[table(accessor = move_tick_timer, scheduled(move_tick))]
pub struct MoveTickTimer {
    #[primary_key]
    #[auto_inc]
    scheduled_id: u64,
    scheduled_at: ScheduleAt,
}

// ---------------------------------------------------------------------------
// Reducers — players
// ---------------------------------------------------------------------------

/// Insert a new player with 100 HP at the origin (standing still).
#[reducer]
pub fn spawn_player(ctx: &ReducerContext, name: String) {
    info!("spawn_player: name={}", name);
    ctx.db.player().insert(Player {
        id: 0,
        name,
        hp: 100,
        x: 0,
        y: 0,
        vx: 0,
        vy: 0,
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

/// Set movement intent. Client should call this only when WASD vector changes
/// (press, direction change, or release → `0,0`), not every frame.
#[reducer]
pub fn set_player_velocity(ctx: &ReducerContext, id: u64, vx: i32, vy: i32) {
    let Some(mut row) = ctx.db.player().id().find(id) else {
        panic!("set_player_velocity: id={id} not found");
    };
    let nx = vx.clamp(-MOVE_SPEED, MOVE_SPEED);
    let ny = vy.clamp(-MOVE_SPEED, MOVE_SPEED);
    if row.vx == nx && row.vy == ny {
        return;
    }
    row.vx = nx;
    row.vy = ny;
    info!("set_player_velocity: id={id} v=({nx},{ny})");
    ctx.db.player().id().update(row);
}

/// Scheduled: integrate position from velocity for moving players only.
#[reducer]
pub fn move_tick(ctx: &ReducerContext, _timer: MoveTickTimer) {
    let movers: Vec<Player> = ctx
        .db
        .player()
        .iter()
        .filter(|p| p.vx != 0 || p.vy != 0)
        .collect();
    for mut row in movers {
        row.x = row.x.saturating_add(row.vx);
        row.y = row.y.saturating_add(row.vy);
        ctx.db.player().id().update(row);
    }
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
pub fn init(ctx: &ReducerContext) {
    info!("SpacetimeDB GameMaker example module initialized");
    if ctx.db.move_tick_timer().count() == 0 {
        ctx.db.move_tick_timer().insert(MoveTickTimer {
            scheduled_id: 0,
            scheduled_at: ScheduleAt::Interval(Duration::from_millis(MOVE_TICK_MS).into()),
        });
        info!("move_tick scheduled every {MOVE_TICK_MS}ms");
    }
}

#[reducer(client_connected)]
pub fn client_connected(ctx: &ReducerContext) {
    info!("Client connected: {:?}", ctx.sender());
}

#[reducer(client_disconnected)]
pub fn client_disconnected(ctx: &ReducerContext) {
    info!("Client disconnected: {:?}", ctx.sender());
}
