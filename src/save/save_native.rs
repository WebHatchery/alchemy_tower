use super::save_codec::encode_save;
use super::save_native_path::save_path;
use crate::data::SaveData;

pub(crate) fn save(save_data: &SaveData) -> Result<(), String> {
    let json = encode_save(save_data)?;
    macroquad_toolkit::persistence::save_string_atomic(save_path()?, &json)
}

pub(crate) fn exists() -> bool {
    save_path().map(|path| path.exists()).unwrap_or_default()
}

pub(crate) fn load() -> Result<SaveData, String> {
    macroquad_toolkit::persistence::load_json(save_path()?)
}
