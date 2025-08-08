// main.rs
// main.rs is in charge of :
// - calling the functions in read_file.rs to get the data from the JSON file
// - calling the lib.rs "build" function with the data as an argument -
// that will build an instance of the "Statistics" struct which contains the processed data
// - print the data to the console in a user-friendly way

mod read_file;
use std::process;

use serde_json::Value;
use tiktok_json_analyzer::Statistics;
mod chart_utils;

fn main() {
    println!("---------- The TikTok JSON Analyzer ----------");
    println!("This program reads the TikTok JSON data export and calculates statistics");
    println!(
        "Read the documentation at https://github.com/Elazrod56/tiktok_json_analyzer#readme\n"
    );

    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("\u{274C} ERROR: No JSON file path provided.");
        eprintln!("Usage: cargo run -- <path_to_your_json_file>");
        process::exit(1);
    }
    let file_path = &args[1];

    let file = read_file::file_into_str(file_path);
    println!("\u{2705} File read successfully!");

    let data = read_file::str_into_object(file).unwrap_or_else(|err| {
        eprintln!(
            "\u{274C} ERROR when converting file into object : {err}\nFile is most likely invalid."
        );
        process::exit(2);
    });
    check_data_validity(&data);
    println!("\u{2705} Data seems valid!\n");

    let statistics = Statistics::build(data);
    println!(
        "The data of {} has been analyzed. Results :",
        statistics.username
    );

    println!("\n---------- LOGINS \u{1F511} ----------");
    println!(
        "In the last {} days, you've launched TikTok {} times",
        statistics.logins.get("Days since 1st login").unwrap(),
        statistics.logins.get("Openings").unwrap()
    ); // it's safe to unwrap because there would have been an error sooner if something was wrong
       // it's also much more concise in terms of code length
    println!(
        "- {} launches per day on average",
        statistics.logins.get("Launches per day").unwrap()
    );

    println!("\n---------- VIDEO CONSUMPTION \u{1F4FA} ----------");
    println!(
        "In the last {} days, you've watched {} videos",
        statistics.watched.get("Days since 1st video").unwrap(),
        statistics.watched.get("Videos watched").unwrap()
    );
    println!(
        "- {} watched videos per day on average",
        statistics.watched.get("Watched per day").unwrap()
    );
    if let Some(info) = statistics.watch_info {
        println!("  - First seen: {} ({})", info.first.content, info.first.date);
        println!("  - Last seen: {} ({})", info.last.content, info.last.date);
    }

    println!("\n---------- TIME SPENT DAILY \u{1F570}----------");
    chart_utils::print_time_spent_chart(
        (*statistics.watched.get("Watched per day").unwrap_or(&0) as f32 * 27.5 / 60.0) as usize,
    );
    println!("TikTok videos are on average 27.5 seconds long, so we can estimate how much time you spend on TikTok every day.");
    println!(
        "Time wasted on TikTok every day : {} on average",
        statistics.time
    );
    println!("This stat is not 100% precise! The more you tend to not watch whole videos, the more time you can remove.");

    println!("\n---------- FAVORITES \u{1F60D} ----------");
    println!(
        "{} favorite effects",
        statistics.favorites.get("Effects").unwrap()
    );
    println!(
        "{} favorite hashtags",
        statistics.favorites.get("Hashtags").unwrap()
    );
    println!(
        "{} favorite sounds",
        statistics.favorites.get("Sounds").unwrap()
    );
    println!(
        "{} favorite videos",
        statistics.favorites.get("Videos").unwrap()
    );

    println!("\n---------- LIKES \u{2764} ----------");
    println!(
        "You've liked {} videos in the last {} days",
        statistics.likes_left.get("Videos liked").unwrap(),
        statistics.likes_left.get("Days since oldest like").unwrap()
    );
    println!(
        "You've liked {} videos per day on average\n",
        statistics.likes_left.get("Likes per day").unwrap()
    );
    println!(
        "You've liked {}% of the videos you've watched\n",
        statistics
            .likes_left
            .get("Liked videos percentage")
            .unwrap()
    );
    if let Some(info) = statistics.like_info {
        println!("  - First liked video: {} ({})", info.first.content, info.first.date);
        println!("  - Last liked video: {} ({})", info.last.content, info.last.date);
    }

    println!("\n---------- COMMENTS \u{1F4AC} ----------");
    println!("You've posted {} comments", statistics.comments);
    if let Some(info) = statistics.comment_info {
        println!("  - First comment: \"{}\" ({})", info.first.content, info.first.date);
        println!("  - Last comment: \"{}\" ({})", info.last.content, info.last.date);
    }

    println!("\n---------- DIRECT MESSAGES \u{2709} ----------");
    for entry in statistics.dms.iter() {
        println!("{} {} messages", entry.0, entry.1);
    }
    println!(
        "\nTotal number of messages : {}",
        statistics.dms.values().copied().sum::<usize>()
    );
    chart_utils::print_dms_chart(&statistics.dms);
    if let Some(info) = statistics.dm_info {
        println!("  - First message: \"{}\" ({})", info.first.content, info.first.date);
        println!("  - Last message: \"{}\" ({})", info.last.content, info.last.date);
    }

    println!("\n---------- REPOSTS \u{1F504} ----------");
    println!("You've shared {} videos", statistics.shares);

    println!("\n---------- HASHTAGS \u{1F516} ----------");
    println!(
        "You've viewed content from {} different hashtags",
        statistics.hashtags_viewed
    );

    println!("\n---------- AUDIENCE STATISTICS \u{1F464} ----------");

    if statistics.likes_received > 0 && statistics.videos_published == 0 {
        println!("You've received {} likes but no videos were found in the data. You must have deleted old videos", statistics.likes_received);
    } else if statistics.likes_received == 0 && statistics.videos_published == 0 {
        println!("You haven't posted any videos or received any likes");
    } else if statistics.likes_received == 0 && statistics.videos_published > 0 {
        println!(
            "You've published {} videos - sadly no one liked them",
            statistics.videos_published
        );
    } else {
        println!(
            "You've received {} likes with {} videos",
            statistics.likes_received, statistics.videos_published
        );
        println!(
            "You got {} likes per video on average",
            (statistics.likes_received / statistics.videos_published)
        );
    }
    println!("Note that likes from old videos are still counted.")
}

fn check_data_validity(data: &Value) {
    let profile_info = &data["Profile"]["Profile Info"];
    // To check the validity of the file, we are checking if we can find the Profile Info value.
    // If we can't, then the file is not valid
    if profile_info.is_null() {
        eprintln!("\u{274C} ERROR: File is readable but doesn't seem to be valid data!");
        eprintln!("Could not find the 'Profile Info' section within the 'Profile' section.");
        eprintln!("This is necessary to retrieve basic user information.");
        process::exit(2);
    }
}
