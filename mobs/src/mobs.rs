use std::collections::{HashMap, HashSet};
use self::boss::Boss;
use self::member::{Member, Role};

// Declare submodules
pub mod boss;
pub mod member;

#[derive(Debug, PartialEq)]
pub struct Mob {
    name: String,
    boss: Boss,
    members: HashMap<String, Member>,
    cities: HashSet<String>,
    wealth: u64,
}

impl Mob {
    pub fn new(name: &str, boss: Boss) -> Self {
        Mob {
            name: name.to_string(),
            boss,
            members: HashMap::new(),
            cities: HashSet::new(),
            wealth: 0,
        }
    }

    pub fn recruit(&mut self, member_info: (&str, u32)) {
        let (name, age) = member_info;
        let member = Member {
            role: Role::Associate,
            age,
        };
        self.members.insert(name.to_string(), member);
    }

    pub fn attack(&mut self, other: &mut Mob) {
        let self_score = self.calculate_combat_score();
        let other_score = other.calculate_combat_score();

        let self_is_winner = self_score > other_score;
        let (loser, winner) = if self_is_winner {
            (other, self)
        } else {
            (self, other)
        };

        let min_age = loser.members.values().map(|m| m.age).min();
        if let Some(min_age) = min_age {
            let youngest_members: Vec<String> = loser
                .members
                .iter()
                .filter(|(_, m)| m.age == min_age)
                .map(|(name, _)| name.clone())
                .collect();

            for name in youngest_members {
                loser.members.remove(&name);
            }

            if loser.members.is_empty() {
                winner.cities.extend(loser.cities.drain());
                winner.wealth += loser.wealth;
                loser.wealth = 0;
            }
        }
    }

    fn calculate_combat_score(&self) -> u64 {
        self.members.values().map(|m| match m.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }).sum()
    }

    pub fn steal(&mut self, target: &mut Mob, amount: u64) {
        let actual_amount = amount.min(target.wealth);
        target.wealth -= actual_amount;
        self.wealth += actual_amount;
    }

    pub fn conquer_city(&mut self, other_mobs: &[&Mob], city: String) {
        let city_taken = other_mobs.iter().any(|mob| mob.cities.contains(&city));
        if !city_taken {
            self.cities.insert(city);
        }
    }
}