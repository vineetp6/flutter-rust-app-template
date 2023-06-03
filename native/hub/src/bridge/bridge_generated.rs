#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case,
    clippy::too_many_arguments
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`@ 1.77.1.

use crate::bridge::api::*;
use core::panic::UnwindSafe;
use flutter_rust_bridge::*;
use std::ffi::c_void;
use std::sync::Arc;

// Section: imports

// Section: wire functions

fn wire_prepare_viewmodel_update_stream_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "prepare_viewmodel_update_stream",
            port: Some(port_),
            mode: FfiCallMode::Stream,
        },
        move || {
            move |task_callback| Ok(prepare_viewmodel_update_stream(task_callback.stream_sink()))
        },
    )
}
fn wire_prepare_channels_impl() -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "prepare_channels",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || Ok(prepare_channels()),
    )
}
fn wire_start_rust_logic_impl(port_: MessagePort) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "start_rust_logic",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(start_rust_logic()),
    )
}
fn wire_send_user_action_impl(
    task_address: impl Wire2Api<String> + UnwindSafe,
    serialized: impl Wire2Api<Serialized> + UnwindSafe,
) -> support::WireSyncReturn {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap_sync(
        WrapInfo {
            debug_name: "send_user_action",
            port: None,
            mode: FfiCallMode::Sync,
        },
        move || {
            let api_task_address = task_address.wire2api();
            let api_serialized = serialized.wire2api();
            Ok(send_user_action(api_task_address, api_serialized))
        },
    )
}
// Section: wrapper structs

// Section: static checks

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        (!self.is_null()).then(|| self.wire2api())
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

// Section: impl IntoDart

impl support::IntoDart for Serialized {
    fn into_dart(self) -> support::DartAbi {
        vec![self.bytes.into_dart(), self.formula.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for Serialized {}

impl support::IntoDart for ViewmodelUpdate {
    fn into_dart(self) -> support::DartAbi {
        vec![self.item_address.into_dart(), self.serialized.into_dart()].into_dart()
    }
}
impl support::IntoDartExceptPrimitive for ViewmodelUpdate {}

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

/// cbindgen:ignore
#[cfg(target_family = "wasm")]
#[path = "bridge_generated.web.rs"]
mod web;
#[cfg(target_family = "wasm")]
pub use web::*;

#[cfg(not(target_family = "wasm"))]
#[path = "bridge_generated.io.rs"]
mod io;
#[cfg(not(target_family = "wasm"))]
pub use io::*;
