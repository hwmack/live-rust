use crate::items::Item;
use colored::*;
use std::slice::{IterMut, Iter};

pub const DEFAULT_SIZE: usize = 10;

pub struct Inventory {
  size: usize,
  items: Vec<Item>,
}

impl Inventory {
  pub fn new() -> Self {
    Inventory {
      size: DEFAULT_SIZE,
      items: Vec::with_capacity(DEFAULT_SIZE),
    }
  }

  pub fn full(&self) -> bool {
    self.items.len() == self.size
  }

  pub fn slots_left(&self) -> usize {
    self.size - self.items.len()
  }

  pub fn add_item(&mut self, item: Item) {
    if !self.full() {
      self.items.push(item);
    }
  }

  pub fn rm_item(&mut self, item_id: &str) -> bool {
    let item_idx = self.items.iter().position(|item| item.id == item_id);

    match item_idx {
      Some(idx) => {
        self.items.remove(idx);
        println!("{}", "Item removed".green());
        true
      }
      None => {
        println!(
          "{} Type '{}' to list available items.",
          "Item not in inventory.".red(),
          "inventory".bold()
        );
        false
      }
    }
  }

  pub fn print_inventory(&self) {
    println!("Items in your backpack:");
    for item in &self.items {
      println!("{}", item);
    }
    println!();

    let inv_size = self.items.len();
    let inv_size_str = inv_size.to_string();

    let inv_size_str = if inv_size < self.size - 3 {
      inv_size_str.green()
    } else if inv_size < self.size {
      inv_size_str.yellow()
    } else {
      inv_size_str.red()
    };
    println!("{}/{}", inv_size_str, self.size);
  }

  pub fn has_weapon(&mut self, chosen_weapon: &str) -> Option<&mut Item> {
    self.items.iter_mut().find(|item| item.id == chosen_weapon)
  }

  pub fn get_item(&self, idx: usize) -> &Item {
    &self.items[idx]
  }

  pub fn items_iter(&self) -> Iter<Item> {
    self.items.iter()
  }

  pub fn items_iter_mut(&mut self) -> IterMut<Item> {
    self.items.iter_mut()
  }

  pub fn get_items(&mut self) -> &mut Vec<Item> {
    &mut self.items
  }

}
