use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Effect {
    id: usize,
    duration: usize,
    armor: isize,   // while active
    hpt: isize,     // each tick
    mpt: isize,     // each tick
}

impl<'a> fmt::Display for Effect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Effect {}t, {:+} armor, {:+} hp/t, {:+} mp/t", self.duration, self.armor, self.hpt, self.mpt))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Spell<'a> {
    name: &'a str,
    hp_self: isize,
    hp_target: isize,
    mp_self: isize,
    mp_target: isize,
    effect_self: Option<Effect>,
    effect_target: Option<Effect>,
}

impl<'a> fmt::Display for Spell<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Spell {}, self {:+}|{:+}, target {:+}|{:+}", self.name, self.hp_self, self.mp_self, self.hp_target, self.mp_target))
    }
}

impl<'a> Spell<'a> {
    fn mana_usage(&self) -> isize {
        if self.mp_self < 0 { -self.mp_self } else { 0 }
    }
}

pub const SPELLS: [Spell<'static>; 5] = [
    Spell { name: "Magic Missile", hp_self: 0, hp_target: -4, mp_self:  -53, mp_target: 0,   effect_self: None,                                                             effect_target: None },
    Spell { name: "Drain",         hp_self: 2, hp_target: -2, mp_self:  -73, mp_target: 0,   effect_self: None,                                                             effect_target: None },
    Spell { name: "Shield",        hp_self: 0, hp_target:  0, mp_self: -113, mp_target: 0,   effect_self: Some(Effect { id: 1, duration: 6, armor: 7, hpt:  0, mpt:   0 }), effect_target: None },
    Spell { name: "Poison",        hp_self: 0, hp_target:  0, mp_self: -173, mp_target: 0, effect_target: Some(Effect { id: 2, duration: 6, armor: 0, hpt: -3, mpt:   0 }),   effect_self: None },
    Spell { name: "Recharge",      hp_self: 0, hp_target:  0, mp_self: -229, mp_target: 0,   effect_self: Some(Effect { id: 3, duration: 5, armor: 0, hpt:  0, mpt: 101 }), effect_target: None },
];

pub trait Fighter: Sized {
    fn damage(&self) -> isize;
    fn health(&self) -> isize;
    fn health_mut(&mut self) -> &mut isize;
    fn mana(&self) -> isize;
    fn mana_mut(&mut self) -> &mut isize;
    fn effects(&self) -> &[Effect];
    fn effects_mut(&mut self) -> &mut Vec<Effect>;

    fn is_dead(&self) -> bool {
        self.health() <= 0
    }

    fn suffer(&mut self, dmg: isize) {
        if dmg != 0 { *self.health_mut() -= dmg; }
    }

    fn drain(&mut self, mn: isize) {
        if mn != 0 { *self.mana_mut() -= mn; }
    }

    fn has_effect(&self, effect: &Effect) -> bool {
        self.effects().iter().any(|e| e.id == effect.id)
    }

    fn add_effect(&mut self, effect: Effect) {
        self.effects_mut().push(effect);
    }

    fn apply_effects(&mut self) {
        let mut hpt = 0;
        let mut mpt = 0;
        for effect in self.effects_mut() {
            hpt += effect.hpt;
            mpt += effect.mpt;
            effect.duration -= 1;
        }
        self.suffer(-hpt);
        self.drain(-mpt);
        self.effects_mut().retain(|e| e.duration > 0);
    }

    fn armor(&self) -> isize {
        self.effects().iter().map(|e| e.armor).sum()
    }

    fn attack<F: Fighter>(&mut self, target: &mut F) {
        let damage = self.damage() - target.armor();
        target.suffer(if damage > 0 { damage } else { 1 });
    }

    fn can_cast<F: Fighter>(&self, target: &F, spell: &Spell) -> bool {
        spell.mana_usage() <= self.mana() &&
            (spell.effect_self.is_none() || !self.has_effect(spell.effect_self.as_ref().unwrap())) &&
            (spell.effect_target.is_none() || !target.has_effect(spell.effect_target.as_ref().unwrap()))
    }

