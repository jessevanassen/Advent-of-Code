import { log } from '../../lib';
import { count, pipe, pipeline, sort } from '../../lib/fp';
import { collectToArray, contains, filter, flatMap, join, map } from '../../lib/fp/generators';
import { readBlocksFromStdin } from '../../lib/fs';

const foods = pipe(
	map(parseLine),
	collectToArray,
)(readBlocksFromStdin());

const allIngredients = new Set(pipeline(foods, flatMap(food => food.ingredients)));
const allAllergens = new Set(pipeline(foods, flatMap(food => food.allergens)));

const allergensToIngredients = matchAllergensToIngredients();
const ingredientsWithAllergens = new Set(allergensToIngredients.values());
const ingredientsWithoutAllergens = new Set(pipeline(allIngredients, filter(ingredient => !ingredientsWithAllergens.has(ingredient))));

pipeline(foods,
	flatMap(foods => foods.ingredients),
	filter(food => ingredientsWithoutAllergens.has(food)),
	count,
	log('Part 1:'),
);

pipeline(
	allergensToIngredients,
	sort(([allergen1], [allergen2]) => allergen1.localeCompare(allergen2)),
	map(([, ingredient]) => ingredient),
	join(','),
	log('Part 2'),
);

function matchAllergensToIngredients(): Map<string, string> {
	const possibleIngredientsWithAllergen = new Map<string, Set<string>>();

	for (const food of foods) {
		for (const allergen of food.allergens) {
			const possibleIngredients = possibleIngredientsWithAllergen.get(allergen);
			if (!possibleIngredients) {
				possibleIngredientsWithAllergen.set(allergen, new Set(food.ingredients));
				continue;
			}

			for (const ingredient of possibleIngredients) {
				if (!food.ingredients.includes(ingredient)) {
					possibleIngredients.delete(ingredient);
				}
			}
		}
	}

	const allergenToIngredient = new Map<string, string>();
	while (allergenToIngredient.size !== allAllergens.size) {
		for (const [allergen, ingredients] of possibleIngredientsWithAllergen) {
			if (allergenToIngredient.has(allergen)) {
				continue;
			}

			const possibleIngredients = pipeline(
				ingredients,
				filter(ingredient => !contains(ingredient)(allergenToIngredient.values())),
				collectToArray,
			);

			if (possibleIngredients.length === 1) {
				allergenToIngredient.set(allergen, possibleIngredients[0]);
			}
		}
	}

	return allergenToIngredient;
}

function parseLine(line: string): { ingredients: string[]; allergens: string[]; } {
	const [ingredients, allergens] = line.slice(0, -1).split(' (contains ', 2);
	return {
		ingredients: ingredients.split(' '),
		allergens: allergens.split(', '),
	};
}
