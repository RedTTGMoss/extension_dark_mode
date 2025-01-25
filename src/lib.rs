mod moss_definitions;
pub use moss_definitions::functions::*;
pub use moss_definitions::types::*;

use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[plugin_fn]
pub unsafe fn moss_extension_register(Json(state): Json<MossState>) -> FnResult<ExtensionInfo> {
    let white = Color::new_monochrome(255, None);
    let background = Color::new_monochrome(30, None);
    let selected = Color::new_monochrome(230, None);
    let line_gray = Color::new_monochrome(255 - 88, None);
    let line_gray_light = Color::new_monochrome(255 - 167, None);
    let document_gray = Color::new_monochrome(255 - 184, None);
    let document_background = Color::new_monochrome(50, Some(230));
    let subtitle_gray = Color::new_monochrome(155, None);

    let button_active_color = Color::from_existing(selected, Some(25));
    let button_active_color_inverted = Color::from_existing(background, Some(50));
    let button_disabled_color = Color::from_existing(selected, Some(100));
    let button_disabled_light_color = Color::from_existing(background, Some(100));

    // Base colors
    moss_defaults_set_color("BACKGROUND".to_string(), background)?;
    moss_defaults_set_color("SELECTED".to_string(), selected)?;
    moss_defaults_set_color("LINE_GRAY".to_string(), line_gray)?;
    moss_defaults_set_color("LINE_GRAY_LIGHT".to_string(), line_gray_light)?;
    moss_defaults_set_color("DOCUMENT_GRAY".to_string(), document_gray)?;
    moss_defaults_set_color("DOCUMENT_BACKGROUND".to_string(), document_background)?;
    moss_defaults_set_color("BUTTON_ACTIVE_COLOR".to_string(), button_active_color)?;
    moss_defaults_set_color("BUTTON_ACTIVE_COLOR_INVERTED".to_string(), button_active_color_inverted)?;
    moss_defaults_set_color("BUTTON_DISABLED_COLOR".to_string(), button_disabled_color)?;
    moss_defaults_set_color("BUTTON_DISABLED_LIGHT_COLOR".to_string(), button_disabled_light_color)?;
    moss_defaults_set_color("OUTLINE_COLOR".to_string(), selected)?;

    // Text colors
    moss_defaults_set_text_color("TEXT_COLOR".to_string(), TextColors{ foreground: white, background: Some(background)})?;
    moss_defaults_set_text_color("DOCUMENT_TITLE_COLOR".to_string(), TextColors{ foreground: selected, background: Some(background)})?;
    moss_defaults_set_text_color("DOCUMENT_TITLE_COLOR_INVERTED".to_string(), TextColors { foreground: background, background: Some(selected)})?;
    moss_defaults_set_text_color("DOCUMENT_SUBTITLE_COLOR".to_string(), TextColors { foreground: subtitle_gray, background: Some(background) })?;
    moss_defaults_set_text_color("TEXT_COLOR_T".to_string(), TextColors { foreground: white, background: None })?;
    moss_defaults_set_text_color("TEXT_COLOR_H".to_string(), TextColors { foreground: background, background: None })?;

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
pub unsafe fn moss_extension_loop(Json(state): Json<MossState>) -> FnResult<()> {
    Ok(())
}

#[plugin_fn]
pub fn moss_extension_unregister() -> FnResult<()> {
    Ok(())
}
