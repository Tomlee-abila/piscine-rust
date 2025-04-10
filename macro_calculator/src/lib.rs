use json::JsonValue;

pub struct Food {
    pub name: String,
    pub calories: [String; 2], 
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64,
}

fn smart_round(value: f64) -> f64 {
    let rounded = (value * 100.0).round() / 100.0;
    if (rounded * 10.0).fract() == 0.0 {
        (rounded * 10.0).round() / 10.0
    } else {
        rounded
    }
}

pub fn calculate_macros(foods: Vec<Food>) -> JsonValue {
    let mut total_cals = 0.0;
    let mut total_fats = 0.0;
    let mut total_carbs = 0.0;
    let mut total_proteins = 0.0;

    for food in foods {
        let kcal_str = &food.calories[1]; 
        let kcal: f64 = kcal_str
            .replace("kcal", "")
            .trim()
            .parse()
            .unwrap_or(0.0);

        let portions = food.nbr_of_portions;

        total_cals += kcal * portions;
        total_fats += food.fats * portions;
        total_carbs += food.carbs * portions;
        total_proteins += food.proteins * portions;
    }

    json::object! {
        "cals": smart_round(total_cals),
        "carbs": smart_round(total_carbs),
        "proteins": smart_round(total_proteins),
        "fats": smart_round(total_fats)
    }
}
