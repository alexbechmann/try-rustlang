use std::error::Error;

use syrette::injectable;
use syrette::ptr::TransientPtr;
use syrette::DIContainer;

trait IWeapon {
    fn deal_damage(&self, damage: i32);
}

struct Sword {}

#[injectable(IWeapon)]
impl Sword {
    fn new() -> Self {
        Self {}
    }
}

impl IWeapon for Sword {
    fn deal_damage(&self, damage: i32) {
        println!("Sword dealt {} damage!", damage);
    }
}

trait IWarrior {
    fn fight(&self);

    fn level_up(&mut self);

    // async fn travel(&self);
    fn get_level(&self) -> i32;
}

struct Warrior {
    weapon: TransientPtr<dyn IWeapon>,
    level: i32,
}

#[injectable(IWarrior)]
impl Warrior {
    fn new(weapon: TransientPtr<dyn IWeapon>) -> Self {
        Self { weapon, level: 1 }
    }
}

impl IWarrior for Warrior {
    fn fight(&self) {
        self.weapon.deal_damage(30);
    }

    fn level_up(&mut self) {
        self.level += 1;
    }

    fn get_level(&self) -> i32 {
        self.level
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut container = DIContainer::new();
    container.bind::<dyn IWeapon>().to::<Sword>()?;
    container.bind::<dyn IWarrior>().to::<Warrior>()?;

    let mut warrior = container.get::<dyn IWarrior>()?.transient()?;

    warrior.fight();
    warrior.level_up();

    println!("Warrior has fighted");
    println!("Warrior level: {}", warrior.get_level());

    Ok(())
}
