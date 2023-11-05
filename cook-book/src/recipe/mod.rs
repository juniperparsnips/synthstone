use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};

pub use self::{
    shaped_crafting_recipe::ShapedCraftingRecipe,
    shapeless_crafting_recipe::ShapelessCraftingRecipe, smelting_recipe::SmeltingRecipe,
};
use crate::error::CookBookError;

mod shaped_crafting_recipe;
mod shapeless_crafting_recipe;
mod smelting_recipe;

lazy_static! {
    pub static ref FILLER_ID: ItemID = ItemID {
        item: "FILLER_ITEM_CHANGE_ME_LATER".to_string()
    };
    pub static ref FILLER_SLOT: RecipeSlot = RecipeSlot(" ".as_bytes()[0]);
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
/// The non-special automatable types of crafting recipes
/// Smoker and blast furnace have been left out since they're (in vanilla)
/// redundant.
///
/// Special recipe types like
/// * firework rockets
/// * suspicious stew
/// * decorated pots
/// * tipped arrows
/// Have non-standard recipe formats that can't be easily determined by the json
/// i.e. it's a unique type and is likely hardcoded :(
pub enum Recipe {
    #[serde(rename = "minecraft:crafting_shapeless")]
    Shapeless(ShapelessCraftingRecipe),
    #[serde(rename = "minecraft:crafting_shaped")]
    Shaped(ShapedCraftingRecipe),
    #[serde(rename = "minecraft:smelting")]
    Smelting(SmeltingRecipe),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
// #[serde(rename_all = "lowercase")]
#[serde(untagged)]
pub enum RecipeInput {
    Item(ItemID),
    Tag(TagID),
    List(InputOptions),
}

// for mapping tags ands lists to a single item id in storage
impl Into<ItemID> for RecipeInput {
    fn into(self) -> ItemID {
        match self {
            RecipeInput::Item(id) => id,
            // we could technically do load balancing on the storage side by requesting tags...
            // that's gotta be a later feature though. at least on this end it could just be changed by editing the rom
            // so that's what we should probably do
            // Otherwise, we might have manually choose 1 "true" item for each tag
            // i.e. "minecraft:planks" -> "minecraft:oak_planks"
            RecipeInput::Tag(tag) => todo!(),
            // This'll be similar for the worst case scenario for the tag
            // (we choose a default)
            RecipeInput::List(items) => todo!(),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ItemCount(u8);

impl Default for ItemCount {
    fn default() -> Self {
        ItemCount(1)
    }
}

pub trait CraftingRecipe {
    /// The item result from performing this recipe
    fn result(&self) -> &RecipeResult;

    /// The number of filler items needed when crafting this recipe
    fn num_fillers(&self) -> u8;

    /// The list of items needed to perform 1 operation of this recipe excluding fillers
    /// paired with their slot numbers (0 is top left)
    fn input_list(&self) -> Vec<(u8, ItemID)>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Represents a unique ID of an item (i.e. such that it could be requested from a storage system)
pub struct ItemID {
    item: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagID {
    tag: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputOptions(Vec<ItemID>);

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[serde(try_from = "String")]
pub struct RecipeSlot(u8);

impl TryFrom<String> for RecipeSlot {
    type Error = CookBookError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let bytes = value.as_bytes();
        if bytes.len() != 1 {
            return Err(CookBookError::Unknown(format!(
                "Recipe slot must be length exactly 1 ascii character"
            )));
        }

        Ok(RecipeSlot(bytes[0]))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecipeResult {
    // todo.... this is being a problem for the list deserialization
    item: ItemID,
    // maybe item count has to leave an be separate?
    #[serde(default)]
    count: ItemCount,
}
