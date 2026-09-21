//! Semantic validation for authored game content.

use std::collections::HashSet;

use super::GameData;

impl GameData {
    pub(super) fn validate(&self) -> Result<(), String> {
        self.validate_ids()?;
        self.validate_config()?;
        self.validate_world_references()?;
        self.validate_crafting_references()?;
        self.validate_narrative_references()?;
        self.validate_invariants()
    }

    fn validate_ids(&self) -> Result<(), String> {
        check_unique(self.areas.iter().map(|entry| entry.id.as_str()), "area")?;
        check_unique(self.items.iter().map(|entry| entry.id.as_str()), "item")?;
        check_unique(
            self.gathering_routes.iter().map(|entry| entry.id.as_str()),
            "gathering route",
        )?;
        check_unique(self.npcs.iter().map(|entry| entry.id.as_str()), "NPC")?;
        check_unique(self.quests.iter().map(|entry| entry.id.as_str()), "quest")?;
        check_unique(
            self.stations.iter().map(|entry| entry.id.as_str()),
            "station",
        )?;
        check_unique(self.recipes.iter().map(|entry| entry.id.as_str()), "recipe")?;
        check_unique(
            self.rune_recipes.iter().map(|entry| entry.id.as_str()),
            "rune recipe",
        )?;
        check_unique(
            self.mutation_formulas.iter().map(|entry| entry.id.as_str()),
            "mutation formula",
        )
    }

    fn validate_config(&self) -> Result<(), String> {
        require(
            self.area_exists(&self.config.starting_area),
            "config.starting_area",
        )?;
        positive(self.config.move_speed, "config.move_speed")?;
        positive(self.config.interaction_range, "config.interaction_range")?;
        positive(self.config.day_length_seconds, "config.day_length_seconds")?;
        if !self.config.tutorial_hint_delay_seconds.is_finite()
            || self.config.tutorial_hint_delay_seconds < 0.0
        {
            return Err(
                "config.tutorial_hint_delay_seconds must be finite and non-negative".to_owned(),
            );
        }
        for quest_id in &self.config.archive_required_completed_quests {
            require(
                self.quest_exists(quest_id),
                &format!("config quest {quest_id}"),
            )?;
        }
        self.validate_quality_percents()
    }

    fn validate_quality_percents(&self) -> Result<(), String> {
        let balance = &self.config.balance;
        for (name, value) in [
            ("crude", balance.quality_value_percent.crude),
            ("serviceable", balance.quality_value_percent.serviceable),
            ("fine", balance.quality_value_percent.fine),
            ("excellent", balance.quality_value_percent.excellent),
            ("masterwork", balance.quality_value_percent.masterwork),
            ("crude effect", balance.quality_effect_percent.crude),
            (
                "serviceable effect",
                balance.quality_effect_percent.serviceable,
            ),
            ("fine effect", balance.quality_effect_percent.fine),
            ("excellent effect", balance.quality_effect_percent.excellent),
            (
                "masterwork effect",
                balance.quality_effect_percent.masterwork,
            ),
        ] {
            if value == 0 {
                return Err(format!("config.balance {name} percentage must be positive"));
            }
        }
        let rapport = &balance.rapport;
        if !(0 < rapport.friend
            && rapport.friend < rapport.confidant
            && rapport.confidant < rapport.kin)
        {
            return Err("config.balance.rapport thresholds must increase".to_owned());
        }
        let salvage = &balance.salvage;
        if salvage.blind_cap == 0
            || salvage.cap_per_attempt == 0
            || salvage.bonus_per_attempt == 0
            || salvage.practice_cap == 0
            || salvage.discovery_attempts == 0
            || salvage.practice_cap > salvage.blind_cap
        {
            return Err("config.balance.salvage values are inconsistent".to_owned());
        }
        self.validate_alchemy_tuning()?;
        Ok(())
    }

