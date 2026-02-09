use std::collections::{BTreeSet, HashMap};

#[derive(Default)]
struct FoodRatings {
    food_to_cuisine: HashMap<String, String>,
    food_to_rating: HashMap<String, i32>,
    cuisine_to_foods: HashMap<String, BTreeSet<(i32, String)>>,
}

impl FoodRatings {
    pub fn new(foods: Vec<String>, cuisines: Vec<String>, ratings: Vec<i32>) -> Self {
        let mut fr = FoodRatings::default();
        for i in 0..foods.len() {
            let food = foods[i].clone();
            let cuisine = cuisines[i].clone();
            let rating = ratings[i];

            fr.food_to_cuisine.insert(food.clone(), cuisine.clone());
            fr.food_to_rating.insert(food.clone(), rating);

            fr.cuisine_to_foods
                .entry(cuisine)
                .or_insert_with(BTreeSet::new)
                .insert((-rating, food));
        }
        fr
    }

    pub fn change_rating(&mut self, food: String, new_rating: i32) {
        let cuisine = self.food_to_cuisine.get(&food).unwrap().clone();
        let old_rating = *self.food_to_rating.get(&food).unwrap();

        if let Some(set) = self.cuisine_to_foods.get_mut(&cuisine) {
            set.remove(&(-old_rating, food.clone()));
            set.insert((-new_rating, food.clone()));
        }

        self.food_to_rating.insert(food, new_rating);
    }

    pub fn highest_rated(&self, cuisine: String) -> String {
        let set = self.cuisine_to_foods.get(&cuisine).unwrap();
        let (_, food) = set.iter().next().unwrap();
        food.clone()
    }
}

fn main() {
    let foods = vec!["kimchi", "miso", "sushi", "moussaka", "ramen", "bulgogi"]
        .into_iter()
        .map(String::from)
        .collect();
    let cuisines = vec!["korean", "japanese", "japanese", "greek", "japanese", "korean"]
        .into_iter()
        .map(String::from)
        .collect();
    let ratings = vec![9, 12, 8, 15, 14, 7];

    let mut fr = FoodRatings::new(foods, cuisines, ratings);
    assert_eq!(fr.highest_rated("korean".to_string()), "kimchi");
    assert_eq!(fr.highest_rated("japanese".to_string()), "ramen");

    fr.change_rating("sushi".to_string(), 16);
    assert_eq!(fr.highest_rated("japanese".to_string()), "sushi");

    fr.change_rating("ramen".to_string(), 16);
    assert_eq!(fr.highest_rated("japanese".to_string()), "ramen");

    println!("ok");
}
