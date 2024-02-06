#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]

enum ShirtColor {
    Red,
    Blue,
}
struct Inventory {
    shirts: Vec<ShirtColor>,
}
impl Inventory {
    fn most_in_stock(&self) -> ShirtColor {
        let mut num_of_blue_shirts = 0;
        let mut num_of_red_shirts = 0;
        for shirt in &self.shirts {
            match shirt {
                ShirtColor::Blue => num_of_blue_shirts += 1,
                ShirtColor::Red => num_of_red_shirts += 1,
            }
        }
        if num_of_blue_shirts > num_of_red_shirts {
            ShirtColor::Blue
        } else {
            ShirtColor::Red
        }
    }

    fn give_away(&self, personal_pref: Option<ShirtColor>) -> ShirtColor {
        personal_pref.unwrap_or_else(|| self.most_in_stock())
    }
}

#[test]
fn test_closures() {
    let store = Inventory {
        shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    };

    let user_pref1 = Some(ShirtColor::Red);
    assert_eq!(store.give_away(user_pref1), ShirtColor::Red);
    assert_eq!(store.give_away(None), ShirtColor::Blue);
}
