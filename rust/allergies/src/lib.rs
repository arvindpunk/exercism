extern crate num;
#[macro_use]
extern crate num_derive;

pub struct Allergies {
    mask: u32
}

#[derive(Debug, PartialEq, Clone, Copy, FromPrimitive)]
pub enum Allergen {
    Eggs = 1 << 0,
    Peanuts = 1 << 1,
    Shellfish = 1 << 2,
    Strawberries = 1 << 3,
    Tomatoes = 1 << 4,
    Chocolate = 1 << 5,
    Pollen = 1 << 6,
    Cats = 1 << 7
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies{
            mask: score
        }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.mask & (*allergen as u32) != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mask = self.mask;
        let mut allergies = vec![];
        let mut counter = 0;
        while (mask >> counter) != 0 {
            if (mask >> counter)%2 != 0 {
                let possible_alergy = num::FromPrimitive::from_u32(1 << counter);
                match possible_alergy {
                    Some(_) => allergies.push(possible_alergy.unwrap()),
                    None => {},
                }
            }
            counter += 1;
        }
        allergies
    }
}
