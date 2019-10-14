use colored::*;
use std::thread::sleep;
use std::time::Duration;

use crate::inventory::Inventory;
use crate::items::{Item, ItemProperties};
use crate::stats::Stats;

const HUNTABLE_ITEMS: [Item; 1] = [Item {
  id: "rabbit",
  name: "Dead rabbit",
  description: "Poor little thing",
  properties: ItemProperties::StandardItem,
}];

pub fn hunt(inv: &mut Inventory, stats: &mut Stats) {
  let slots_left = inv.slots_left();
  let number_of_items = if slots_left < 3 { slots_left } else { 3 };

  let chosen_weapon = "bow";

  match inv.has_weapon(chosen_weapon) {
    Some(ref mut weapon) => {
      if number_of_items == 0 {
        println!("Your inventory is full. Remove at least one item to proceed.");
      } else {
        println!("{}", "Hunting…".italic().dimmed());
        sleep(Duration::new(4, 0));
        stats.energy.decrease(8.0);
        stats.water.decrease(8.0);
        stats.food.decrease(4.0);

        if rand::random() {
          let broke_down = weapon.decrease_use();

          if broke_down {
            println!("{} {}", chosen_weapon.red(), "broke down".red());
            inv.rm_item(chosen_weapon);
          }

          let item = HUNTABLE_ITEMS.first().unwrap().clone();
          println!("You found {}", item.name.bold());
          inv.add_item(item);
        } else {
          println!("You were unable to track down any animal. Better luck next time!");
        }
      }
    }
    None => println!("You need to craft a weapon first"),
  }
}
