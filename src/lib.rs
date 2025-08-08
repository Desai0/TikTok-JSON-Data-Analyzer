// lib.rs
// lib.rs is in charge of :
// - Turning the arguments given by main.rs (= the user's TikTok data) into an instance of the "Statistics" struct
// The library processes all of this information and nicely puts it into an instance of the struct, which is then
// returned to main.rs

use serde_json::Value;
use std::collections::HashMap;
mod date_utils;

#[derive(Clone)]
pub struct ActivityItem {
    pub date: String,
    pub content: String,
}

pub struct DateInfo {
    pub first: ActivityItem,
    pub last: ActivityItem,
}

pub struct Statistics {
    pub username: String,
    pub logins: HashMap<String, usize>,
    pub watched: HashMap<String, usize>,
    pub time: String,
    pub favorites: HashMap<String, usize>,
    pub likes_left: HashMap<String, usize>,
    pub comments: usize,
    pub dms: HashMap<String, usize>,
    pub likes_received: usize,
    pub videos_published: usize,
    pub shares: usize,
    pub hashtags_viewed: usize,
    pub comment_info: Option<DateInfo>,
    pub like_info: Option<DateInfo>,
    pub watch_info: Option<DateInfo>,
    pub dm_info: Option<DateInfo>,
}

impl Statistics {
    pub fn build(data: Value) -> Statistics {
        let username =
            String::from(&data["Profile"]["Profile Info"]["userName"].to_string().replace("\"", ""));

        let latest_timestamp = find_latest_timestamp(&data);

        let watched_per_day = read_videos(latest_timestamp, &data)
            .get("Watched per day")
            .unwrap_or(&0usize)
            .to_owned();

        Statistics {
            username,
            logins: read_logins(latest_timestamp, &data),
            watched: read_videos(latest_timestamp, &data),
            time: daily_time(watched_per_day),
            favorites: favorites(&data),
            likes_left: likes(latest_timestamp, &data),
            comments: value_length(&data["Comment"]["Comments"]["CommentsList"]),
            dms: private_messages(&data),
            likes_received: audience_stats(&data)
                .get("Likes received")
                .unwrap_or(&0usize)
                .to_owned(),
            videos_published: audience_stats(&data)
                .get("Videos published")
                .unwrap_or(&0usize)
                .to_owned(),
            shares: value_length(&data["Your Activity"]["Share History"]["ShareHistoryList"]),
            hashtags_viewed: value_length(&data["Your Activity"]["Hashtag"]["HashtagList"]),
            comment_info: get_comment_info(&data),
            like_info: get_like_info(&data),
            watch_info: get_watch_info(&data),
            dm_info: get_dm_info(&data),
        }
    }
}

// Calculates how many elements there are in a JSON category
fn value_length(input_value: &Value) -> usize {
    input_value.as_array().map(|a| a.len()).unwrap_or(0)
}

fn find_latest_timestamp(data: &Value) -> i64 {
    let paths = vec![
        &data["Your Activity"]["Login History"]["LoginHistoryList"],
        &data["Your Activity"]["Watch History"]["VideoList"],
        &data["Your Activity"]["Like List"]["ItemFavoriteList"],
    ];

    let mut max_ts = 0;

    for path in paths {
        if let Some(list) = path.as_array() {
            for item in list {
                let date_str_opt = item.get("Date")
                    .or_else(|| item.get("date"))
                    .and_then(|d| d.as_str());

                if let Some(date_str) = date_str_opt {
                    if let Some(ts) = date_utils::date_to_unix_timestamp(date_str) {
                        if ts > max_ts {
                            max_ts = ts;
                        }
                    }
                }
            }
        }
    }
    max_ts
}

