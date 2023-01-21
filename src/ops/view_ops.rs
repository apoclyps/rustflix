use crate::args::{CreateView, ViewCommand, ViewSubcommand};
use crate::db::establish_connection;
use crate::models::{NewView, View as DBView};
use diesel::prelude::*;

pub fn handle_view_command(view: ViewCommand) {
    let command = view.command;
    match command {
        ViewSubcommand::Create(new_view) => {
            create_view(new_view);
        }
        ViewSubcommand::Show => {
            show_views();
        }
        ViewSubcommand::ShowPretty => {
            show_views_pretty();
        }
    }
}

fn create_view(new_view: CreateView) {
    println!("Creating view: {:?}", new_view);
    use crate::schema::views::dsl::*;

    let new_view = NewView {
        user_id: new_view.user_id,
        video_id: new_view.video_id,
        watch_start: &new_view.watch_start,
        duration: new_view.duration,
    };

    diesel::insert_into(views)
        .values(&new_view)
        .execute(&mut establish_connection())
        .expect("Error saving new view");
}

fn show_views() {
    println!("Showing views");

    use crate::schema::views::dsl::*;

    let results = views
        .load::<DBView>(&mut establish_connection())
        .expect("Error loading views");

    println!("Displaying {} views", results.len());
    for view in results {
        println!("{:?}", view);
    }
}

fn show_views_pretty() {
    println!("Showing views");

    use crate::schema::users;
    use crate::schema::videos;
    use crate::schema::views;

    let results = views::table
        .inner_join(videos::table)
        .inner_join(users::table)
        .select((
            users::name,
            videos::title,
            views::watch_start,
            views::duration,
        ))
        .load::<(String, String, chrono::NaiveDateTime, i32)>(&mut establish_connection())
        .expect("Error loading views");

    for view in results {
        println!("{:?} {:?} {:?} {:?}", view.0, view.1, view.2, view.3);
    }
}
