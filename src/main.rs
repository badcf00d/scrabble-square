use rayon::prelude::*;
use regex::{Captures, Regex};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::{
    env,
    fs::{read_to_string, File},
    io::Write,
    sync::Mutex,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn find_regex_groups<'a>(regex: String, haystack: &'a Vec<&str>) -> Vec<Captures<'a>> {
    let mut list: Vec<Captures> = Vec::new();
    let re: Regex = Regex::new(regex.as_str()).unwrap();

    for word in haystack {
        if let Some(caps) = re.captures(word) {
            list.push(caps);
        }
    }

    list
}

fn produce_line_list<'a>(
    word_list: &'a Vec<&'a str>,
    word1: &str,
    word2: &str,
) -> Vec<Vec<Captures<'a>>> {
    let mut rng = thread_rng();

    let mut line_list: Vec<Vec<Captures>> = vec![];

    let mut first_line = find_regex_groups(
        format!(
            ".*({}).{}.*",
            word1.chars().next().unwrap(),
            word2.chars().next().unwrap()
        ),
        word_list,
    );
    let mut second_line = find_regex_groups(
        format!(
            ".*({}).{}.*",
            word1.chars().nth(1).unwrap(),
            word2.chars().nth(1).unwrap()
        ),
        word_list,
    );
    let mut third_line = find_regex_groups(
        format!(
            ".*({}).{}.*",
            word1.chars().nth(2).unwrap(),
            word2.chars().nth(2).unwrap()
        ),
        word_list,
    );
    let mut fourth_line = find_regex_groups(
        format!(
            ".*({}).{}.*",
            word1.chars().nth(3).unwrap(),
            word2.chars().nth(3).unwrap()
        ),
        word_list,
    );
    let mut fifth_line = find_regex_groups(
        format!(
            ".*({}).{}.*",
            word1.chars().nth(4).unwrap(),
            word2.chars().nth(4).unwrap()
        ),
        word_list,
    );

    first_line.shuffle(&mut rng);
    second_line.shuffle(&mut rng);
    third_line.shuffle(&mut rng);
    fourth_line.shuffle(&mut rng);
    fifth_line.shuffle(&mut rng);

    line_list.push(first_line);
    line_list.push(second_line);
    line_list.push(third_line);
    line_list.push(fourth_line);
    line_list.push(fifth_line);

    line_list
}

fn word_grid_test(matches: [&Captures; 5], word_list: &Vec<&str>, output: &Mutex<File>) {
    let mut grid: [String; 5] = [
        String::with_capacity(32),
        String::with_capacity(32),
        String::with_capacity(32),
        String::with_capacity(32),
        String::with_capacity(32),
    ];
    let mut match_pos;

    for i in 0..matches.len() {
        match_pos = 12 - matches[i].get(1).unwrap().start();
        grid[i].push_str(&" ".repeat(match_pos));
        grid[i].push_str(&matches[i][0]);
    }

    let longest_row = grid.iter().max_by_key(|x| x.len()).unwrap();

    for i in 0..longest_row.len() {
        let vertical_string: String = grid.iter().filter_map(|s| s.chars().nth(i)).collect();

        let vertical_words: Vec<&str> = vertical_string
            .split_whitespace()
            .filter(|word| word.len() >= 2)
            .collect();

        for vertical_word in vertical_words {
            if word_list.binary_search(&vertical_word).is_err() {
                return;
            }
        }
    }

    let result = format!(
        "--------------------------\n{}\n{}\n{}\n{}\n{}\n",
        &grid[0], &grid[1], &grid[2], &grid[3], &grid[4]
    );
    println!("{}", result);

    output.lock().unwrap().write_all(result.as_bytes()).unwrap();
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut word_list_string: Vec<String> = read_lines("sowpods.txt");

    word_list_string.retain(|word| word.len() < 12);

    let word_list: Vec<&str> = word_list_string.iter().map(|x| x.as_str()).collect();

    let line_list: Vec<Vec<Captures>> = produce_line_list(&word_list, &args[1], &args[2]);

    let raw_file = File::create("choices.log").unwrap();
    let file = Mutex::new(raw_file);

    line_list[0]
    .par_iter()
    .enumerate()
    .for_each(|(_i, first_match)| {
        for second_match in &line_list[1] {
            for third_match in &line_list[2] {
                for fourth_match in &line_list[3] {
                    for fifth_match in &line_list[4] {
                        word_grid_test([first_match, second_match, third_match, fourth_match, fifth_match], &word_list, &file);
                    }
                }
            }
        }
    });
}
