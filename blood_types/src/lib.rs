#[derive(Debug, PartialEq, Eq, Clone, PartialOrd, Ord)]
pub enum Antigen {
	A,
	AB,
	B,
	O,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum RhFactor {
	Positive,
	Negative,
}

#[derive(PartialEq, Eq, PartialOrd)]
pub struct BloodType {
	pub antigen: Antigen,
	pub rh_factor: RhFactor,
}

use std::cmp::{Ord, Ordering};

use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub struct ParseAntigenError;

impl FromStr for Antigen {
    type Err = ParseAntigenError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let antigen = s.trim().chars().filter(|&ch| ch != '-' && ch != '+').collect::<String>();
        
        match antigen.as_str(){
            "A" => Ok(Antigen::A),
            "B" => Ok(Antigen::B),
            "AB" => Ok(Antigen::AB),
            "O" => Ok(Antigen::O),
            _ => Err(ParseAntigenError)
        }
    }
}

#[derive(Debug)]
pub struct ParseRhFactorError;

impl FromStr for RhFactor {
    type Err = ParseRhFactorError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rh_factor = s.trim().chars().filter(|&ch| ch == '-' || ch == '+').collect::<String>();
        
        match rh_factor.as_str(){
            "-" => Ok(RhFactor::Negative),
            "+" => Ok(RhFactor::Positive),
            _ => Err(ParseRhFactorError)
        }
    }
}

impl Ord for BloodType {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.antigen.cmp(&other.antigen) {
            Ordering::Equal => self.rh_factor.cmp(&other.rh_factor),
            other => other,
        }
    }
}

#[derive(Debug)]
pub struct ParseBloodTypeError;

impl FromStr for BloodType {
    type Err = ParseBloodTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rh_factor = RhFactor::from_str(s);
        let antigen = Antigen::from_str(s);

        match (antigen, rh_factor){
            (Ok(an), Ok(rh)) => Ok(BloodType{
                antigen: an,
                rh_factor: rh
            }),
            _ => Err(ParseBloodTypeError)
        }       
        
    }
}

use std::fmt::{self, Debug};

impl Debug for BloodType {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let antigen = match self.antigen {
			Antigen::A => "A",
			Antigen::B => "B",
			Antigen::AB => "AB",
			Antigen::O => "O",
		};
		let rh = match self.rh_factor {
			RhFactor::Positive => "+",
			RhFactor::Negative => "-",
		};
		write!(f, "{}{}", antigen, rh)
	}
}
impl BloodType {
	pub fn can_receive_from(&self, other: &Self) -> bool {
		let antigen_ok = match self.antigen {
			Antigen::AB => true,
			Antigen::A => matches!(other.antigen, Antigen::A | Antigen::O),
			Antigen::B => matches!(other.antigen, Antigen::B | Antigen::O),
			Antigen::O => matches!(other.antigen, Antigen::O),
		};

		let rh_ok = match self.rh_factor {
			RhFactor::Positive => true,
			RhFactor::Negative => matches!(other.rh_factor, RhFactor::Negative),
		};

		antigen_ok && rh_ok
	}

	pub fn donors(&self) -> Vec<Self> {
		Self::all().into_iter().filter(|b| self.can_receive_from(b)).collect()
	}

	pub fn recipients(&self) -> Vec<BloodType> {
		Self::all().into_iter().filter(|b| b.can_receive_from(self)).collect()
	}

	fn all() -> Vec<BloodType> {
		let mut all = Vec::new();
		let antigens = [Antigen::A, Antigen::B, Antigen::AB, Antigen::O];
		let rh_factors = [RhFactor::Positive, RhFactor::Negative];
		for a in antigens.iter() {
			for r in rh_factors.iter() {
				all.push(BloodType {
					antigen: a.clone(),
					rh_factor: r.clone(),
				});
			}
		}
		all
	}
}