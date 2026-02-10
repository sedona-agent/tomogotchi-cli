/// Hunger decay per tick
const HUNGER_DECAY: u8 = 2;
/// Happiness decay per tick
const HAPPINESS_DECAY: u8 = 1;
/// Hunger restored when fed
const FEED_AMOUNT: u8 = 20;
/// Happiness restored when played with
const PLAY_AMOUNT: u8 = 15;

/// Pet mood based on stats
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Happy,
    Content,
    Sad,
    Miserable,
}

/// The pet with stats and state
pub struct Pet {
    pub name: String,
    pub hunger: u8,
    pub happiness: u8,
}

impl Pet {
    /// Create a new pet with starting stats
    pub fn new(name: String) -> Self {
        Self {
            name,
            hunger: 80,
            happiness: 80,
        }
    }

    /// Tick: decay stats over time
    pub fn tick(&mut self) {
        self.hunger = self.hunger.saturating_sub(HUNGER_DECAY);
        self.happiness = self.happiness.saturating_sub(HAPPINESS_DECAY);
    }

    /// Calculate current mood from stats
    pub fn mood(&self) -> Mood {
        let avg = (self.hunger as u16 + self.happiness as u16) / 2;
        match avg {
            70..=100 => Mood::Happy,
            40..=69 => Mood::Content,
            20..=39 => Mood::Sad,
            _ => Mood::Miserable,
        }
    }

    /// Feed the pet, restoring hunger
    pub fn feed(&mut self) {
        self.hunger = self.hunger.saturating_add(FEED_AMOUNT).min(100);
    }

    /// Play with the pet, restoring happiness
    pub fn play(&mut self) {
        self.happiness = self.happiness.saturating_add(PLAY_AMOUNT).min(100);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_pet_starts_healthy() {
        let pet = Pet::new("Tomo".to_string());
        assert_eq!(pet.hunger, 80);
        assert_eq!(pet.happiness, 80);
        assert_eq!(pet.mood(), Mood::Happy);
    }

    #[test]
    fn test_tick_decays_stats() {
        let mut pet = Pet::new("Tomo".to_string());
        pet.tick();
        assert_eq!(pet.hunger, 78);
        assert_eq!(pet.happiness, 79);
    }

    #[test]
    fn test_tick_clamps_at_zero() {
        let mut pet = Pet::new("Tomo".to_string());
        pet.hunger = 1;
        pet.happiness = 0;
        pet.tick();
        assert_eq!(pet.hunger, 0);
        assert_eq!(pet.happiness, 0);
    }

    #[test]
    fn test_mood_thresholds() {
        let mut pet = Pet::new("Tomo".to_string());
        
        pet.hunger = 80;
        pet.happiness = 80;
        assert_eq!(pet.mood(), Mood::Happy);
        
        pet.hunger = 50;
        pet.happiness = 50;
        assert_eq!(pet.mood(), Mood::Content);
        
        pet.hunger = 30;
        pet.happiness = 30;
        assert_eq!(pet.mood(), Mood::Sad);
        
        pet.hunger = 10;
        pet.happiness = 10;
        assert_eq!(pet.mood(), Mood::Miserable);
    }

    #[test]
    fn test_feed_increases_hunger() {
        let mut pet = Pet::new("Tomo".to_string());
        pet.hunger = 50;
        pet.feed();
        assert_eq!(pet.hunger, 70);
    }

    #[test]
    fn test_feed_caps_at_100() {
        let mut pet = Pet::new("Tomo".to_string());
        pet.hunger = 95;
        pet.feed();
        assert_eq!(pet.hunger, 100);
    }

    #[test]
    fn test_play_increases_happiness() {
        let mut pet = Pet::new("Tomo".to_string());
        pet.happiness = 50;
        pet.play();
        assert_eq!(pet.happiness, 65);
    }

    #[test]
    fn test_play_caps_at_100() {
        let mut pet = Pet::new("Tomo".to_string());
        pet.happiness = 90;
        pet.play();
        assert_eq!(pet.happiness, 100);
    }
}
