use std::{collections::HashMap, fs};
use json::JsonValue;
use chrono::{DateTime, Utc, Datelike};

pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut week_counts = HashMap::new();
    
    if !data.is_array() {
        return week_counts;
    }

    for commit in data.members() {
        let date_str = commit["commit"]["author"]["date"]
            .as_str()
            .unwrap_or("");
        
        if let Ok(date) = DateTime::parse_from_rfc3339(date_str) {
            let utc_date: DateTime<Utc> = date.with_timezone(&Utc);
            let year = utc_date.year();
            let week = utc_date.iso_week().week();
            let week_key = format!("{}-W{}", year, week);
            
            *week_counts.entry(week_key).or_insert(0) += 1;
        }
    }
    
    week_counts
}

pub fn commits_per_author(data: &JsonValue) -> HashMap<String, u32> {
    let mut author_counts = HashMap::new();
    
    if !data.is_array() {
        return author_counts;
    }

    for commit in data.members() {
        if let Some(login) = commit["author"]["login"].as_str() {
            *author_counts.entry(login.to_string()).or_insert(0) += 1;
        }
    }
    
    author_counts
}   