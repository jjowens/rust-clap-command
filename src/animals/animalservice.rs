use crate::animals::animal::Animal;
use crate::animals::sheep::Sheep;
use crate::animals::cat::Cat;

pub fn process() {
    sheep_process();
    println!("");
    cat_process();
}

fn sheep_process() {
    let mut dolly: Sheep = Animal::new("Dolly");
    // TODO ^ Try removing the type annotations.

    dolly.talk();
    dolly.shear();
    dolly.shear();
    dolly.talk();

    dolly.dances();
    dolly.sleuth();
}

fn cat_process() {
    let mut molly: Cat = Animal::new("Molly");
    // TODO ^ Try removing the type annotations.

    molly.talk();
    molly.shear();
    molly.shear();
    molly.talk();

    molly.dances();
    molly.sleuth();
}