    fn cast<F: Fighter>(&mut self, target: &mut F, spell: &Spell) -> isize {
        if !self.can_cast(target, spell) { return 0; }
        self.suffer(-spell.hp_self);
        target.suffer(-spell.hp_target);
        self.drain(-spell.mp_self);
        target.drain(-spell.mp_target);
        if let Some(ref effect) = spell.effect_self {
            self.add_effect(effect.clone());
        }
        if let Some(ref effect) = spell.effect_target {
            target.add_effect(effect.clone());
        }
        spell.mana_usage()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    hp: isize,
    mp: isize,
    effects: Vec<Effect>,
}

impl Player {
    fn new(hp: isize, mp: isize) -> Player {
        Player { hp: hp, mp: mp, effects: vec![] }
    }
}

impl Fighter for Player {
    fn damage(&self) -> isize { 0 }
    fn health(&self) -> isize { self.hp }
    fn health_mut(&mut self) -> &mut isize { &mut self.hp }
    fn mana(&self) -> isize { self.mp }
    fn mana_mut(&mut self) -> &mut isize { &mut self.mp }
    fn effects(&self) -> &[Effect] { &self.effects }
    fn effects_mut(&mut self) -> &mut Vec<Effect> { &mut self.effects }
}

impl fmt::Display for Player {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("Player({}|{})", self.hp, self.mp))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Monster<'a> {
    name: &'a str,
    hp: isize,
    damage: isize,
    effects: Vec<Effect>,
}

impl<'a> Monster<'a> {
    fn new(name: &str, hp: isize, damage: isize) -> Monster {
        Monster { name: name, hp: hp, damage: damage, effects: vec![] }
    }
}

impl<'a> Fighter for Monster<'a> {
    fn damage(&self) -> isize { self.damage }
    fn health(&self) -> isize { self.hp }
    fn health_mut(&mut self) -> &mut isize { &mut self.hp }
    fn mana(&self) -> isize { 0 }
    fn mana_mut(&mut self) -> &mut isize { unimplemented!() }
    fn effects(&self) -> &[Effect] { &self.effects }
    fn effects_mut(&mut self) -> &mut Vec<Effect> { &mut self.effects }
}

impl<'a> fmt::Display for Monster<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{}({})", self.name, self.hp))
    }
}

fn simulate_fights(player: &Player, monster: &Monster, min_mp: &mut isize, mp: isize) {
    for spell in SPELLS.iter() {
        let mut player = player.clone();
        let mut monster = monster.clone();

        player.apply_effects();
        monster.apply_effects();
        if player.is_dead() { continue; }
        if monster.is_dead() { if mp < *min_mp { *min_mp = mp }; continue; }

        if !player.can_cast(&monster, &spell) { continue; }
        if mp + spell.mana_usage() >= *min_mp { continue; }

        let mp_used = player.cast(&mut monster, spell);
        if player.is_dead() { continue; }
        if monster.is_dead() { if mp + mp_used < *min_mp { *min_mp = mp + mp_used }; continue; }

        player.apply_effects();
        monster.apply_effects();
        if player.is_dead() { continue; }
        if monster.is_dead() { if mp + mp_used < *min_mp { *min_mp = mp + mp_used }; continue; }

        monster.attack(&mut player);
        if player.is_dead() { continue; }
        if monster.is_dead() { if mp + mp_used < *min_mp { *min_mp = mp + mp_used }; continue; }

        simulate_fights(&player, &monster, min_mp, mp + mp_used);
    }
}

