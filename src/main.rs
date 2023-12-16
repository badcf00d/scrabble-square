use rayon::prelude::*;
use regex::{Captures, Regex};

use rand::seq::SliceRandom;
use rand::thread_rng;

use std::{
    fs::{read_to_string, File},
    io::{BufWriter, Write},
    sync::Mutex, env,
};

fn read_lines(filename: &str) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for line in read_to_string(filename).unwrap().lines() {
        result.push(line.to_string())
    }

    result
}

fn is_word_in_list(haystack: &Vec<&str>, needle: &str) -> bool {
    // for word in haystack {
    //     if word == needle {
    //         return true;
    //     }
    // }

    haystack.binary_search(&needle).is_ok()

    //false
}

fn search_for_string<'a>(regex: String, haystack: &'a Vec<&str>) -> Vec<Captures<'a>> {
    let mut list: Vec<Captures> = Vec::new();
    let re: Regex = Regex::new(regex.as_str()).unwrap();

    for word in haystack {
        if let Some(caps) = re.captures(&word) {
            list.push(caps);
        }
    }

    list
}

// fn search_list(
//     mut index_array: [usize; 5],
//     list_to_iterate: usize,
//     line_list: &Vec<Vec<Captures>>,
//     words_5_letter: &Vec<String>,
//     output: &Mutex<File>,
// ) {
//     for i in 0..line_list[list_to_iterate].len() {
//         let test = [
//             &line_list[0][index_array[0]],
//             &line_list[1][index_array[1]],
//             &line_list[2][index_array[2]],
//             &line_list[3][index_array[3]],
//             &line_list[4][index_array[4]],
//         ];

//         let search = format!(
//             "{}{}{}{}{}",
//             &test[0][1], &test[1][1], &test[2][1], &test[3][1], &test[4][1]
//         );
//         let middle_word = search_for_string(search.as_str(), words_5_letter);

//         if middle_word.len() > 0 {
//             let result = format!(
//                 "--------------------------\n{}\n{}\n{}\n{}\n{}\n",
//                 &test[0][0], &test[1][0], &test[2][0], &test[3][0], &test[4][0]
//             );
//             //print!("{result}");
//             output.lock().unwrap().write_all(result.as_bytes()).unwrap();
//         }
//         index_array[list_to_iterate] = i;
//     }
// }

