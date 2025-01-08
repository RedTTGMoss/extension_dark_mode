use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct File {
    pub key: String,
    pub path: String,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ExtensionInfo {
    pub files: Vec<File>,
}

#[derive(ToBytes, Serialize, PartialEq, Debug, Clone)]
#[encoding(Json)]
pub struct ContextButton {
    pub text: String,
    pub icon: String,
    pub context_icon: String,
    pub action: String,
    pub context_menu: String,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ContextMenu {
    pub key: String,
    pub buttons: Vec<ContextButton>,
}

#[derive(ToBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct MossState {
    pub width: i32,
    pub height: i32,
    pub current_screen: String,
    pub opened_context_menus: Vec<String>,
    pub icons: Vec<String>,
}

#[derive(FromBytes, Deserialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ConfigGet<T> {
    pub value: T,
}

#[derive(ToBytes, Serialize, PartialEq, Debug)]
#[encoding(Json)]
pub struct ConfigSet<T> {
    pub key: String,
    pub value: T,
}

#[host_fn]
extern "ExtismHost" {
    pub fn moss_gui_register_context_menu(menu: ContextMenu);
    pub fn moss_em_config_get<T: for<'de> Deserialize<'de>>(key: &str) -> ConfigGet<T>;
    pub fn moss_gui_invert_icon(key: String, result_key: String);
    #[link_name = "moss_em_config_set"]
    fn _moss_em_config_set<T: Serialize>(value: ConfigSet<T>);
}

pub unsafe fn moss_em_config_set<T: Serialize>(key: &str, value: T) {
    let _ = _moss_em_config_set::<T>(ConfigSet::<T> {
        key: key.into(),
        value,
    });
}

#[link(wasm_import_module = "extism:host/user")]
extern "C" {
    #[link_name = "moss_gui_open_context_menu"]
    fn moss_gui_open_context_menu_impl(key: u64, x: i64, y: i64) -> ();
    #[link_name = "moss_defaults_set_color"]
    fn moss_defaults_set_color_impl(key: u64, r: i64, g: i64, b: i64, a: i64) -> ();
    #[link_name = "moss_defaults_set_text_color"]
    fn moss_defaults_set_text_color_impl(key: u64, r1: i64, g1: i64, b1: i64, r2: i64, g2: i64, b2: i64) -> ();
}

pub unsafe fn moss_gui_open_context_menu(key: &str, x: i64, y: i64)
    -> Result<(), extism_pdk::Error> {
    let res =
        moss_gui_open_context_menu_impl(extism_pdk::ToMemory::to_memory(&&key)?.offset(), x, y);
    Ok(res)
}
pub unsafe fn moss_defaults_set_color(key: &str, r: i64, g: i64, b: i64, a: i64)
    -> Result<(), extism_pdk::Error> {
    let res =
        moss_defaults_set_color_impl(extism_pdk::ToMemory::to_memory(&&key)?.offset(), r, g, b, a);
    Ok(res)
}

pub unsafe fn moss_defaults_set_text_color(key: &str, r1: i64, g1: i64, b1: i64, r2: i64, g2: i64, b2: i64)
    -> Result<(), extism_pdk::Error> {
    let res =
        moss_defaults_set_text_color_impl(extism_pdk::ToMemory::to_memory(&&key)?.offset(), r1, g1, b1, r2, g2, b2);
    Ok(res)
}

// #[link(wasm_import_module = "extism:host/user")]
// extern "C" {
//     #[link_name = "moss_em_config_get"]
//     fn moss_em_config_get_impl(key: u64) -> u64;
// }
// pub unsafe fn moss_em_config_get<T: extism_pdk::FromBytesOwned + for<'de> Deserialize<'de>>(
//     key: &str,
// ) -> Result<Return<T>, extism_pdk::Error> {
//     let res = extism_pdk::Memory::from(moss_em_config_get_impl(
//         extism_pdk::ToMemory::to_memory(&&key)?.offset(),
//     ));
//     let return_value: Result<Return<T>, extism_pdk::Error> =
//         extism_pdk::FromBytes::from_bytes(&res.to_vec());
//     return_value
// }
