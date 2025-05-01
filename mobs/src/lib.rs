use std::collections::{HashMap, HashSet};
use boss::Boss;
use member::{Member, Role};

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

        // Determine if self is the winner (self wins if score is higher, loses on tie or lower)
        let self_is_winner = self_score > other_score;

        // Reference the loser and winner without moving
        let (loser, winner) = if self_is_winner {
            (other, self)
        } else {
            (self, other)
        };

        // Find the youngest member(s)
        let min_age = loser.members.values().map(|m| m.age).min();
        if let Some(min_age) = min_age {
            let youngest_members: Vec<String> = loser
                .members
                .iter()
                .filter(|(_, m)| m.age == min_age)
                .map(|(name, _)| name.clone())
                .collect();

            // Remove all youngest members
            for name in youngest_members {
                loser.members.remove(&name);
            }

            // If loser has no members, transfer cities and wealth
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

pub mod boss;
pub mod member;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recruit() {
        let boss = Boss::new("Don", 50);
        let mut mob = Mob::new("Corleone", boss);
        mob.recruit(("Vito", 30));
        assert_eq!(mob.members.len(), 1);
        assert_eq!(mob.members.get("Vito").unwrap().role, Role::Associate);
    }

    #[test]
    fn test_steal() {
        let boss1 = Boss::new("Don1", 50);
        let boss2 = Boss::new("Don2", 50);
        let mut mob1 = Mob::new("Mob1", boss1);
        let mut mob2 = Mob::new("Mob2", boss2);
        mob2.wealth = 100;
        mob1.steal(&mut mob2, 60);
        assert_eq!(mob1.wealth, 60);
        assert_eq!(mob2.wealth, 40);
    }
}