    fn validate_alchemy_tuning(&self) -> Result<(), String> {
        let alchemy = &self.config.balance.alchemy;
        if alchemy.min_heat > alchemy.max_heat
            || !(alchemy.min_heat..=alchemy.max_heat).contains(&alchemy.initial_heat)
        {
            return Err("config.balance.alchemy heat bounds are inconsistent".to_owned());
        }
        if alchemy.timing_ids.is_empty()
            || alchemy.timing_ids.iter().any(String::is_empty)
            || has_duplicates(alchemy.timing_ids.iter().map(String::as_str))
        {
            return Err(
                "config.balance.alchemy timing_ids must be unique and non-empty".to_owned(),
            );
        }
        validate_thresholds(
            &alchemy.quality_bands,
            |band| band.minimum_score,
            |band| band.id.as_str(),
            "quality bands",
        )?;
        validate_thresholds(
            &alchemy.mastery_stages,
            |stage| stage.minimum_brews,
            |stage| stage.id.as_str(),
            "mastery stages",
        )?;
        if alchemy
            .quality_bands
            .first()
            .is_none_or(|band| band.minimum_score != 0)
            || alchemy
                .mastery_stages
                .first()
                .is_none_or(|stage| stage.minimum_brews != 0)
        {
            return Err("config.balance.alchemy thresholds must start at zero".to_owned());
        }
        if alchemy
            .quality_bands
            .iter()
            .any(|band| band.label_key.is_empty())
            || alchemy
                .mastery_stages
                .iter()
                .any(|stage| stage.label_key.is_empty())
        {
            return Err("config.balance.alchemy threshold labels must be non-empty".to_owned());
        }
        let scoring = &alchemy.scoring;
        if scoring.catalyst_quality_divisor == 0 || scoring.score_cap == 0 {
            return Err(
                "config.balance.alchemy scoring divisors and caps must be positive".to_owned(),
            );
        }
        let volatility = &alchemy.volatility;
        if volatility.instability_limit == 0
            || volatility.volatile_trait.is_empty()
            || volatility.stabilizer_traits.is_empty()
            || volatility.stabilizer_traits.iter().any(String::is_empty)
        {
            return Err("config.balance.alchemy volatility identifiers are incomplete".to_owned());
        }
        Ok(())
    }

    fn validate_world_references(&self) -> Result<(), String> {
        for route in &self.gathering_routes {
            require(
                self.area_exists(&route.area_id),
                &format!("route {} area", route.id),
            )?;
        }
        for station in &self.stations {
            require(
                self.area_exists(&station.area_id),
                &format!("station {} area", station.id),
            )?;
            for stock in &station.stock {
                require(
                    self.item_exists(&stock.item_id),
                    &format!("station {} stock", station.id),
                )?;
                if stock.price == 0 {
                    return Err(format!("station {} has a free stock item", station.id));
                }
            }
            for item_id in &station.planter_seed_ids {
                require(
                    self.item_exists(item_id),
                    &format!("station {} seed", station.id),
                )?;
            }
            if !station.habitat_output_item_id.is_empty() {
                require(
                    self.item_exists(&station.habitat_output_item_id),
                    &format!("station {} habitat output", station.id),
                )?;
            }
        }
        for npc in &self.npcs {
            require(
                self.area_exists(&npc.area_id),
                &format!("NPC {} area", npc.id),
            )?;
            for area_id in npc.schedule.iter().map(|entry| &entry.area_id) {
                require(
                    self.area_exists(area_id),
                    &format!("NPC {} schedule area", npc.id),
                )?;
            }
            for quest_id in npc.quest_chain() {
                require(
                    self.quest_exists(quest_id),
                    &format!("NPC {} quest", npc.id),
                )?;
            }
            for item_id in [&npc.friendship_reward_item_id, &npc.trusted_reward_item_id] {
                if !item_id.is_empty() {
                    require(self.item_exists(item_id), &format!("NPC {} reward", npc.id))?;
                }
            }
        }
        for area in &self.areas {
            for warp in &area.warps {
                require(
                    self.area_exists(&warp.target_area),
                    &format!("warp {} target", warp.id),
                )?;
                validate_optional_item(
                    self,
                    &warp.required_item_id,
                    &format!("warp {} item", warp.id),
                )?;
                validate_optional_quest(
                    self,
                    &warp.required_completed_quest,
                    &format!("warp {} quest", warp.id),
                )?;
            }
            for node in &area.gather_nodes {
                require(
                    self.item_exists(&node.item_id),
                    &format!("node {} item", node.id),
                )?;
                require(
                    self.route_exists(&node.route_id),
                    &format!("node {} route", node.id),
                )?;
                validate_optional_quest(
                    self,
                    &node.required_completed_quest,
                    &format!("node {} quest", node.id),
                )?;
            }
            for target in &area.apply_targets {
                validate_effect(
                    &target.required_effect_kind,
                    &format!("target {} effect", target.id),
                )?;
            }
        }
        Ok(())
    }

