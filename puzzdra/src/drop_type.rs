pub const DROP_MAX: usize = 5;

#[derive(Eq, PartialEq)]
pub enum DropType {
    NONE,
    FIRE,
    WATER,
    WOOD,
    LIGHT,
    DARK,
}

impl DropType {
    //アスキーアートに変換
    pub fn to_aa(&self) -> String {
        match *self {
            DropType::NONE => String::from("　"),
            DropType::FIRE => String::from("▽"),
            DropType::WATER => String::from("△"),
            DropType::WOOD => String::from("◻"),
            DropType::LIGHT => String::from("◯"),
            DropType::DARK => String::from("☆"),
        }
    }

    //選択時のアスキーアートに変換
    pub fn to_aa_selected(&self) -> String {
        match *self {
            DropType::NONE => String::from("　"),
            DropType::FIRE => String::from("▼"),
            DropType::WATER => String::from("▲"),
            DropType::WOOD => String::from("◼"),
            DropType::LIGHT => String::from("◉"),
            DropType::DARK => String::from("★"),
        }
    }

    //数値からの変換
    pub fn from(n: usize) -> Self {
        match n {
            1 => DropType::FIRE,
            2 => DropType::WATER,
            3 => DropType::WOOD,
            4 => DropType::LIGHT,
            5 => DropType::DARK,
            _ => DropType::NONE
        }
    }
}
