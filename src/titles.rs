pub enum TitleLocale {
    RU,
    EN,
}

pub struct Titles {
    pub title: String,
    pub check: String,
    pub mate: String,
    pub turn: String,
    pub button_new_game: String,
    pub button_continue_game: String,
    pub button_exit_game: String,
}

impl Titles {
    pub fn new(locale: TitleLocale) -> Titles {
        match locale {
            TitleLocale::EN => Titles {
                title: "Chess Game".to_string(),
                check: "Check".to_string(),
                mate: "Mate".to_string(),
                turn: "Turn".to_string(),
                button_new_game: "New Game".to_string(),
                button_continue_game: "Continue".to_string(),
                button_exit_game: "Exit".to_string(),
            },
            TitleLocale::RU => Titles {
                title: "Шахматы".to_string(),
                check: "Шах".to_string(),
                mate: "Мат".to_string(),
                turn: "Ход".to_string(),
                button_new_game: "Новая игра".to_string(),
                button_continue_game: "Продолжить".to_string(),
                button_exit_game: "Выход".to_string(),
            },
        }
    }
}