fn main() {
    let player = Player::new(50, 500);
    let monster = Monster::new("Boss", 58, 9);
    let mut min_mp = isize::max_value();
    simulate_fights(&player, &monster, &mut min_mp, 0);
    println!("Least amount of mana to spend and win: {}", min_mp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attacking1() {
        let mut player = Player::new(10, 250);
        let mut monster = Monster::new("Boss", 13, 8);
        assert_eq!(player.health(), 10);
        assert_eq!(player.mana(), 250);
        assert_eq!(monster.health(), 13);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(monster.effects(), []);
        player.cast(&mut monster, &SPELLS[3]); // Poison
        assert_eq!(player.health(), 10);
        assert_eq!(player.mana(), 77);
        assert_eq!(monster.health(), 13);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(monster.effects(), [Effect { id: 2, duration: 5, armor: 0, hpt: -3, mpt: 0 }]);
        monster.attack(&mut player);
        assert_eq!(player.health(), 2);
        assert_eq!(player.mana(), 77);
        assert_eq!(monster.health(), 10);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(monster.effects(), [Effect { id: 2, duration: 4, armor: 0, hpt: -3, mpt: 0 }]);
        player.cast(&mut monster, &SPELLS[0]); // Magic Missile
        assert_eq!(player.health(), 2);
        assert_eq!(player.mana(), 24);
        assert_eq!(monster.health(), 3);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(monster.effects(), [Effect { id: 2, duration: 3, armor: 0, hpt: -3, mpt: 0 }]);
        assert_eq!(player.health(), 2);
        assert_eq!(player.mana(), 24);
        assert_eq!(monster.health(), 0);
    }

    #[test]
    fn attacking2() {
        let mut player = Player::new(10, 250);
        let mut monster = Monster::new("Boss", 14, 8);
        assert_eq!(player.health(), 10);
        assert_eq!(player.armor(), 0);
        assert_eq!(player.mana(), 250);
        assert_eq!(monster.health(), 14);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), []);
        assert_eq!(monster.effects(), []);
        player.cast(&mut monster, &SPELLS[4]); // Recharge
        assert_eq!(player.health(), 10);
        assert_eq!(player.armor(), 0);
        assert_eq!(player.mana(), 21);
        assert_eq!(monster.health(), 14);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), [Effect { id: 3, duration: 4, armor: 0, hpt: 0, mpt: 101 }]);
        assert_eq!(monster.effects(), []);
        monster.attack(&mut player);
        assert_eq!(player.health(), 2);
        assert_eq!(player.armor(), 0);
        assert_eq!(player.mana(), 122);
        assert_eq!(monster.health(), 14);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), [Effect { id: 3, duration: 3, armor: 0, hpt: 0, mpt: 101 }]);
        assert_eq!(monster.effects(), []);
        player.cast(&mut monster, &SPELLS[2]); // Shield
        assert_eq!(player.health(), 2);
        assert_eq!(player.armor(), 7);
        assert_eq!(player.mana(), 110);
        assert_eq!(monster.health(), 14);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), [Effect { id: 3, duration: 2, armor: 0, hpt: 0, mpt: 101 }, Effect { id: 1, duration: 5, armor: 7, hpt: 0, mpt: 0 }]);
        assert_eq!(monster.effects(), []);
        monster.attack(&mut player);
        assert_eq!(player.health(), 1);
        assert_eq!(player.armor(), 7);
        assert_eq!(player.mana(), 211);
        assert_eq!(monster.health(), 14);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), [Effect { id: 3, duration: 1, armor: 0, hpt: 0, mpt: 101 }, Effect { id: 1, duration: 4, armor: 7, hpt: 0, mpt: 0 }]);
        assert_eq!(monster.effects(), []);
        player.cast(&mut monster, &SPELLS[1]); // Drain
        assert_eq!(player.health(), 3);
        assert_eq!(player.armor(), 7);
        assert_eq!(player.mana(), 239);
        assert_eq!(monster.health(), 12);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), [Effect { id: 1, duration: 3, armor: 7, hpt: 0, mpt: 0 }]);
        assert_eq!(monster.effects(), []);
        monster.attack(&mut player);
        assert_eq!(player.health(), 2);
        assert_eq!(player.armor(), 7);
        assert_eq!(player.mana(), 340);
        assert_eq!(monster.health(), 12);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), [Effect { id: 1, duration: 2, armor: 7, hpt: 0, mpt: 0 }]);
        assert_eq!(monster.effects(), []);
        player.cast(&mut monster, &SPELLS[3]); // Poison
        assert_eq!(player.health(), 2);
        assert_eq!(player.armor(), 7);
        assert_eq!(player.mana(), 167);
        assert_eq!(monster.health(), 12);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), [Effect { id: 1, duration: 1, armor: 7, hpt: 0, mpt: 0 }]);
        assert_eq!(monster.effects(), [Effect { id: 2, duration: 5, armor: 0, hpt: -3, mpt: 0 }]);
        monster.attack(&mut player);
        assert_eq!(player.health(), 1);
        assert_eq!(player.armor(), 7);
        assert_eq!(player.mana(), 167);
        assert_eq!(monster.health(), 9);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), []);
        assert_eq!(monster.effects(), [Effect { id: 2, duration: 4, armor: 0, hpt: -3, mpt: 0 }]);
        player.cast(&mut monster, &SPELLS[0]); // Magic Missile
        assert_eq!(player.health(), 1);
        assert_eq!(player.armor(), 0);
        assert_eq!(player.mana(), 114);
        assert_eq!(monster.health(), 2);
        player.apply_effects(); monster.apply_effects();
        assert_eq!(player.effects(), []);
        assert_eq!(monster.effects(), [Effect { id: 2, duration: 3, armor: 0, hpt: -3, mpt: 0 }]);
        assert_eq!(player.health(), 1);
        assert_eq!(monster.health(), -1);
    }
}
