use crate::codegen::config::internal_config::RustInputPathPack;
use crate::codegen::generator::codec::structs::CodecModePack;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct ParserInternalConfig {
    pub rust_input_path_pack: RustInputPathPack,
    pub rust_crate_dir: PathBuf,
    pub force_codec_mode_pack: Option<CodecModePack>,
}
