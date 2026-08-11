use crate::data::load_embedded;

/// A target asks for a kind of brew and, sometimes, a grade. Both are
/// strings in a content file, so both can be wrong in ways nothing else
/// would catch: an effect kind no potion produces makes a target that can
/// never be treated, and since gates now wait on targets, that is a wall
/// with no door rather than a missed flourish.
/// The premise on the critical path. Applying a brew to the world was
/// supposed to open things, not merely decorate them — if no gate anywhere
/// waits on a treated target, the whole mechanic is optional scenery.
/// The ending was a wall. Every one of the game's requests, nodes, warps and
/// flourishes was reachable *before* the observatory, so the moment a player
/// finished the thing the whole game builds towards, the valley had nine
/// sentences of last words and then never changed again — in a game whose
/// own scope note says a finished product is twenty to twenty-five hours.
///
/// The ending beat is read off the narrative spine rather than spelled here,
/// so renaming it cannot quietly turn this into a test of nothing.
/// And the half of that wall the request count could not see. The pass that
/// answered "the ending is a wall" answered it entirely in **paperwork** —
/// three standing orders, a commission and an unsigned note — and left the
/// valley itself frozen: of 85 gather nodes, 23 routes, 6 apply targets and
/// 14 areas, **not one** waited on the ending. A player who finished the
/// thing the whole game builds towards could go on being paid, and had
/// nowhere new to stand while it happened.
///
/// This is the world counterpart, and it asks for ground rather than for a
/// number of nodes: the ending has to open at least two *routes*, because
/// one route is a corner and the rule this project already wrote down for
/// flourishes is that a world change satisfied in a single place is not a
/// world change.
/// And the half *that* guard could not see either. The pass which opened
/// ground after the ending opened it in the plains and on the lake shore —
/// outdoors, both of them — so the **building the whole game is about
/// reopening** was still the one place the ending changed nothing. Measured
/// afterwards: of the six tower rooms, one carried a single post-ending
/// flourish and no room carried a post-ending node, station, apply target
/// or warp.
///
/// "A room the player works in" is derived from where the stations are,
/// exactly as the flourish guard derives it, so a bench on a new floor is
/// covered the day it is placed and the valley's outdoor routes correctly
/// do not count towards this.
/// A flourish waits on a quest or a beat, both of which are strings in an
/// area file. One that names something that does not exist is a piece of
/// the world that never appears, and nothing on screen would say why.
/// The point of moving these into data was coverage. Twelve story chains
/// finish in this game and the world used to acknowledge four of them,
/// across two areas, because each one was a `match` arm somebody had to
/// write. This is a floor, not a target.
/// The floor above counts flourishes and is satisfied by putting them all in
/// one room, which is exactly what happened: nine of the first fourteen were
/// in the town square, and the tower — the building this game is *about*
/// reopening — changed in two of its six rooms. The entry lab, where the
/// player starts every day and brews for the first several hours, changed
/// for nothing at all.
///
/// A room the player works in is derived from the stations, not listed here,
/// so a new bench on a new floor is covered the day it is placed.
/// Every beat a milestone-writer records should lead somewhere. Treating a
/// thing, or funding a commission, is expensive — bottles at a grade, and
/// for a commission thousands of coins — and a payoff that exists only as a
/// journal entry reads as a receipt rather than a change.
///
/// "Somewhere" is deliberately broad: a route, a facility, ground that
/// starts growing, or a visible change in the world all count. What does
/// not count is nothing.
/// Second-order brewing: a bench that takes finished bottles as reagents.
/// It is the only structural sink the deep benches' outputs have — nothing
/// asks for a benchlight solution, so the way it stops being vendor trash
/// is for something else to need one.
///
/// The rule this pins is the one a content author will trip over: a recipe
/// may only call for a potion at a bench that accepts potions. The entry
/// cauldron does not, and a recipe there naming a bottle would be
/// unfillable with nothing on screen to explain it.
/// The tier was built to give the deep benches' outputs a destination, so
/// it must not become the new worst offender. A compound bottle costs two
/// finished brews and a reagent to make; if nothing then asks for it, the
/// whole chain terminates in vendor trash one layer further up than before.
///
/// "Asks for it" is deliberately broad — a request, a repeatable order, a
/// commission, a rune pattern, or another recipe using it as a reagent. A
/// morph target does not count: that is another way to *make* the thing,
/// not a reason to have one.
/// A morph branch is the hardest thing the brewing system asks for: the
/// quality bar, the exact heat and stir count, the timing word, sometimes a
/// named catalyst, a reagent order and the room bonus, all at once. Thirteen
/// of the twenty-nine bottles only a branch can make were wanted by nothing
/// — so the reward for the deepest verb in the tower was a thing to sell.
///
/// "Wanted" is the same broad definition the compound tier uses: a request,
/// a repeatable order, a commission, a rune pattern, or a reagent slot. A
/// second morph reaching the same bottle deliberately does not count; that
/// is another way to make it, not a reason to have one.
/// The rune floor's whole verb is "take a bottle you can already make and
/// rework it into something else", and nine of its seventeen imbuings came
/// out to a thing nobody wanted. Worse, they were the *early* nine: the
/// inputs are the glow potion, the healing draught, the lantern draught,
/// calmleaf, the verdant restorative and the stamina tonic — everything a
/// player learns in act one. So the most natural first use of a newly opened
/// floor, improve what I am already good at, paid out in vendor trash every
/// single time, while the eight imbuings the valley did want all sat at the
/// far end of the game.
///
/// Same broad definition of "wanted" the compound tier and the morph
/// branches use: a request, a repeatable order, a commission, a rune input,
/// a reagent slot, or a warp toll.
/// The same question as the rune and morph guards, asked one level up: not
/// "is this bottle wanted" but **"is this room worth working in"**.
///
/// Counted per bench, the answer was no for the greenhouse still — **six of
/// its twelve recipes made a bottle nothing in the game asks for**, which
/// is the bench act two lives at. And the six were not a random half. Every
/// wanted greenhouse output has a morph branch or feeds another recipe;
/// every unwanted one is flat — two reagents, one bottle, no branch, no
/// downstream, no buyer. They are the *place* recipes: the salve made of
/// what grows on the terraces Brin uncovered, the tonic made of the plant
/// and the seed it throws from opposite ends of a nine-year question, the
/// draught made of pollen that did not exist in the valley a season ago.
/// The game opened the ground, authored the brew that ground exists for,
/// and then nobody ever asked for the result.
///
/// The rule is a floor rather than a target: a bench must want more of its
/// own output than it wastes. Six of twelve is not more, so this fails on
/// the state that prompted it.
/// The per-bench floor exposed a final tail of ten ordinary recipes whose
/// output nothing in the game asked for: seven at the entry cauldron, two
/// at the cold bench and one at the reading bench. They had no shared
/// mechanical cause. What they did share was authored prose that already
/// named a buyer, so the missing half was demand rather than another recipe
/// layer.
///
/// With that tail routed, the stronger invariant is honest: every ordinary
/// recipe ends in a request, a reagent slot, a rune pattern or a gate. The
/// morph and rune-output counterparts remain separate because they test the
/// rewards for different verbs.
/// Second-order brewing existed at exactly one bench in a five-bench tower,
/// which made it a feature of the archive rather than a tier. The measure
/// that showed it: counting, per bench, how many of its outputs anything
/// anywhere uses as a reagent. The archive ate four of its own and the
/// greenhouse fed four; **the entry cauldron's twenty-four outputs and the
/// rune forge's five fed nothing at all, anywhere**. Three of the forge's
/// five recipes made bottles nobody wanted, which is the worst ratio in the
/// building, and it is a floor of the tower.
///
/// Two rules, both about a claim matching a fact: a tier is not one room,
/// and a bench that advertises it will take a finished bottle has to have
/// something that actually asks for one.
/// The late tier exists to deepen decisions, not to lengthen a list. A
/// second-order recipe should use the parts of the lattice the mid-game
/// barely touches: three reagents, a full sequence, and a branch.
/// The apply-potion verb is the game's answer to "a brew is for pouring on
/// something, not only for drinking or handing over", and it shipped with
/// exactly the three examples the TODO listed — two restore, one misfire.
/// A whole effect kind with nothing to pour it on means a player who brews
/// lights has no reason to carry one anywhere but a dark node.
///
/// Effect kinds come from the potions that exist, so this cannot go stale
/// against a fifth kind being authored.
mod test_part_1;
mod test_part_2;
