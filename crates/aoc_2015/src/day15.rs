use aoc_common::{file_lines, IteratorExt};

struct Ingredient {
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

impl Ingredient {
    fn capacity(&self) -> i32 { self.capacity }
    fn durability(&self) -> i32 { self.durability }
    fn flavor(&self) -> i32 { self.flavor }
    fn texture(&self) -> i32 { self.texture }
    fn calories(&self) -> i32 { self.calories }
}

fn input() -> Vec<Ingredient> {
    file_lines("inputs/day15.txt").map(|l| {
        let split = l.split(' ').to_vec();

        Ingredient {
            capacity: split[2].trim_end_matches(',').parse().unwrap(),
            durability: split[4].trim_end_matches(',').parse().unwrap(),
            flavor: split[6].trim_end_matches(',').parse().unwrap(),
            texture: split[8].trim_end_matches(',').parse().unwrap(),
            calories: split[10].parse().unwrap()
        }
    }).to_vec()
}

fn sum_ingredient(recipe: &[i32], ingredients: &[Ingredient], selector: fn(&Ingredient) -> i32) -> i32 {
    recipe.iter().zip(ingredients.iter()).map(|(count, ingredient)| {
        *count * selector(ingredient)
    }).sum::<i32>().max(0)
}

fn run(calorie_filter: fn(&[i32], &[Ingredient]) -> bool) -> i32 {
    let ingredients = input();
    let mut recipe = vec![0; ingredients.len()];

    fn backtrack(recipe: &mut [i32], ingredients: &[Ingredient], idx: usize, total: i32, max: &mut i32, calorie_filter: fn(&[i32], &[Ingredient]) -> bool) {
        if idx == recipe.len() - 1 {
            recipe[idx] = 100 - total;

            if !calorie_filter(&recipe, &ingredients) {
                return;
            }

            let score = 
                sum_ingredient(recipe, ingredients, Ingredient::capacity) *
                sum_ingredient(recipe, ingredients, Ingredient::durability) *
                sum_ingredient(recipe, ingredients, Ingredient::flavor) *
                sum_ingredient(recipe, ingredients, Ingredient::texture);

            if score > *max {
                *max = score;
            }
        } else {
            for i in 0..= 100 - total {
                recipe[idx] = i;
                backtrack(recipe, ingredients, idx + 1, total + i, max, calorie_filter);
            }
        }
    }

    let mut max = 0;
    backtrack(&mut recipe, &ingredients, 0, 0, &mut max, calorie_filter);

    max
}

#[test]
fn part1() {
    fn nop_filter(_: &[i32], _: &[Ingredient]) -> bool {
        true
    }

    let answer = run(nop_filter);
    assert_eq!(answer, 13882464);
}

#[test]
fn part2() {
    fn calorie_filter(recipe: &[i32], ingredients: &[Ingredient]) -> bool {
        sum_ingredient(recipe, ingredients, Ingredient::calories) == 500
    }

    let answer = run(calorie_filter);
    assert_eq!(answer, 11171160);
}