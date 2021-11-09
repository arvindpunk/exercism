// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub struct Player {
    pub health: u32,
    pub mana: Option<u32>,
    pub level: u32,
}

impl Player {
    pub fn revive(&self) -> Option<Player> {
        match self.health {
            0 => Some(Player {
                health: 100,
                level: self.level,
                mana: {
                    if self.level < 10 {
                        None
                    } else {
                        Some(100)
                    }
                },
            }),
            _ => None,
        }
    }

    pub fn cast_spell(&mut self, mana_cost: u32) -> u32 {
        if self.level < 10 {
            self.health -= u32::min(self.health, mana_cost);
            0
        } else {
            if self.mana > Some(mana_cost) {
                self.mana = Some(self.mana.unwrap() - mana_cost);
                mana_cost * 2
            } else {
                0
            }
        }
    }
}
