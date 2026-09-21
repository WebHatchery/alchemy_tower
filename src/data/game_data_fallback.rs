use super::game_data::GameData;

impl GameData {
    #[cfg(test)]
    pub(crate) fn fallback() -> Self {
        super::loader::load_embedded().expect("embedded fallback game data must remain valid")
    }

    #[cfg(debug_assertions)]
    pub(crate) fn runtime_fallback() -> Self {
        let mut data: GameData = macroquad_toolkit::data_loader::load_embedded_json_labeled(
            "game_data_fallback",
            macroquad_toolkit::include_json_str!("../../assets/data/game_data_fallback.json"),
        )
        .unwrap_or_else(|error| panic!("embedded game_data_fallback should be valid: {error}"));
        data.build_indexes()
            .expect("embedded runtime fallback indexes must remain valid");
        data
    }
}
