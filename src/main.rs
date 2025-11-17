use std::rc::Rc;

mod models;

mod db;
use db::*;

mod ui;

mod io_utils;
use io_utils::*;

mod navigator;
use navigator::*;

fn main() -> anyhow::Result<()> {
    let database = JiraDatabase::new("data/db.json");
    let mut navigator = Navigator::new(Rc::new(database));

    while let Some(current_page) = navigator.get_current_page() {
        clearscreen::clear()?;
        current_page.draw_page()?;
        let input = get_user_input()?; // see io_utils.rs suggestion
        if let Some(action) = current_page.handle_input(&input)? {
            navigator.handle_action(action)?;
        }
    }

    Ok(())
}