fn produce_line_list<'a>(word_list: &'a Vec<&'a str>, word1: &str, word2: &str) -> Vec<Vec<Captures<'a>>> {
    let mut rng = thread_rng();

    let mut line_list: Vec<Vec<Captures>> = vec![];

    let mut first_line = search_for_string(format!(".*({}).{}.*", word1.chars().nth(0).unwrap(), word2.chars().nth(0).unwrap()), &word_list);
    let mut second_line = search_for_string(format!(".*({}).{}.*", word1.chars().nth(1).unwrap(), word2.chars().nth(1).unwrap()), &word_list);
    let mut third_line = search_for_string(format!(".*({}).{}.*", word1.chars().nth(2).unwrap(), word2.chars().nth(2).unwrap()), &word_list);
    let mut fourth_line = search_for_string(format!(".*({}).{}.*", word1.chars().nth(3).unwrap(), word2.chars().nth(3).unwrap()), &word_list);
    let mut fifth_line = search_for_string(format!(".*({}).{}.*", word1.chars().nth(4).unwrap(), word2.chars().nth(4).unwrap()), &word_list);

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

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut word_list_string: Vec<String> = read_lines("sowpods.txt");

    word_list_string = word_list_string
        .into_iter()
        .filter(|word| word.len() < 10)
        .collect();

    let word_list: Vec<&str> = word_list_string.iter().map(|x| x.as_str()).collect();



    // let mut restricted_word_lists: Vec<Vec<String>> = vec![];

    // let words_2_letters: Vec<String> = word_list.clone()
    //     .into_iter()
    //     .filter(|word| word.len() == 2)
    //     .collect();

    // let words_3_letters: Vec<String> = word_list.clone()
    //     .into_iter()
    //     .filter(|word| word.len() == 3)
    //     .collect();

    // let words_4_letters: Vec<String> = word_list.clone()
    //     .into_iter()
    //     .filter(|word| word.len() == 4)
    //     .collect();

    // let words_5_letters: Vec<String> = word_list.clone()
    //     .into_iter()
    //     .filter(|word| word.len() == 5)
    //     .collect();

    // restricted_word_lists.push(words_2_letters);
    // restricted_word_lists.push(words_3_letters);
    // restricted_word_lists.push(words_4_letters);
    // restricted_word_lists.push(words_5_letters);

    let line_list: Vec<Vec<Captures>> = produce_line_list(&word_list, &args[1], &args[2]);
    let find_two_letter_words_re: Regex = Regex::new(r"\w{2,}").unwrap();

    // let longest_list = line_list.iter().max_by_key(|x| x.len()).unwrap().len();

    let raw_file = File::create("choices.log").unwrap();
    let file = Mutex::new(raw_file);

    // (0..longest_list).into_par_iter().for_each(|i| {
    //     let mut index_array: [usize; 5] = [0; 5];

    //     index_array[0] = i % &line_list[0].len();
    //     index_array[1] = i % &line_list[1].len();
    //     index_array[2] = i % &line_list[2].len();
    //     index_array[3] = i % &line_list[3].len();
    //     index_array[4] = i % &line_list[4].len();

    //     println!("{}", i);

    //     for j in 0..line_list.len() {
    //         search_list(index_array, j, &line_list, &words_5_letter, &file);
    //     }
    // });

    line_list[0].par_iter().enumerate().for_each(|(i, first_match)| {
    //for first_match in & line_list[0] {
        for second_match in & line_list[1] {
            for third_match in & line_list[2]{
                println!("{}", &third_match[0]);

                for fourth_match in & line_list[3] {
                    for fifth_match in & line_list[4] {
                        let mut grid: Vec<String> = vec![];
                        let mut match_pos;
                        let mut valid_word = true;

                        match_pos = 10 - first_match.get(1).unwrap().start();
                        grid.push(format!("{:match_pos$}{}", "", &first_match[0]));

                        match_pos = 10 - second_match.get(1).unwrap().start();
                        grid.push(format!("{:match_pos$}{}", "", &second_match[0]));

                        match_pos = 10 - third_match.get(1).unwrap().start();
                        grid.push(format!("{:match_pos$}{}", "", &third_match[0]));

                        match_pos = 10 - fourth_match.get(1).unwrap().start();
                        grid.push(format!("{:match_pos$}{}", "", &fourth_match[0]));

                        match_pos = 10 - fifth_match.get(1).unwrap().start();
                        grid.push(format!("{:match_pos$}{}", "", &fifth_match[0]));

                        'outer: for i in 0..grid.iter().max_by_key(|x| x.len()).unwrap().len() {
                            let first_chars: String = grid.iter()
                                .filter_map(|s| s.chars().nth(i))
                                .collect();


                            for down_word in find_two_letter_words_re.captures_iter(&first_chars) {
                                if is_word_in_list(&word_list, &down_word[0]) == false {
                                    valid_word = false;
                                    break 'outer;
                                }                                   
                            }
                        }

                        if valid_word {
                            let result = format!("--------------------------\n{}\n{}\n{}\n{}\n{}\n",&grid[0],&grid[1],&grid[2],&grid[3],&grid[4]);
                            println!("{}", result);

                            file.lock().unwrap().write_all(result.as_bytes()).unwrap();
                        }

                        // let search = format!("{first_letter}{second_letter}{third_letter}{fourth_letter}{fifth_letter}");
                        // let middle_word = search_string(search.as_str(), &words_5_letter);
 
                        // if middle_word.len() > 0 {
                        //     let result = format!("--------------------------\n{}\n{}\n{}\n{}\n{}\n",&first_match[0],&second_match[0],&third_match[0],&fourth_match[0],&fifth_match[0]);

                        //     file.lock().unwrap().write_all(result.as_bytes()).unwrap();
                        // }
                    }
                }
            }
        }
    //}
    });
}