    fn validate_crafting_references(&self) -> Result<(), String> {
        for recipe in &self.recipes {
            require(
                self.station_exists(&recipe.station_id),
                &format!("recipe {} station", recipe.id),
            )?;
            require(
                self.item_exists(&recipe.output_item_id),
                &format!("recipe {} output", recipe.id),
            )?;
            require(
                self.item_exists(&recipe.unstable_output_item_id),
                &format!("recipe {} unstable output", recipe.id),
            )?;
            if recipe.ingredients.is_empty() || recipe.output_amount == 0 {
                return Err(format!(
                    "recipe {} must have ingredients and output",
                    recipe.id
                ));
            }
            for ingredient in &recipe.ingredients {
                require(
                    self.item_exists(&ingredient.item_id),
                    &format!("recipe {} ingredient", recipe.id),
                )?;
                if ingredient.amount == 0 {
                    return Err(format!("recipe {} has a zero-amount ingredient", recipe.id));
                }
            }
            for morph in &recipe.morph_targets {
                require(
                    self.item_exists(&morph.output_item_id),
                    &format!("recipe {} morph output", recipe.id),
                )?;
                validate_timing_id(
                    &self.config.balance.alchemy.timing_ids,
                    &morph.required_timing,
                    &format!("recipe {} morph timing", recipe.id),
                )?;
            }
            validate_timing_id(
                &self.config.balance.alchemy.timing_ids,
                &recipe.required_timing,
                &format!("recipe {} timing", recipe.id),
            )?;
        }
        for recipe in &self.rune_recipes {
            require(
                self.station_exists(&recipe.station_id),
                &format!("rune {} station", recipe.id),
            )?;
            for item_id in [
                &recipe.input_item_id,
                &recipe.rune_item_id,
                &recipe.output_item_id,
            ] {
                require(
                    self.item_exists(item_id),
                    &format!("rune {} item", recipe.id),
                )?;
            }
        }
        for formula in &self.mutation_formulas {
            require(
                self.item_exists(&formula.seed_item_id),
                &format!("mutation {} seed", formula.id),
            )?;
            if !formula.required_effect_kind.is_empty() {
                validate_effect(
                    &formula.required_effect_kind,
                    &format!("mutation {} effect", formula.id),
                )?;
            }
        }
        Ok(())
    }

    fn validate_narrative_references(&self) -> Result<(), String> {
        for quest in &self.quests {
            require(
                self.item_exists(&quest.required_item_id),
                &format!("quest {} item", quest.id),
            )?;
            if quest.giver_npc_id != "quest_board" {
                require(
                    self.npc_exists(&quest.giver_npc_id),
                    &format!("quest {} giver", quest.id),
                )?;
            }
            for prerequisite in &quest.prerequisite_quests {
                require(
                    self.quest_exists(prerequisite),
                    &format!("quest {} prerequisite", quest.id),
                )?;
            }
            validate_optional_npc(
                self,
                &quest.rapport_npc_id,
                &format!("quest {} rapport NPC", quest.id),
            )?;
            validate_optional_npc(
                self,
                &quest.required_rapport_npc_id,
                &format!("quest {} required NPC", quest.id),
            )?;
            validate_quality_band(
                &quest.minimum_quality_band,
                &format!("quest {} quality", quest.id),
                &self
                    .config
                    .balance
                    .alchemy
                    .quality_bands
                    .iter()
                    .map(|band| band.id.clone())
                    .collect::<Vec<_>>(),
            )?;
        }
        Ok(())
    }

    fn validate_invariants(&self) -> Result<(), String> {
        for item in &self.items {
            if item.quality > 100 || item.synthesis_weight == 0 {
                return Err(format!(
                    "item {} has invalid quality or synthesis weight",
                    item.id
                ));
            }
            for effect in &item.effects {
                if effect.magnitude < 0.0 || effect.duration_seconds < 0.0 {
                    return Err(format!("item {} has a negative effect value", item.id));
                }
            }
        }
        for area in &self.areas {
            if area.size.iter().any(|value| *value <= 0.0) {
                return Err(format!("area {} must have positive dimensions", area.id));
            }
        }
        if self.config.balance.vitality.brew_cost <= 0.0
            || self.config.balance.vitality.gather_cost <= 0.0
            || self.config.balance.vitality.sleep_restores <= 0.0
            || self.config.balance.vitality.collapse_restores <= 0.0
        {
            return Err("config.balance.vitality values must be positive".to_owned());
        }
        let alchemy = &self.config.balance.alchemy;
        if alchemy.min_heat <= 0
            || alchemy.max_heat < alchemy.min_heat
            || !(alchemy.min_heat..=alchemy.max_heat).contains(&alchemy.initial_heat)
        {
            return Err("config.balance.alchemy heat bounds are inconsistent".to_owned());
        }
        Ok(())
    }

    fn area_exists(&self, id: &str) -> bool {
        self.areas.iter().any(|entry| entry.id == id)
    }

    fn item_exists(&self, id: &str) -> bool {
        self.items.iter().any(|entry| entry.id == id)
    }