fn get_comment_info(data: &Value) -> Option<DateInfo> {
    let list = data["Comment"]["Comments"]["CommentsList"].as_array()?;
    if list.is_empty() {
        return None;
    }

    let first = ActivityItem {
        date: list[0].get("date").and_then(|d| d.as_str()).unwrap_or("").to_string(),
        content: list[0].get("comment").and_then(|c| c.as_str()).unwrap_or("").to_string(),
    };

    let last_item = &list[list.len() - 1];
    let last = ActivityItem {
        date: last_item.get("date").and_then(|d| d.as_str()).unwrap_or("").to_string(),
        content: last_item.get("comment").and_then(|c| c.as_str()).unwrap_or("").to_string(),
    };

    Some(DateInfo { first, last })
}

fn get_like_info(data: &Value) -> Option<DateInfo> {
    let list = data["Your Activity"]["Like List"]["ItemFavoriteList"].as_array()?;
    if list.is_empty() {
        return None;
    }

    // Likes seem to be sorted new to old
    let last_item = &list[list.len() - 1];
    let first = ActivityItem {
        date: last_item.get("date").and_then(|d| d.as_str()).unwrap_or("").to_string(),
        content: last_item.get("link").and_then(|l| l.as_str()).unwrap_or("").to_string(),
    };
    
    let first_item = &list[0];
    let last = ActivityItem {
        date: first_item.get("date").and_then(|d| d.as_str()).unwrap_or("").to_string(),
        content: first_item.get("link").and_then(|l| l.as_str()).unwrap_or("").to_string(),
    };

    Some(DateInfo { first, last })
}

fn get_watch_info(data: &Value) -> Option<DateInfo> {
    let list = data["Your Activity"]["Watch History"]["VideoList"].as_array()?;
    if list.is_empty() {
        return None;
    }

    // Watch history is new to old
    let last_item = &list[list.len() - 1];
    let first = ActivityItem {
        date: last_item.get("Date").and_then(|d| d.as_str()).unwrap_or("").to_string(),
        content: last_item.get("Link").and_then(|l| l.as_str()).unwrap_or("No link found").to_string(),
    };
    
    let first_item = &list[0];
    let last = ActivityItem {
        date: first_item.get("Date").and_then(|d| d.as_str()).unwrap_or("").to_string(),
        content: first_item.get("Link").and_then(|l| l.as_str()).unwrap_or("No link found").to_string(),
    };

    Some(DateInfo { first, last })
}

fn get_dm_info(data: &Value) -> Option<DateInfo> {
    let chat_history = data["Direct Message"]["Direct Messages"]["ChatHistory"].as_object()?;

    let mut first_item: Option<ActivityItem> = None;
    let mut last_item: Option<ActivityItem> = None;

    let mut min_ts = i64::MAX;
    let mut max_ts = 0;

    for (chat_name, messages) in chat_history {
        if let Some(messages_array) = messages.as_array() {
            for msg in messages_array {
                if let Some(date_str) = msg.get("Date").and_then(|d| d.as_str()) {
                    if let Some(ts) = date_utils::date_to_unix_timestamp(date_str) {
                        let content = msg.get("Content").and_then(|c| c.as_str()).unwrap_or("").to_string();

                        if ts < min_ts {
                            min_ts = ts;
                            first_item = Some(ActivityItem { date: date_str.to_string(), content: format!("(in {}) {}", chat_name, content) });
                        }
                        if ts > max_ts {
                            max_ts = ts;
                            last_item = Some(ActivityItem { date: date_str.to_string(), content: format!("(in {}) {}", chat_name, content) });
                        }
                    }
                }
            }
        }
    }
    
    if let (Some(first), Some(last)) = (first_item, last_item) {
        Some(DateInfo { first, last })
    } else {
        None
    }
}


