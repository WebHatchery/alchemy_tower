use super::*;

#[test]
fn everything_a_brew_can_be_poured_on_can_actually_be_treated() {
    let data = load_embedded().expect("embedded game data should load");
    let mut targets = 0usize;
    let mut impossible = Vec::new();

    for area in &data.areas {
        for target in &area.apply_targets {
            targets += 1;
            let candidates = data
                .items
                .iter()
                .filter(|item| item.category == crate::data::ItemCategory::Potion)
                .filter(|item| {
                    item.effects
                        .iter()
                        .any(|effect| effect.kind.to_string() == target.required_effect_kind)
                })
                .collect::<Vec<_>>();
            if candidates.is_empty() {
                impossible.push(format!(
                    "{} wants a {} brew, and nothing in the game does that",
                    target.id, target.required_effect_kind
                ));
                continue;
            }
            // Something has to be able to reach the grade it asks for.
            if !target.minimum_quality_band.is_empty()
                && !candidates
                    .iter()
                    .any(|item| data.recipes.iter().any(|r| r.output_item_id == item.id))
            {
                impossible.push(format!(
                    "{} wants {} of a {} brew, and no recipe makes one",
                    target.id, target.minimum_quality_band, target.required_effect_kind
                ));
            }
        }
    }

    assert!(
        targets > 0,
        "nothing in the world can have a brew used on it"
    );
    assert!(
        impossible.is_empty(),
        "targets no brew could ever treat:
{impossible:#?}"
    );
}
#[test]
fn something_in_the_world_opens_only_by_treating_it() {
    let data = load_embedded().expect("embedded game data should load");
    let target_beats = data
        .areas
        .iter()
        .flat_map(|area| area.apply_targets.iter())
        .flat_map(|target| target.completion_milestones.iter())
        .map(|milestone| milestone.id.clone())
        .collect::<std::collections::HashSet<_>>();

    let gated = data
        .areas
        .iter()
        .flat_map(|area| area.warps.iter())
        .any(|warp| target_beats.contains(&warp.required_journal_milestone))
        || data
            .stations
            .iter()
            .any(|station| target_beats.contains(&station.required_journal_milestone));

    assert!(
        gated,
        "no route or facility waits on a brew being poured on anything"
    );
}
#[test]
fn something_in_the_game_happens_after_the_ending() {
    let data = load_embedded().expect("embedded game data should load");
    let ending = crate::content::narrative_text()
        .milestones
        .observatory_ending
        .id
        .clone();

    let requests = data
        .quests
        .iter()
        .filter(|quest| quest.required_journal_milestone == ending)
        .count();
    let commissions = data
        .quests
        .iter()
        .filter(|quest| quest.required_journal_milestone == ending && quest.coin_cost > 0)
        .count();

    assert!(
        requests >= 3,
        "only {requests} request(s) wait on the ending; the valley stops the day it is finished"
    );
    assert!(
            commissions >= 1,
            "nothing after the ending costs anything, so a finished campaign's coins have nowhere left to go"
        );
}
#[test]
fn the_ending_opens_ground_and_not_only_paperwork() {
    let data = load_embedded().expect("embedded game data should load");
    let ending = crate::content::narrative_text()
        .milestones
        .observatory_ending
        .id
        .clone();

    let opened = data
        .areas
        .iter()
        .flat_map(|area| area.gather_nodes.iter())
        .filter(|node| node.required_journal_milestone == ending)
        .map(|node| node.route_id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
            opened.len() >= 2,
            "the ending opens {} route(s) of ground; the valley stops being walkable the day it is finished",
            opened.len()
        );
    for route_id in &opened {
        assert!(
            data.route(route_id).is_some(),
            "post-ending ground is filed under {route_id}, which is not a route"
        );
    }
}
#[test]
fn the_rooms_change_after_the_ending_and_not_only_the_valley() {
    let data = load_embedded().expect("embedded game data should load");
    let ending = crate::content::narrative_text()
        .milestones
        .observatory_ending
        .id
        .clone();

    // Everything the ending leads to: itself, plus whatever the requests
    // that wait on it record when they are finished.
    let mut after = std::collections::HashSet::from([ending.clone()]);
    for quest in &data.quests {
        if quest.required_journal_milestone == ending {
            after.extend(
                quest
                    .completion_milestones
                    .iter()
                    .map(|milestone| milestone.id.clone()),
            );
        }
    }

    let worked_in = data
        .stations
        .iter()
        .map(|station| station.area_id.as_str())
        .collect::<std::collections::HashSet<_>>();

    let changed = data
        .areas
        .iter()
        .filter(|area| worked_in.contains(area.id.as_str()))
        .filter(|area| {
            area.gather_nodes
                .iter()
                .any(|node| after.contains(&node.required_journal_milestone))
                || area.flourishes.iter().any(|flourish| {
                    flourish
                        .after_any_journal_milestone
                        .iter()
                        .any(|beat| after.contains(beat))
                })
        })
        .map(|area| area.id.as_str())
        .collect::<std::collections::BTreeSet<_>>();

    assert!(
        changed.len() >= 2,
        "only {} room(s) the player works in change after the ending: {changed:?}",
        changed.len()
    );
}
#[test]
fn every_flourish_waits_on_something_real() {
    use crate::data::game_data_narrative_tests::tests::recordable_milestone_ids;

    let data = load_embedded().expect("embedded game data should load");
    let recordable = recordable_milestone_ids(&data);
    let mut flourishes = 0usize;
    let mut dangling = Vec::new();

    for area in &data.areas {
        for flourish in &area.flourishes {
            flourishes += 1;
            for quest_id in &flourish.after_any_completed_quest {
                if data.quest(quest_id).is_none() {
                    dangling.push(format!("{} waits on quest {quest_id}", flourish.id));
                }
            }
            for milestone_id in &flourish.after_any_journal_milestone {
                if !recordable.contains(milestone_id) {
                    dangling.push(format!("{} waits on beat {milestone_id}", flourish.id));
                }
            }
            if flourish.shapes.is_empty() {
                dangling.push(format!("{} draws nothing at all", flourish.id));
            }
        }
    }

    assert!(flourishes > 0, "the world never changes for anything");
    assert!(
        dangling.is_empty(),
        "flourishes waiting on things that never happen:
{dangling:#?}"
    );
}
#[test]
fn the_world_changes_in_more_than_a_couple_of_places() {
    let data = load_embedded().expect("embedded game data should load");
    let areas = data
        .areas
        .iter()
        .filter(|area| !area.flourishes.is_empty())
        .count();
    let total = data
        .areas
        .iter()
        .map(|area| area.flourishes.len())
        .sum::<usize>();

    assert!(
        areas >= 3 && total >= 6,
        "only {total} flourishes across {areas} areas; the world barely notices what you do"
    );
}
#[test]
fn every_room_the_player_works_in_changes_for_something() {
    let data = load_embedded().expect("embedded game data should load");

    let working_rooms = data
        .stations
        .iter()
        .map(|station| station.area_id.clone())
        .collect::<std::collections::BTreeSet<_>>();

    let mut unchanging = working_rooms
        .iter()
        .filter(|area_id| {
            data.areas
                .iter()
                .find(|area| &&area.id == area_id)
                .is_none_or(|area| area.flourishes.is_empty())
        })
        .cloned()
        .collect::<Vec<_>>();

    unchanging.sort();
    assert!(
        unchanging.is_empty(),
        "rooms the player works in that never change: {unchanging:?}"
    );
}
#[test]
fn treating_and_funding_things_leads_somewhere() {
    let data = load_embedded().expect("embedded game data should load");

    let mut consumed = std::collections::HashSet::new();
    for area in &data.areas {
        for warp in &area.warps {
            consumed.insert(warp.required_journal_milestone.clone());
        }
        for node in &area.gather_nodes {
            consumed.insert(node.required_journal_milestone.clone());
        }
        for flourish in &area.flourishes {
            consumed.extend(flourish.after_any_journal_milestone.iter().cloned());
        }
    }
    for station in &data.stations {
        consumed.insert(station.required_journal_milestone.clone());
    }

    let mut orphaned = Vec::new();

    for area in &data.areas {
        for target in &area.apply_targets {
            if !target
                .completion_milestones
                .iter()
                .any(|milestone| consumed.contains(&milestone.id))
            {
                orphaned.push(format!("treating {} opens nothing", target.id));
            }
        }
    }
    for quest in data.quests.iter().filter(|quest| quest.coin_cost > 0) {
        if !quest
            .completion_milestones
            .iter()
            .any(|milestone| consumed.contains(&milestone.id))
        {
            orphaned.push(format!("funding {} changes nothing", quest.id));
        }
    }

    orphaned.sort();
    assert!(
        orphaned.is_empty(),
        "work whose only payoff is a journal entry:
{orphaned:#?}"
    );
}
#[test]
fn a_recipe_only_asks_for_a_bottle_at_a_bench_that_takes_bottles() {
    use crate::data::ItemCategory;

    let data = load_embedded().expect("embedded game data should load");
    let mut second_order = 0usize;
    let mut misplaced = Vec::new();

    for recipe in &data.recipes {
        let bottles = recipe
            .ingredients
            .iter()
            .filter(|ingredient| {
                data.item(&ingredient.item_id)
                    .is_some_and(|item| item.category == ItemCategory::Potion)
            })
            .count();
        if bottles == 0 {
            continue;
        }
        second_order += 1;
        let takes_bottles = data
            .stations
            .iter()
            .find(|station| station.id == recipe.station_id)
            .is_some_and(|station| station.accepts_potions);
        if !takes_bottles {
            misplaced.push(format!(
                "{} wants {bottles} finished bottle(s) at {}, which will not take one",
                recipe.id, recipe.station_id
            ));
        }
    }

    assert!(
        second_order > 0,
        "no recipe consumes a finished bottle; the deep benches still make vendor trash"
    );
    assert!(
        misplaced.is_empty(),
        "recipes asking for bottles at a bench that refuses them:
{misplaced:#?}"
    );
}
#[test]
fn the_late_tier_does_not_make_its_own_vendor_trash() {
    use crate::data::ItemCategory;

    let data = load_embedded().expect("embedded game data should load");

    let mut wanted = std::collections::HashSet::new();
    for quest in &data.quests {
        wanted.insert(quest.required_item_id.clone());
    }
    for recipe in &data.recipes {
        for ingredient in &recipe.ingredients {
            wanted.insert(ingredient.item_id.clone());
        }
    }
    for rune in &data.rune_recipes {
        wanted.insert(rune.input_item_id.clone());
    }

    let mut unwanted = Vec::new();
    for recipe in &data.recipes {
        let second_order = recipe.ingredients.iter().any(|ingredient| {
            data.item(&ingredient.item_id)
                .is_some_and(|item| item.category == ItemCategory::Potion)
        });
        if second_order && !wanted.contains(&recipe.output_item_id) {
            unwanted.push(format!(
                "{} makes {}, which nothing asks for",
                recipe.id, recipe.output_item_id
            ));
        }
    }

    unwanted.sort();
    assert!(
        unwanted.is_empty(),
        "compound bottles with nowhere to go:
{unwanted:#?}"
    );
}
#[test]
fn a_morph_branch_pays_out_in_something_somebody_wants() {
    let data = load_embedded().expect("embedded game data should load");

    let mut wanted = std::collections::HashSet::new();
    for quest in &data.quests {
        wanted.insert(quest.required_item_id.clone());
    }
    for recipe in &data.recipes {
        for ingredient in &recipe.ingredients {
            wanted.insert(ingredient.item_id.clone());
        }
    }
    for rune in &data.rune_recipes {
        wanted.insert(rune.input_item_id.clone());
    }

    // A bottle an ordinary recipe also makes is somebody else's problem —
    // this guard is about what the *branch* is worth reaching for.
    let plainly_brewable = data
        .recipes
        .iter()
        .map(|recipe| recipe.output_item_id.clone())
        .collect::<std::collections::HashSet<_>>();

    let mut unwanted = data
        .recipes
        .iter()
        .flat_map(|recipe| {
            recipe
                .morph_targets
                .iter()
                .map(move |morph| (recipe, &morph.output_item_id))
        })
        .filter(|(_, output)| !wanted.contains(*output) && !plainly_brewable.contains(*output))
        .map(|(recipe, output)| format!("{} branches into {output}, unasked for", recipe.id))
        .collect::<Vec<_>>();

    unwanted.sort();
    unwanted.dedup();
    assert!(
        unwanted.is_empty(),
        "precision brewing that pays out in vendor trash:
{unwanted:#?}"
    );
}
#[test]
fn every_imbuing_the_rune_floor_makes_is_wanted_by_something() {
    let data = load_embedded().expect("embedded game data should load");

    let mut wanted = std::collections::HashSet::new();
    for quest in &data.quests {
        wanted.insert(quest.required_item_id.clone());
    }
    for recipe in &data.recipes {
        for ingredient in &recipe.ingredients {
            wanted.insert(ingredient.item_id.clone());
        }
    }
    for rune in &data.rune_recipes {
        wanted.insert(rune.input_item_id.clone());
    }
    for area in &data.areas {
        for warp in &area.warps {
            wanted.insert(warp.required_item_id.clone());
        }
    }

    let mut unwanted = data
        .rune_recipes
        .iter()
        .filter(|rune| !wanted.contains(&rune.output_item_id))
        .map(|rune| format!("{} makes {}, unasked for", rune.id, rune.output_item_id))
        .collect::<Vec<_>>();

    unwanted.sort();
    assert!(
        unwanted.is_empty(),
        "imbuings the valley has no use for:
{unwanted:#?}"
    );
}
#[test]
fn no_bench_makes_more_vendor_trash_than_it_makes_work() {
    let data = load_embedded().expect("embedded game data should load");

    let mut wanted = std::collections::HashSet::new();
    for quest in &data.quests {
        wanted.insert(quest.required_item_id.clone());
    }
    for recipe in &data.recipes {
        for ingredient in &recipe.ingredients {
            wanted.insert(ingredient.item_id.clone());
        }
    }
    for rune in &data.rune_recipes {
        wanted.insert(rune.input_item_id.clone());
    }

    let mut counts: std::collections::BTreeMap<&str, (usize, usize)> =
        std::collections::BTreeMap::new();
    for recipe in &data.recipes {
        let entry = counts.entry(recipe.station_id.as_str()).or_default();
        if wanted.contains(&recipe.output_item_id) {
            entry.0 += 1;
        } else {
            entry.1 += 1;
        }
    }

    let idle = counts
        .iter()
        .filter(|(_, (asked, unasked))| unasked >= asked)
        .map(|(station, (asked, unasked))| {
            format!("{station}: {unasked} unasked-for against {asked} asked-for")
        })
        .collect::<Vec<_>>();

    assert!(!counts.is_empty(), "no bench brews anything");
    assert!(
        idle.is_empty(),
        "benches that mostly make things to sell:
{idle:#?}"
    );
}
#[test]
fn every_plain_brew_has_somewhere_to_go() {
    let data = load_embedded().expect("embedded game data should load");

    let mut wanted = std::collections::HashSet::new();
    for quest in &data.quests {
        wanted.insert(quest.required_item_id.clone());
    }
    for recipe in &data.recipes {
        for ingredient in &recipe.ingredients {
            wanted.insert(ingredient.item_id.clone());
        }
    }
    for rune in &data.rune_recipes {
        wanted.insert(rune.input_item_id.clone());
    }
    for area in &data.areas {
        for warp in &area.warps {
            wanted.insert(warp.required_item_id.clone());
        }
    }

    let mut unwanted = data
        .recipes
        .iter()
        .filter(|recipe| !wanted.contains(&recipe.output_item_id))
        .map(|recipe| {
            format!(
                "{} makes {}, which nothing asks for",
                recipe.id, recipe.output_item_id
            )
        })
        .collect::<Vec<_>>();

    unwanted.sort();
    assert!(
        unwanted.is_empty(),
        "plain brews with nowhere to go:
{unwanted:#?}"
    );
}
#[test]
fn more_than_one_bench_takes_a_finished_bottle_and_means_it() {
    use crate::data::ItemCategory;

    let data = load_embedded().expect("embedded game data should load");
    let accepting = data
        .stations
        .iter()
        .filter(|station| station.accepts_potions)
        .collect::<Vec<_>>();

    assert!(
        accepting.len() >= 2,
        "only {} bench takes finished bottles; that is a feature of one room, not a tier",
        accepting.len()
    );

    let mut idle = accepting
        .iter()
        .filter(|station| {
            !data.recipes.iter().any(|recipe| {
                recipe.station_id == station.id
                    && recipe.ingredients.iter().any(|ingredient| {
                        data.item(&ingredient.item_id)
                            .is_some_and(|item| item.category == ItemCategory::Potion)
                    })
            })
        })
        .map(|station| station.id.clone())
        .collect::<Vec<_>>();

    idle.sort();
    assert!(
        idle.is_empty(),
        "benches that claim to take a finished bottle and are never asked for one: {idle:?}"
    );
}
#[test]
fn the_late_tier_is_deeper_than_the_middle_of_the_game() {
    use crate::data::ItemCategory;

    let data = load_embedded().expect("embedded game data should load");
    let mut shallow = Vec::new();

    for recipe in &data.recipes {
        let second_order = recipe.ingredients.iter().any(|ingredient| {
            data.item(&ingredient.item_id)
                .is_some_and(|item| item.category == ItemCategory::Potion)
        });
        if !second_order {
            continue;
        }
        if recipe.ingredients.len() < 3
            || recipe.required_sequence.len() < 3
            || recipe.morph_targets.len() < 2
        {
            shallow.push(format!(
                "{}: {} reagents, {}-step sequence, {} branches",
                recipe.id,
                recipe.ingredients.len(),
                recipe.required_sequence.len(),
                recipe.morph_targets.len()
            ));
        }
    }

    assert!(
        shallow.is_empty(),
        "late-tier recipes that are flat variants in disguise:
{shallow:#?}"
    );
}