    fn route_exists(&self, id: &str) -> bool {
        self.gathering_routes.iter().any(|entry| entry.id == id)
    }

    fn npc_exists(&self, id: &str) -> bool {
        self.npcs.iter().any(|entry| entry.id == id)
    }

    fn quest_exists(&self, id: &str) -> bool {
        self.quests.iter().any(|entry| entry.id == id)
    }

    fn station_exists(&self, id: &str) -> bool {
        self.stations.iter().any(|entry| entry.id == id)
    }
}

fn check_unique<'a>(ids: impl IntoIterator<Item = &'a str>, kind: &str) -> Result<(), String> {
    let mut seen = HashSet::new();
    for id in ids {
        if id.is_empty() || !seen.insert(id) {
            return Err(format!("{kind} ids must be non-empty and unique"));
        }
    }
    Ok(())
}

fn require(found: bool, label: &str) -> Result<(), String> {
    found
        .then_some(())
        .ok_or_else(|| format!("unresolved content reference: {label}"))
}

fn positive(value: f32, label: &str) -> Result<(), String> {
    (value.is_finite() && value > 0.0)
        .then_some(())
        .ok_or_else(|| format!("{label} must be finite and positive"))
}

fn validate_optional_item(data: &GameData, id: &str, label: &str) -> Result<(), String> {
    if id.is_empty() {
        Ok(())
    } else {
        require(data.item_exists(id), label)
    }
}

fn validate_optional_quest(data: &GameData, id: &str, label: &str) -> Result<(), String> {
    if id.is_empty() {
        Ok(())
    } else {
        require(data.quest_exists(id), label)
    }
}

fn validate_optional_npc(data: &GameData, id: &str, label: &str) -> Result<(), String> {
    if id.is_empty() {
        Ok(())
    } else {
        require(data.npc_exists(id), label)
    }
}

fn validate_effect(value: &str, label: &str) -> Result<(), String> {
    ["glow", "speed", "misfire", "restore"]
        .contains(&value)
        .then_some(())
        .ok_or_else(|| format!("{label} names unknown effect {value}"))
}

fn validate_quality_band(value: &str, label: &str, known_ids: &[String]) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }
    known_ids
        .iter()
        .any(|known| known.eq_ignore_ascii_case(value))
        .then_some(())
        .ok_or_else(|| format!("{label} names unknown quality band {value}"))
}

fn validate_timing_id(known_ids: &[String], value: &str, label: &str) -> Result<(), String> {
    if value.is_empty() {
        return Ok(());
    }
    known_ids
        .iter()
        .any(|known| known == value)
        .then_some(())
        .ok_or_else(|| format!("{label} names unknown timing {value}"))
}

fn validate_thresholds<T, Score, Id>(
    values: &[T],
    score: Score,
    id: Id,
    label: &str,
) -> Result<(), String>
where
    Score: Fn(&T) -> u32,
    Id: Fn(&T) -> &str,
{
    if values.is_empty()
        || values.iter().any(|value| id(value).is_empty())
        || has_duplicates(values.iter().map(&id))
        || values
            .windows(2)
            .any(|pair| score(&pair[0]) >= score(&pair[1]))
    {
        return Err(format!(
            "config.balance.alchemy {label} must be ordered and unique"
        ));
    }
    Ok(())
}

fn has_duplicates<'a>(values: impl Iterator<Item = &'a str>) -> bool {
    let mut seen = HashSet::new();
    values.into_iter().any(|value| !seen.insert(value))
}

#[cfg(test)]
mod tests {
    use crate::data::load_embedded;

    #[test]
    fn malformed_recipe_reference_is_rejected_at_load_boundary() {
        let mut data = load_embedded().expect("fixture data should load");
        data.recipes[0].output_item_id = "missing_output".to_owned();
        let error = data.validate().expect_err("missing output should fail");
        assert!(
            error.contains("recipe") && error.contains("output"),
            "{error}"
        );
    }

    #[test]
    fn malformed_balance_is_rejected_before_indexes_are_built() {
        let mut data = load_embedded().expect("fixture data should load");
        data.config.balance.rapport.kin = 1;
        let error = data.validate().expect_err("descending rapport should fail");
        assert!(error.contains("rapport"), "{error}");
    }

    #[test]
    fn malformed_alchemy_tuning_is_rejected_before_indexes_are_built() {
        let mut data = load_embedded().expect("fixture data should load");
        data.config.balance.alchemy.quality_bands[0]
            .label_key
            .clear();
        let error = data
            .validate()
            .expect_err("missing alchemy label should fail");
        assert!(error.contains("threshold labels"), "{error}");
    }
}
