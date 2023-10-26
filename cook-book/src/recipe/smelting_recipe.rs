use serde::{Deserialize, Serialize};

use super::{CraftingRecipe, ItemID, RecipeInput, RecipeResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmeltingRecipe {
    input: RecipeInput,
    #[serde(rename = "cookingtime")]
    cooking_time: usize,
    result: RecipeResult,
}

impl CraftingRecipe for SmeltingRecipe {
    fn result(&self) -> &RecipeResult {
        &self.result
    }

    fn input_list(&self) -> Vec<(u8, ItemID)> {
        vec![(0, self.input.clone().into())]
    }

    fn num_fillers(&self) -> u8 {
        0
    }
}
