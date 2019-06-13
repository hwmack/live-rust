use colored::*;
use std::fmt;

#[derive(Debug)]
pub struct Recipe {
  pub id: &'static str,
  pub name: &'static str,
  pub description: &'static str,
  pub items_needed: Vec<(&'static str, usize)>,
  pub tools_needed: Vec<&'static str>,
  pub upgrades_needed: Vec<&'static str>,
  pub result: Vec<&'static str>,
  pub category: RecipeCategory,
}

impl fmt::Display for Recipe {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name.bold())
  }
}

#[derive(Debug)]
pub enum RecipeCategory {
  Consumable,
  Tool,
  Weapon,
  Other,
}

pub fn recipes() -> [Recipe; 8] {
  return [
    Recipe {
      id: "rope",
      name: "Rope",
      description: "",
      items_needed: vec![("string", 2)],
      tools_needed: Vec::new(),
      upgrades_needed: Vec::new(),
      result: vec!["rope"],
      category: RecipeCategory::Tool,
    },
    Recipe {
      id: "bow",
      name: "Bow",
      description: "",
      items_needed: vec![("string", 1), ("wood", 1)],
      tools_needed: vec![],
      upgrades_needed: vec![],
      result: vec!["bow"],
      category: RecipeCategory::Weapon,
    },
    Recipe {
      id: "knife",
      name: "Knife",
      description: "",
      items_needed: vec![("flint", 1), ("wood", 1), ("rope", 1)],
      tools_needed: vec![],
      upgrades_needed: vec![],
      result: vec!["knife"],
      category: RecipeCategory::Tool,
    },
    Recipe {
      id: "jerky",
      name: "Jerky",
      description: "",
      items_needed: vec![("meat", 1), ("salt", 1)],
      tools_needed: vec![],
      upgrades_needed: vec![],
      result: vec!["jerky"],
      category: RecipeCategory::Consumable,
    },
    Recipe {
      id: "medicinal tea",
      name: "Medicinal tea",
      description: "",
      items_needed: vec![("clean water", 1), ("medicinal herbs", 1)],
      tools_needed: vec![],
      upgrades_needed: vec![],
      result: vec!["medicinal tea"],
      category: RecipeCategory::Consumable,
    },
    Recipe {
      id: "cooked meat",
      name: "Cooked meat",
      description: "",
      items_needed: vec![("meat", 1)],
      tools_needed: vec![],
      upgrades_needed: vec!["fire"],
      result: vec!["cooked meat"],
      category: RecipeCategory::Consumable,
    },
    Recipe {
      id: "clean water",
      name: "Clean water",
      description: "",
      items_needed: vec![("dirty water", 1)],
      tools_needed: vec![],
      upgrades_needed: vec!["fire"],
      result: vec!["clean water"],
      category: RecipeCategory::Consumable,
    },
    Recipe {
      id: "skinned rabbit",
      name: "Skinned rabbit",
      description: "Obtain meat and pelt",
      items_needed: vec![("rabbit", 1)],
      tools_needed: vec!["knife"],
      upgrades_needed: vec![],
      result: vec!["meat", "rabbit pelt"],
      category: RecipeCategory::Other,
    },
  ];
}

pub fn print_recipes() {
  let recipes = recipes();

  for recipe in &recipes {
    println!("{} - Items needed:", recipe);
    for item in &recipe.items_needed {
      println!(
        "\t{} {}",
        item.1.to_string().dimmed(),
        item.0.bold().dimmed(),
      )
    }
  }
}
