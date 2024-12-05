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

    let search_word = ['X', 'M', 'A', 'S'];

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

    println!("found: {}", found);
}