// The following functions (except the test functions) calculate specific data
fn read_logins(latest_timestamp: i64, data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    let login_history = &data["Your Activity"]["Login History"]["LoginHistoryList"];
    let login_history_len = value_length(login_history);

    let days_since_1st_login = if login_history_len > 0 {
        login_history[0]
            .get("Date")
            .and_then(|d| d.as_str())
            .and_then(|s| date_utils::days_between(latest_timestamp, s))
            .unwrap_or(0)
    } else {
        0
    };

    let launches_per_day = if days_since_1st_login > 0 {
        login_history_len / days_since_1st_login
    } else {
        login_history_len
    };

    result.insert(String::from("Days since 1st login"), days_since_1st_login);
    result.insert(String::from("Openings"), login_history_len);
    result.insert(String::from("Launches per day"), launches_per_day);

    result
}

fn read_videos(latest_timestamp: i64, data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();
    let watched_videos = &data["Your Activity"]["Watch History"]["VideoList"];
    let watched_videos_len = value_length(watched_videos);

    let days_since_1st_vid = if watched_videos_len > 0 {
        watched_videos[watched_videos_len - 1]
            .get("Date")
            .and_then(|d| d.as_str())
            .and_then(|s| date_utils::days_between(latest_timestamp, s))
            .unwrap_or(0)
    } else {
        0
    };

    let watched_per_day = if days_since_1st_vid > 0 {
        watched_videos_len / days_since_1st_vid
    } else {
        watched_videos_len
    };

    result.insert(String::from("Days since 1st video"), days_since_1st_vid);
    result.insert(String::from("Videos watched"), watched_videos_len);
    result.insert(String::from("Watched per day"), watched_per_day);

    result
}

fn daily_time(watched_per_day: usize) -> String {
    let total_time_in_minutes = (watched_per_day as f32 * 27.5) as usize / 60;
    // We are converting types for precision and readability purposes. (one after the other)
    // Originally the time is in seconds, but we transform it into minutes for simplicity (that's the / 60).
    let hours = total_time_in_minutes / 60;
    let minutes = total_time_in_minutes % 60;

    let result = format!("{} hours and {} minutes", hours, minutes);
    result
}

fn favorites(data: &Value) -> HashMap<String, usize> {
    let mut result = HashMap::new();

    let effects_len = value_length(&data["Your Activity"]["Favorite Effects"]["FavoriteEffectsList"]);

    let hashtags_len = value_length(&data["Your Activity"]["Favorite Hashtags"]["FavoriteHashtagList"]);

    let sounds_len = value_length(&data["Your Activity"]["Favorite Sounds"]["FavoriteSoundList"]);

    let videos_len = value_length(&data["Your Activity"]["Favorite Videos"]["FavoriteVideoList"]);

    result.insert(String::from("Effects"), effects_len);
    result.insert(String::from("Hashtags"), hashtags_len);
    result.insert(String::from("Sounds"), sounds_len);
    result.insert(String::from("Videos"), videos_len);

    result
}

fn likes(latest_timestamp: i64, data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    let watched_videos = &data["Your Activity"]["Watch History"]["VideoList"];
    let watched_videos_len = value_length(watched_videos);
    let days_since_1st_vid = if watched_videos_len > 0 {
        watched_videos[watched_videos_len - 1]
            .get("Date")
            .and_then(|d| d.as_str())
            .and_then(|s| date_utils::days_between(latest_timestamp, s))
            .unwrap_or(0)
    } else {
        0
    };

    let watched_per_day_float = if days_since_1st_vid > 0 {
        watched_videos_len as f64 / days_since_1st_vid as f64
    } else {
        watched_videos_len as f64
    };

    let liked_videos = &data["Your Activity"]["Like List"]["ItemFavoriteList"];
    let liked_videos_len = value_length(liked_videos);

    let days_since_oldest_like = if liked_videos_len > 0 {
        liked_videos[liked_videos_len - 1]
            .get("date")
            .and_then(|d| d.as_str())
            .and_then(|s| date_utils::days_between(latest_timestamp, s))
            .unwrap_or(0)
    } else {
        0
    };

    let likes_per_day = if days_since_oldest_like > 0 {
        liked_videos_len / days_since_oldest_like
    } else {
        liked_videos_len
    };

    let liked_percentage = if watched_per_day_float > 0.0 {
        ((likes_per_day as f64 / watched_per_day_float) * 100.0) as usize
    } else {
        0
    };

    result.insert(String::from("Videos liked"), liked_videos_len);
    result.insert(
        String::from("Days since oldest like"),
        days_since_oldest_like,
    );
    result.insert(String::from("Likes per day"), likes_per_day);
    result.insert(
        String::from("Liked videos percentage"),
        liked_percentage,
    );

    result
}

