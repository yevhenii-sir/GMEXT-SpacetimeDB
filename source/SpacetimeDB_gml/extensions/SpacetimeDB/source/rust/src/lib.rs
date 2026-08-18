// ##### extgen :: Auto-generated file - do not edit (regenerated) #####
// Extra `mod` lines: edit src/extra_mods.inc.rs (IfMissing).
// User API: edit src/user/ (IfMissing).

#![allow(non_snake_case)]

pub mod generated;
pub mod user;

include!("extra_mods.inc.rs");

pub use generated::ffi;
pub use generated::types;
pub use generated::codecs;

#[cfg(target_os = "android")]
#[allow(non_snake_case)]
#[path = "generated/android_jni.rs"]
mod android_jni;

