// mod.rs
pub mod boss;
pub mod member;

use member::{Member, Role};
use boss::Boss;

#[derive(Debug, Clone, PartialEq)]
pub struct Mob {
    pub name: String,
    pub boss: Boss,
    pub members: Vec<Member>,
    pub cities: Vec<(String, u8)>,
    pub wealth: u32,
}

impl Mob {
    // Associated function to recruit a new member
    pub fn recruit(&mut self, name: &str, age: u8) {
        let new_member = Member::new(name, Role::Associate, age);
        self.members.push(new_member);
    }

    // Associated function to attack another mob
    pub fn attack(&mut self, other: &mut Mob) {
        let self_power = self.calculate_power();
        let other_power = other.calculate_power();

        // Compare power combat scores and remove a member from the mob with the lower score
        if self_power < other_power {
            if !self.members.is_empty() {
                self.members.pop();
            }
            if other.members.is_empty() {
                self.wealth += other.wealth;
                self.cities.extend(other.cities.clone());
            }
        } else if other_power < self_power {
            if !other.members.is_empty() {
                other.members.pop();
            }
            if self.members.is_empty() {
                other.wealth += self.wealth;
                other.cities.extend(self.cities.clone());
            }
        }
    }

    // Associated function to steal wealth from another mob
    pub fn steal(&mut self, other: &mut Mob, amount: u32) {
        let stolen = std::cmp::min(amount, other.wealth);
        self.wealth += stolen;
        other.wealth -= stolen;
    }

    // Associated function to conquer a city
    pub fn conquer_city(&mut self, mobs: &Vec<Mob>, city_name: &str, city_value: u8) {
        if mobs.iter().all(|mob| !mob.cities.iter().any(|(name, _)| name == city_name)) {
            self.cities.push((city_name.to_string(), city_value));
        }
    }

    // Function to calculate power score of the mob
    fn calculate_power(&self) -> u32 {
        self.members.iter().map(|m| match m.role {
            Role::Underboss => 4,
            Role::Caporegime => 3,
            Role::Soldier => 2,
            Role::Associate => 1,
        }).sum()
    }
}