fn private_messages(data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    if let Some(chat_history) = data
        .get("Direct Message")
        .and_then(|dm| dm.get("Direct Messages"))
        .and_then(|ch| ch.get("ChatHistory"))
        .and_then(|ch_map| ch_map.as_object())
    {
        for (chat_name, chat_messages) in chat_history.iter() {
            if let Some(messages_array) = chat_messages.as_array() {
                let count = messages_array.len();
                if chat_name.len() > 17 {
                    result.insert(format!("Chat with{}", &chat_name[17..]), count);
                } else {
                    result.insert(chat_name.clone(), count);
                }
            }
        }
    }

    result
}

fn audience_stats(data: &Value) -> HashMap<String, usize> {
    let mut result: HashMap<String, usize> = HashMap::new();

    let videos_published = value_length(&data["Post"]["Posts"]["VideoList"]);

    let likes_u64 = data
        .get("Profile")
        .and_then(|p| p.get("Profile Info"))
        .and_then(|pi| pi.get("likesReceived"))
        .and_then(|lr| lr.as_str())
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(0);

    result.insert(String::from("Videos published"), videos_published);
    result.insert(String::from("Likes received"), likes_u64 as usize);

    result
}

#[cfg(test)]
mod tests {
    use core::panic;

    mod tests_read_file;

    // These functions test what happens in different cases by using files that I made
    // in order to check if the file is readable and valid

    #[test]
    #[should_panic(expected = "Error reading the file")]
    fn file_not_found() {
        // this function tests what happens when the file isn't found
        let file_path = "path/to/absolutely/no/file.txt";
        tests_read_file::file_into_str(file_path);
    }

    #[test]
    fn file_found_but_not_valid() {
        // This function tests what happens when the file is found but isn't valid
        let file_path = "src/tests/not_valid_file.json";
        let file_content = tests_read_file::file_into_str(file_path);
        let data = tests_read_file::str_into_object(file_content).unwrap_or_else(|err| {
            eprintln!("Error while trying to read string: {err}");
            panic!("Error while trying to read string: {err}")
        });
        let username = &data["Profile"]["Profile Info"]["userName"];

        // To check the validity of the file, we are checking if we can find the userName value.
        // If we can't, then the file is not valid
        // Obviously we could just copy-paste the "Profile" section of regular data into the file and it would work...
        // But then we can say that you have tried really hard to make the program fail!
        assert_eq!(username.as_str(), None);
        // In this case, we're checking if we can access the "userName" value, which doesn' exist in not_valid_file.json
        // When trying to access a value that does not exist, serde_json returns a None value.
    }

    #[test]
    fn all_good() {
        // This function tests if the file is readable and is valid (= userName value can be read)
        // The file given to analyze is not the real TikTok data file, but it contains the username,
        // in order to pass the test.
        let file_path = "src/tests/partially_valid_file.json";
        let file_content = tests_read_file::file_into_str(file_path);
        let data = tests_read_file::str_into_object(file_content).unwrap_or_else(|err| {
            eprintln!("Error while trying to read string: {err}");
            panic!("Error while trying to read string: {err}")
        });
        let username = &data["Profile"]["Profile Info"]["userName"];

        assert_eq!(username.as_str(), Some("john.doe"));
        // Same as for function file_found_but_not_valid, but here the value exists
    }
}
