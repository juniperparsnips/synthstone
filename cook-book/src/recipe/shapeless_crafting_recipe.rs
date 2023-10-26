use serde::{Deserialize, Serialize};

use super::{CraftingRecipe, ItemID, RecipeInput, RecipeResult};
use crate::error::{CookBookError, CookBookResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapelessCraftingRecipe {
    ingredients: Vec<RecipeInput>,
    result: RecipeResult,
}

impl ShapelessCraftingRecipe {
    pub fn new(ingredients: Vec<RecipeInput>, result: RecipeResult) -> CookBookResult<Self> {
        if ingredients.len() < 1 || ingredients.len() > 9 {
            Err(CookBookError::Unknown(format!(
                "Shapeless recipe must have 1-9 ingredients, has {:}",
                ingredients.len()
            )))
        } else {
            Ok(Self {
                ingredients,
                result,
            })
        }
    }
}

impl CraftingRecipe for ShapelessCraftingRecipe {
    fn result(&self) -> &RecipeResult {
        return &self.result;
    }

    fn input_list(&self) -> Vec<(u8, ItemID)> {
        self.ingredients
            .iter()
            .enumerate()
            .map(|(i, r)| (i as u8, r.clone().into()))
            .collect()
    }

    fn num_fillers(&self) -> u8 {
        return 0;
    }
}
