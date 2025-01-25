mod moss_definitions;
pub use moss_definitions::functions::*;
pub use moss_definitions::types::*;

use extism_pdk::*;
use serde::{Deserialize, Serialize};

#[plugin_fn]
pub unsafe fn moss_extension_register(Json(state): Json<MossState>) -> FnResult<ExtensionInfo> {
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
