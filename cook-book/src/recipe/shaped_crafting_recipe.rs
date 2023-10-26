use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{CraftingRecipe, ItemID, RecipeInput, RecipeResult, RecipeSlot, FILLER_SLOT};
use crate::error::{CookBookError, CookBookResult};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapedCraftingRecipe {
    #[serde(rename = "key")]
    slots: HashMap<RecipeSlot, RecipeInput>,
    pattern: CraftingPattern,
    result: RecipeResult,
}

impl ShapedCraftingRecipe {
    pub fn new(
        slots: HashMap<RecipeSlot, RecipeInput>,
        pattern: CraftingPattern,
        result: RecipeResult,
    ) -> CookBookResult<Self> {
        if slots.len() < 1 || slots.len() > 9 {
            return Err(CookBookError::Unknown(format!(
                "Shapeless recipe must have 1-9 slots, has {:}",
                slots.len()
            )));
        }

        Ok(Self {
            slots,
            pattern,
            result,
        })
    }
}

impl CraftingRecipe for ShapedCraftingRecipe {
    fn result(&self) -> &RecipeResult {
        return &self.result;
    }

    fn input_list(&self) -> Vec<(u8, ItemID)> {
        let mut list = Vec::with_capacity(9);

        for (i, slot) in self.pattern.0.iter().flatten().enumerate() {
            if *slot != *FILLER_SLOT {
                list.push((i as u8, self.slots.get(&slot).unwrap().clone().into()));
            }
        }

        list
    }

    fn num_fillers(&self) -> u8 {
        let mut fillers: u8 = 0;

        for (i, row) in self.pattern.0.iter().enumerate() {
            for slot in row {
                if *slot == *FILLER_SLOT {
                    fillers += 1;
                }
            }
            if i != self.pattern.0.len() {
                fillers += 3 - row.len() as u8;
            }
        }

        fillers
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(try_from = "Vec<String>")]
pub struct CraftingPattern(Vec<Vec<RecipeSlot>>);

impl CraftingPattern {
    pub fn new(pattern: Vec<Vec<RecipeSlot>>) -> CookBookResult<Self> {
        if pattern.len() < 1 || pattern.len() > 3 {
            return Err(CookBookError::Unknown(format!(
                "Recipe pattern must have 1-3 rows"
            )));
        }

        for row in &pattern {
            if row.len() < 1 || row.len() > 3 {
                return Err(CookBookError::Unknown(format!(
                    "Recipe pattern row must have 1 to 3 columns"
                )));
            }
        }

        Ok(CraftingPattern(pattern))
    }
}

impl TryFrom<Vec<String>> for CraftingPattern {
    type Error = CookBookError;

    fn try_from(value: Vec<String>) -> Result<Self, Self::Error> {
        let as_slots = value
            .iter()
            .map(|row| row.bytes().map(|b| RecipeSlot(b)).collect())
            .collect();

        Ok(CraftingPattern::new(as_slots)?)
    }
}
