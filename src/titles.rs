pub enum TitleLocale {
    RU,
    EN,
}

pub struct Titles {
    pub title: String,
    pub check: String,
    pub turn: String,
}

impl Titles {
    pub fn new(locale: TitleLocale) -> Titles {
        match locale {
            TitleLocale::EN => Titles {
                title: "Chess Game".to_string(),
                check: "Check".to_string(),
                turn: "Turn".to_string(),
            },
            TitleLocale::RU => Titles {
                title: "Шахматы".to_string(),
                check: "Шах".to_string(),
                turn: "Ход".to_string(),
            },
        }
    }
}
