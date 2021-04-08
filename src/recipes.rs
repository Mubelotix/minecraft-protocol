use crate::*;

#[derive(Debug, MinecraftPacketPart)]
#[discriminant(VarInt)]
pub enum UnlockRecipesAction<'a> {
    Init {
        crafting_recipe_book_open: bool,
        crafting_recipe_book_filter_active: bool,
        smelting_recipe_book_open: bool,
        smelting_recipe_book_filter_active: bool,
        blast_furnace_recipe_book_open: bool,
        blast_furnace_recipe_book_filter_active: bool,
        smoker_recipe_book_open: bool,
        smoker_recipe_book_filter_active: bool,
        /// Recipes that will be tagged as displayed
        displayed_recipes: Array<'a, Identifier<'a>, VarInt>,
        /// Recipes that will be added to the recipe book
        added_recipes: Array<'a, Identifier<'a>, VarInt>,
    },
    Add {
        crafting_recipe_book_open: bool,
        crafting_recipe_book_filter_active: bool,
        smelting_recipe_book_open: bool,
        smelting_recipe_book_filter_active: bool,
        blast_furnace_recipe_book_open: bool,
        blast_furnace_recipe_book_filter_active: bool,
        smoker_recipe_book_open: bool,
        smoker_recipe_book_filter_active: bool,
        /// Recipes that will be added to the recipe book and have their icon shown in the notification
        recipes: Array<'a, Identifier<'a>, VarInt>,
    },
    Remove {
        crafting_recipe_book_open: bool,
        crafting_recipe_book_filter_active: bool,
        smelting_recipe_book_open: bool,
        smelting_recipe_book_filter_active: bool,
        blast_furnace_recipe_book_open: bool,
        blast_furnace_recipe_book_filter_active: bool,
        smoker_recipe_book_open: bool,
        smoker_recipe_book_filter_active: bool,
        /// Recipes that will be removed
        /// This allows them to be re-displayed when they are re-added.
        recipes: Array<'a, Identifier<'a>, VarInt>,
    },
}
