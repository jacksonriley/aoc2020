use std::collections::{hash_map::Entry, HashMap, HashSet};
use std::time::Instant;

type Ingredient = String;
type Allergen = String;

#[derive(Debug)]
struct Recipe {
    ingredients: HashSet<Ingredient>,
    definite_allergens: Vec<Allergen>,
}

impl Recipe {
    fn from_str(line: &str) -> Self {
        let (ingredients, definite_allergens) = {
            let mut tokens = line.split(" (contains ");
            (
                tokens
                    .next()
                    .unwrap()
                    .split(' ')
                    .map(|s| s.to_string())
                    .collect(),
                tokens
                    .next()
                    .unwrap()
                    .split(", ")
                    .map(|all| all.trim_end_matches(')').to_string())
                    .collect(),
            )
        };
        Self {
            ingredients,
            definite_allergens,
        }
    }
}

fn main() -> Result<(), std::io::Error> {
    let now = Instant::now();
    let input = std::fs::read_to_string("input/21")?;
    let recipes = parse_input(&input);
    let allergenic_ingredients = get_allergenic_ingredients(&recipes);
    println!("Part 1: {}", part_one(&recipes, &allergenic_ingredients));
    println!("Part 2: {}", part_two(&allergenic_ingredients));
    println!("Time: {}µs", now.elapsed().as_micros());
    Ok(())
}

fn parse_input(input: &str) -> Vec<Recipe> {
    input.lines().map(|l| Recipe::from_str(l)).collect()
}

fn get_allergenic_ingredients(recipes: &[Recipe]) -> HashMap<Ingredient, Allergen> {
    // Iterate through the recipes - for each allergen:
    //  * if it has not been seen before, create a HashSet of possible
    //    ingredients that could contain it (only one does).
    //  * if it has been seen before, intersect the set of possible ingredients
    //    with the existing set.
    //  If this results in a set of only one ingredient, then we've solved
    //  that allergen -> ingredient mapping. Remove that ingredient from any
    //  of the other sets of possible ingredients. Repeating this should permit
    //  reducing all sets to length 1, at which point we're done.
    let mut all_to_ing: HashMap<Allergen, HashSet<Ingredient>> = HashMap::new();
    let mut done: HashMap<Ingredient, Allergen> = HashMap::new();

    for recipe in recipes.iter() {
        for allergen in recipe.definite_allergens.iter() {
            match all_to_ing.entry(allergen.to_string()) {
                Entry::Vacant(vacant) => {
                    vacant.insert(recipe.ingredients.clone());
                }
                Entry::Occupied(mut occup) => {
                    occup.insert(
                        occup
                            .get()
                            .intersection(&recipe.ingredients)
                            .cloned()
                            .collect(),
                    );
                    if occup.get().len() == 1 {
                        // We've solved this allergen -> ingredient mapping.
                        // Add to done
                        done.insert(
                            occup.get().iter().next().unwrap().to_string(),
                            allergen.to_string(),
                        );
                    }
                }
            }
        }
    }

    while done.len() < all_to_ing.len() {
        // While not all allergens are mapped
        let mut temp = HashMap::new();
        for done_ingredient in done.keys() {
            for (allergen, ingredients) in all_to_ing.iter_mut() {
                ingredients.remove(done_ingredient);
                if ingredients.len() == 1 {
                    // This is now done - add to done
                    temp.insert(
                        ingredients.iter().next().unwrap().to_string(),
                        allergen.to_string(),
                    );
                }
            }
        }
        for (ing, all) in temp.into_iter() {
            done.insert(ing, all);
        }
    }
    done
}

fn part_one(recipes: &[Recipe], allergenic_ingredients: &HashMap<Ingredient, Allergen>) -> usize {
    recipes
        .iter()
        .map(|recipe| recipe.ingredients.clone())
        .flatten()
        .filter(|i| !allergenic_ingredients.contains_key(i))
        .count()
}

fn part_two(allergenic_ingredients: &HashMap<Ingredient, Allergen>) -> String {
    let mut dangerous_list: Vec<Ingredient> = allergenic_ingredients.keys().cloned().collect();
    dangerous_list.sort_by_key(|ingredient| allergenic_ingredients.get(ingredient).unwrap());
    dangerous_list.join(",")
}

#[test]
fn test_examples() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    let recipes = parse_input(&input);
    let allergenic_ingredients = get_allergenic_ingredients(&recipes);
    assert_eq!(part_one(&recipes, &allergenic_ingredients), 5);
    assert_eq!(part_two(&allergenic_ingredients), "mxmxvkd,sqjhc,fvjkl")
}
