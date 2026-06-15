use crate::animals::animal::Animal;

pub struct Cat { naked: bool, name: &'static str }

impl Cat {
    fn is_naked(&self) -> bool {
        self.naked
    }

    pub(crate) fn shear(&mut self) {
        if self.is_naked() {
            // Implementor methods can use the implementor's trait methods.
            println!("{} is already naked...", self.name());
        } else {
            println!("{} gets a haircut!", self.name);

            self.naked = true;
        }
    }
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Cat {
    // `Self` is the implementor type: `Sheep`.
    fn new(name: &'static str) -> Cat {
        Cat { name: name, naked: false }
    }

    fn name(&self) -> &'static str {
        self.name
    }

    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "meooow? Hooman, you will suffer your fate"
        } else {
            "meooow!"
        }
    }

    // Default trait methods can be overridden.
    fn talk(&self) {
        // For example, we can add some quiet contemplation.
        println!("{} pauses briefly... {}", self.name, self.noise());
    }

}