use crate::items::Item;
use colored::*;
use std::slice::{IterMut, Iter};

pub const DEFAULT_SIZE: usize = 10;

///Represents an inventory that can hold items up to size 
pub struct Inventory {
  ///Amount of items this inventory can hold (can be upgraded)
  size: usize,
  ///The list of items stored in the inventory
  items: Vec<Item>,
}

impl Inventory {
  ///Returns a new Inventory object that has a capacity of the default size
  pub fn new() -> Self {
    Inventory {
      size: DEFAULT_SIZE,
      items: Vec::with_capacity(DEFAULT_SIZE),
    }
  }

  ///Returns true if the inventory is full, otherwise false
  pub fn full(&self) -> bool {
    self.items.len() == self.size
  }

  ///Returns the number of available free slots in the inventory
  pub fn slots_left(&self) -> usize {
    self.size - self.items.len()
  }

  ///Add an item to the inventory only if it is not currently full
  pub fn add_item(&mut self, item: Item) {
    if !self.full() {
      self.items.push(item);
    }
  }

  ///Remove an item from the inventory given an item_id str
  ///item_id str is the name of the item
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

  ///Print the current inventory and the current item count / capacity
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

  ///If the inventory contains a weapon it will return that item
  pub fn has_weapon(&mut self, chosen_weapon: &str) -> Option<&mut Item> {
    self.items.iter_mut().find(|item| item.id == chosen_weapon)
  }

  ///Returns a reference to an item at index
  pub fn get_item(&self, idx: usize) -> &Item {
    &self.items[idx]
  }

  ///Return an iterator over the items
  ///TODO This could possibly be changed to an inventory iter 
  ///(which internally loops over the items)
  pub fn items_iter(&self) -> Iter<Item> {
    self.items.iter()
  }

  ///Returns a mutable iteratory
  ///TODO Should be changed as above
  pub fn items_iter_mut(&mut self) -> IterMut<Item> {
    self.items.iter_mut()
  }

  ///Return a mutable reference to the list of items
  ///FIXME This was a temporary function to get the new Inventory struct to work
  ///with the old code
  pub fn get_items(&mut self) -> &mut Vec<Item> {
    &mut self.items
  }

}
