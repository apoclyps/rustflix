use crate::args::{CreateVideo, DeleteEntity, UpdateVideo, VideoCommand, VideoSubcommand};

use crate::db::establish_connection;
use crate::models::{NewVideo, Video};
use diesel::prelude::*;

pub fn handle_video_command(video: VideoCommand) {
    let command = video.command;
    match command {
        VideoSubcommand::Create(video) => {
            create_video(video);
        }
        VideoSubcommand::Update(video) => {
            update_video(video);
        }
        VideoSubcommand::Delete(delete_entity) => {
            delete_video(delete_entity);
        }
        VideoSubcommand::Show => {
            show_videos();
        }
    }
}

pub fn create_video(video: CreateVideo) {
    println!("Creating video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let new_video = NewVideo {
        title: &video.title,
        description: &video.description,
        removed: false,
    };

    diesel::insert_into(videos)
        .values(&new_video)
        .execute(&mut establish_connection())
        .expect("Error saving new video");
}

pub fn update_video(video: UpdateVideo) {
    println!("Updating video: {:?}", video);
    use crate::schema::videos::dsl::*;

    let db_video = Video {
        id: video.id,
        title: video.title,
        description: video.description,
        removed: false,
    };

    diesel::update(videos.find(video.id))
        .set(&db_video)
        .execute(&mut establish_connection())
        .expect("Error updating video");
}

pub fn delete_video(video: DeleteEntity) {
    println!("Deleting video: {:?}", video);
    use crate::schema::videos::dsl::*;

    diesel::delete(videos.find(video.id))
        .execute(&mut establish_connection())
        .expect("Error deleting video");
}

pub fn show_videos() {
    println!("Showing videos");
    use crate::schema::videos::dsl::*;

    let results = videos
        .filter(removed.eq(false))
        .load::<Video>(&mut establish_connection())
        .expect("Error loading videos");

    println!("Displaying {} videos", results.len());
    for video in results {
        println!("{:?}", video);
    }
}
