use std::fs;

use cook_book::recipe::Recipe;

fn main() {
    let recipe_json = fs::read("assets/recipes/dye_white_bed.json").unwrap();

    let recipe: Recipe = serde_json::from_slice(&recipe_json).unwrap();

    println!("Recipe: {:?}", recipe);
}
