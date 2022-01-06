
pub struct Allergies(u32);

#[derive(Debug, PartialEq, Copy, Clone)]
#[repr(u32)]
pub enum Allergen {
    Eggs = 1,
    Peanuts = 2,
    Shellfish = 4,
    Strawberries = 8,
    Tomatoes = 16,
    Chocolate = 32,
    Pollen = 64,
    Cats = 128,
}

impl Allergen {
    fn all_allergens() -> Vec<Allergen> {
        use Allergen::*;
        vec![Eggs, Peanuts, Shellfish, Strawberries, Tomatoes, Chocolate, Pollen, Cats]
    }
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies ( score )
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        // bitwise and can detect if a given value is a set of another value
        self.0 & *allergen as u32 != 0
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        Allergen::all_allergens().into_iter()
            .filter(|allergen| self.is_allergic_to(&allergen))
            .collect()
    }
}
