struct BattleEntity {
    level: u8,
    attack: u16,
    defense: u16,
    base_power: u16,
}

impl BattleEntity {
    fn new(level: u8, attack: u16, defense: u16, base_power: u16) -> Self {
        Self {
            level,
            attack,
            defense,
            base_power,
        }
    }

    fn calculate_damage(&self, opponent: &BattleEntity) -> u32 {
        (((2 * self.level as u32 / 5 + 2) * self.attack as u32 * self.base_power as u32 / opponent.defense as u32) / 50 + 2)
    }
}
