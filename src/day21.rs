use std::fmt;

pub trait Fighter: Sized {
    fn damage(&self) -> usize;
    fn armor(&self) -> usize;
    fn health(&self) -> usize;
    fn mut_health(&mut self) -> &mut usize;

    fn is_dead(&self) -> bool {
        self.health() == 0
    }

    fn attack<F: Fighter>(&self, other: &mut F) {
        let damage = if other.armor() < self.damage() {
            self.damage() - other.armor()
        } else {
            1
        };
        *other.mut_health() = if other.health() > damage {
            other.health() - damage
        } else {
            0
        };
    }

    fn fight<F: Fighter>(&mut self, other: &mut F) {
        loop {
            self.attack(other);
            if self.is_dead() || other.is_dead() { break; }
            other.attack(self);
            if self.is_dead() || other.is_dead() { break; }
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Item<'a> {
    name: &'a str,
    cost: usize,
    damage: usize,
    armor: usize,
}

impl<'a> fmt::Display for Item<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{} ({}/{})", self.name, self.damage, self.armor))
    }
}

const WEAPONS: [Item<'static>; 5] = [
    Item { name: "Dagger",      cost:   8, damage: 4, armor: 0 },
    Item { name: "Shortsword",  cost:  10, damage: 5, armor: 0 },
    Item { name: "Warhammer",   cost:  25, damage: 6, armor: 0 },
    Item { name: "Longsword",   cost:  40, damage: 7, armor: 0 },
    Item { name: "Greataxe",    cost:  74, damage: 8, armor: 0 },
];

const ARMORS: [Item<'static>; 5] = [
    Item { name: "Leather",     cost:  13, damage: 0, armor: 1 },
    Item { name: "Chainmail",   cost:  31, damage: 0, armor: 2 },
    Item { name: "Splintmail",  cost:  53, damage: 0, armor: 3 },
    Item { name: "Bandedmail",  cost:  75, damage: 0, armor: 4 },
    Item { name: "Platemail",   cost: 102, damage: 0, armor: 5 },
];

const RINGS: [Item<'static>; 6] = [
    Item { name: "Damage +1",   cost:  25, damage: 1, armor: 0 },
    Item { name: "Damage +2",   cost:  50, damage: 2, armor: 0 },
    Item { name: "Damage +3",   cost: 100, damage: 3, armor: 0 },
    Item { name: "Defense +1",  cost:  20, damage: 0, armor: 1 },
    Item { name: "Defense +2",  cost:  40, damage: 0, armor: 2 },
    Item { name: "Defense +3",  cost:  80, damage: 0, armor: 3 },
];

#[derive(Debug, PartialEq, Eq)]
pub struct Player<'a> {
    weapon: &'a Item<'a>,
    armor: Option<&'a Item<'a>>,
    ring1: Option<&'a Item<'a>>,
    ring2: Option<&'a Item<'a>>,
    hp: usize,
}

impl<'a> fmt::Display for Player<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(f.write_str("Player (weapon: "));
        try!(self.weapon.fmt(f));
        try!(f.write_str(", armor: "));
        try!(match self.armor { Some(item) => item.fmt(f), None => f.write_str("-") } );
        try!(f.write_str(", ring1: "));
        try!(match self.ring1 { Some(item) => item.fmt(f), None => f.write_str("-") } );
        try!(f.write_str(", ring2: "));
        try!(match self.ring2 { Some(item) => item.fmt(f), None => f.write_str("-") } );
        try!(f.write_str(")"));
        Ok(())
    }
}

impl<'a> Player<'a> {
    fn value(&self) -> usize {
        self.weapon.cost +
            self.armor.map_or(0, |item| item.cost) +
            self.ring1.map_or(0, |item| item.cost) +
            self.ring2.map_or(0, |item| item.cost)
    }
}

impl<'a> Fighter for Player<'a> {
    fn damage(&self) -> usize {
        self.weapon.damage +
            self.armor.map_or(0, |item| item.damage) +
            self.ring1.map_or(0, |item| item.damage) +
            self.ring2.map_or(0, |item| item.damage)
    }

    fn armor(&self) -> usize {
        self.weapon.armor +
            self.armor.map_or(0, |item| item.armor) +
            self.ring1.map_or(0, |item| item.armor) +
            self.ring2.map_or(0, |item| item.armor)
    }

    fn health(&self) -> usize { self.hp }
    fn mut_health(&mut self) -> &mut usize { &mut self.hp }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Monster {
    hp: usize,
    damage: usize,
    armor: usize,
}

impl Fighter for Monster {
    fn damage(&self) -> usize { self.damage }
    fn armor(&self) -> usize { self.armor }
    fn health(&self) -> usize { self.hp }
    fn mut_health(&mut self) -> &mut usize { &mut self.hp }
}

fn simulate_fights<F: FnMut(&Player, &Monster)>(mut f: F) {
    for weapon in WEAPONS.iter() {
        for armor in vec![None].into_iter().chain(ARMORS.iter().map(|item| Some(item))) {
            for ring1 in vec![None].into_iter().chain(RINGS.iter().map(|item| Some(item))) {
                for ring2 in vec![None].into_iter().chain(RINGS.iter().map(|item| Some(item))) {
                    if Some(ring1) != Some(ring2) {
                        let mut player = Player {
                            weapon: weapon,
                            armor: armor,
                            ring1: ring1,
                            ring2: ring2,
                            hp: 100,
                        };
                        let mut monster = Monster { hp: 103, damage: 9, armor: 2 };
                        player.fight(&mut monster);
                        f(&player, &monster);
                    }
                }
            }
        }
    }
}

fn main() {
    let mut min_price = usize::max_value();
    simulate_fights(|player, monster| {
        if !player.is_dead() && monster.is_dead() && player.value() < min_price {
            min_price = player.value();
        }
    });
    println!("Least amount of gold to spend and win: {}", min_price);

    let mut max_price = 0;
    simulate_fights(|player, monster| {
        if player.is_dead() && !monster.is_dead() && player.value() > max_price {
            max_price = player.value();
        }
    });
    println!("Most amount of gold to spend and lose: {}", max_price);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn attacking() {
        let mut me = Monster { hp: 8, damage: 5, armor: 5 };
        let mut boss = Monster { hp: 12, damage: 7, armor: 2 };
        me.attack(&mut boss);
        assert_eq!(boss.health(), 9);
        boss.attack(&mut me);
        assert_eq!(me.health(), 6);
        me.attack(&mut boss);
        assert_eq!(boss.health(), 6);
        boss.attack(&mut me);
        assert_eq!(me.health(), 4);
        me.attack(&mut boss);
        assert_eq!(boss.health(), 3);
        boss.attack(&mut me);
        assert_eq!(me.health(), 2);
        me.attack(&mut boss);
        assert_eq!(boss.health(), 0);
    }

    #[test]
    fn fighting() {
        let mut me = Monster { hp: 8, damage: 5, armor: 5 };
        let mut boss = Monster { hp: 12, damage: 7, armor: 2 };
        me.fight(&mut boss);
        assert_eq!(me.health(), 2);
        assert!(!me.is_dead());
        assert_eq!(boss.health(), 0);
        assert!(boss.is_dead());
    }
}
