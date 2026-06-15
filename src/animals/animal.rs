pub trait Animal {
    // Associated function signature; `Self` refers to the implementor type.
    fn new(name: &'static str) -> Self;

    // Method signatures; these will return a string.
    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    // Traits can provide default method definitions.
    fn talk(&self) {
        println!("{} says {}", self.name(), self.noise());
    }

    fn dances(&self) {
        println!("{} starts dancing to {} music", self.name(), "rock");
    }

    fn sleuth(&self) {
        println!("{} starts sleuthing", self.name());
    }
}

// pub fn process() {
//     // Type annotation is necessary in this case.
//     let mut dolly: Sheep = Animal::new("Dolly");
//     // TODO ^ Try removing the type annotations.
//
//     dolly.talk();
//     dolly.shear();
//     dolly.shear();
//     dolly.talk();
//
//     dolly.dances();
//     dolly.sleuth();
// }