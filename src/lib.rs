mod moss_definitions;
pub use moss_definitions::*;

use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[plugin_fn]
pub unsafe fn register(Json(state): Json<MossState>) -> FnResult<ExtensionInfo> {
    let bg_tone = 10;
    let selected_tone = 230;
    let line_gray = 255 - 88;
    let line_gray_light = 255 - 167;
    let document_gray = 255 - 184;
    let document_background = 50;
    let subtitle_gray = 255 - 100;
    moss_defaults_set_color("BACKGROUND", bg_tone, bg_tone, bg_tone, 255)?;
    moss_defaults_set_color("SELECTED", selected_tone, selected_tone, selected_tone, 255)?;
    moss_defaults_set_color("LINE_GRAY", line_gray, line_gray, line_gray, 255)?;
    moss_defaults_set_color(
        "LINE_GRAY_LIGHT",
        line_gray_light,
        line_gray_light,
        line_gray_light,
        255,
    )?;
    moss_defaults_set_color(
        "DOCUMENT_GRAY",
        document_gray,
        document_gray,
        document_gray,
        255,
    )?;
    moss_defaults_set_color(
        "DOCUMENT_BACKGROUND",
        document_background,
        document_background,
        document_background,
        230,
    )?;
    moss_defaults_set_color(
        "BUTTON_ACTIVE_COLOR",
        selected_tone,
        selected_tone,
        selected_tone,
        25,
    )?;
    moss_defaults_set_color(
        "BUTTON_ACTIVE_COLOR_INVERTED",
        bg_tone,
        bg_tone,
        bg_tone,
        50,
    )?;
    moss_defaults_set_color(
        "BUTTON_DISABLED_COLOR",
        selected_tone,
        selected_tone,
        selected_tone,
        100,
    )?;
    moss_defaults_set_color(
        "BUTTON_DISABLED_LIGHT_COLOR",
        bg_tone,
        bg_tone,
        bg_tone,
        100,
    )?;
    moss_defaults_set_color(
        "OUTLINE_COLOR",
        selected_tone,
        selected_tone,
        selected_tone,
        255,
    )?;
    moss_defaults_set_text_color("TEXT_COLOR", 255, 255, 255, bg_tone, bg_tone, bg_tone)?;
    moss_defaults_set_text_color(
        "DOCUMENT_TITLE_COLOR",
        selected_tone,
        selected_tone,
        selected_tone,
        bg_tone,
        bg_tone,
        bg_tone,
    )?;
    moss_defaults_set_text_color(
        "DOCUMENT_TITLE_COLOR_INVERTED",
        bg_tone,
        bg_tone,
        bg_tone,
        selected_tone,
        selected_tone,
        selected_tone,
    )?;
    moss_defaults_set_text_color(
        "DOCUMENT_SUBTITLE_COLOR",
        subtitle_gray,
        subtitle_gray,
        subtitle_gray,
        bg_tone,
        bg_tone,
        bg_tone,
    )?;
    moss_defaults_set_text_color("TEXT_COLOR_T", 255, 255, 255, -1, -1, -1)?;
    moss_defaults_set_text_color("TEXT_COLOR_H", bg_tone, bg_tone, bg_tone, -1, -1, -1)?;
    let mut to_invert = vec![];
    for icon in state.icons {
        if icon.ends_with("_inverted") {
            to_invert.push(icon.clone());
            if let Some(non_inverted) = icon.strip_suffix("_inverted") {
                to_invert.push(non_inverted.to_string());
            }
        }
    }
    for icon in to_invert {
        moss_gui_invert_icon(icon.clone(), icon.clone())?;
    }

    Ok(ExtensionInfo { files: [].to_vec() })
}

#[plugin_fn]
pub unsafe fn extension_loop(Json(state): Json<MossState>) -> FnResult<()> {
    Ok(())
}

#[plugin_fn]
pub fn unregister() -> FnResult<()> {
    Ok(())
}
