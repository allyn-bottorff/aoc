fn main() {
    println!("Hello, world!");
    let test_data = r"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    let test_data = std::fs::read_to_string("data.txt").unwrap();

    let word_search: Vec<Vec<char>> = test_data
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect();

    let pt1_search_word = ['X', 'M', 'A', 'S'];

    let pt1_found = pt1_count_words(&word_search, &pt1_search_word);
    println!("pt1 found: {}", pt1_found);

    let pt2_search_word = ['M', 'A', 'S'];
    let pt2_found = pt2_count_words(&word_search, &pt2_search_word);
    println!("pt2 found: {}", pt2_found);
}

fn pt1_count_words(word_search: &Vec<Vec<char>>, search_word: &[char]) -> i32 {
    let i_len = word_search.len();
    let mut found: i32 = 0;
    for (i, line) in word_search.iter().enumerate() {
        let j_len = line.len();
        for (j, _cell) in line.iter().enumerate() {
            //check up
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if i >= search_word.len() - 1 {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i - k][j] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
            //check upright
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if i >= search_word.len() - 1 && j <= (j_len - search_word.len()) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i - k][j + k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
            //check right
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if j <= j_len - search_word.len() {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i][j + k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
            //check downright
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if (i <= i_len - search_word.len()) && (j <= j_len - search_word.len()) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i + k][j + k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
            //check down
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if i <= i_len - search_word.len() {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i + k][j] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
            //check downleft
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if (i <= i_len - search_word.len()) && (j >= search_word.len() - 1) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i + k][j - k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
            //check left
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if j >= search_word.len() - 1 {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i][j - k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
            //check upleft
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if (i >= search_word.len() - 1) && (j >= search_word.len() - 1) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i - k][j - k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    found += 1;
                }
            }
        }
    }

    found
}

fn pt2_count_words(word_search: &Vec<Vec<char>>, search_word: &[char]) -> i32 {
    let mut centers: Vec<(usize, usize)> = Vec::new();
    let i_len = word_search.len();
    let mut found: i32 = 0;
    for (i, line) in word_search.iter().enumerate() {
        let j_len = line.len();
        for (j, _cell) in line.iter().enumerate() {
            //check upright
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if i >= search_word.len() - 1 && j <= (j_len - search_word.len()) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i - k][j + k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    centers.push((i - 1, j + 1)); //magic center value because we know where it is
                }
            }
            //check downright
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if (i <= i_len - search_word.len()) && (j <= j_len - search_word.len()) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i + k][j + k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    centers.push((i + 1, j + 1)); //magic center value because we know where it is
                }
            }
            //check downleft
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if (i <= i_len - search_word.len()) && (j >= search_word.len() - 1) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i + k][j - k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    centers.push((i + 1, j - 1)); //magic center value because we know where it is
                }
            }
            //check upleft
            let mut local_search_word = search_word.iter();
            for k in 0..search_word.len() {
                if (i >= search_word.len() - 1) && (j >= search_word.len() - 1) {
                    if let Some(letter) = local_search_word.next() {
                        if word_search[i - k][j - k] != *letter {
                            break;
                        }
                    }
                } else {
                    break;
                }
                if k == search_word.len() - 1 {
                    centers.push((i - 1, j - 1)); //magic center value because we know where it is
                }
            }
        }
    }

    for center1 in &centers {
        let mut found_centers: i32 = -1;
        for center2 in &centers {
            if center1 == center2 {
                if center1 == center2 {
                    found_centers += 1;
                }
            }
        }
        found += found_centers;
    }

    found / 2
}
