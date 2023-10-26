use std::fs;

use cook_book::recipe::Recipe;

fn main() {
    let recipe_json = fs::read("assets/recipes/magenta_stained_glass.json").unwrap();

    let recipe: Recipe = serde_json::from_slice(&recipe_json).unwrap();

    println!("Recipe: {:?}", recipe);
}
