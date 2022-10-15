use json::object;

pub struct Food {
    _name: String,
    calories: [String; 2],
    proteins: f64,
    fats: f64,
    carbs: f64,
    nbr_of_portions: f64,
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals = 0.0;
    let mut carbs = 0.0;
    let mut proteins = 0.0;
    let mut fats = 0.0;
    for food in foods.iter() {
        cals += food.calories[1]
            .trim_end_matches("kcal")
            .parse::<f64>()
            .unwrap()
            * food.nbr_of_portions;
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
    }
    object! {
        "cals": cals,
        "carbs": carbs,
        "proteins": proteins,
        "fats": fats
    }
}

fn main() {
    let a = vec![
        Food {
            _name: String::from("big mac"),
            calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
            proteins: 27.0,
            fats: 26.0,
            carbs: 41.0,
            nbr_of_portions: 2.0,
        },
        Food {
            _name: "pizza margherita".to_string(),
            calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
            proteins: 13.89,
            fats: 11.21,
            carbs: 49.07,
            nbr_of_portions: 4.9,
        },
    ];

    println!("{:#}", calculate_macros(a));
}

// Food {
//     name: <name>,
//     calories: [<value_in_kJ>, <value_in_kcal>],
//     fats: <fats_in_g>,
//     carbs: <carbs_in_g>,
//     proteins: <proteins_in_g>,
//     nbr_of_portions: <portions>
// }
