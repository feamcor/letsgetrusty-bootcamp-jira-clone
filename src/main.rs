use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() {
    let database = JiraDatabase::new("data/db.json".to_owned());
    let mut navigator = Navigator::new(Rc::new(database));

    loop {
        clearscreen::clear().unwrap();

        let current_page = match navigator.get_current_page() {
            Some(current_page) => current_page,
            None => break,
        };

        match current_page.draw_page() {
            Ok(_) => {},
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            },
        }

        let input = get_user_input();

        match current_page.handle_input(&input) {
            Ok(current_action) => {
                match current_action {
                    Some(current_action) => {
                        match navigator.handle_action(current_action) {
                            Ok(_) => {},
                            Err(e) => {
                                eprintln!("Error: {}", e);
                                break;
                            },
                        }
                    },
                    None => {},
                }
            }
            Err(e) => {
                eprintln!("Error: {}", e);
                break;
            },
        }
    }
}
