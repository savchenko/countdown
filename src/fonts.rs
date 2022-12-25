use std::collections::HashMap;

pub fn symbol_table<'a>() -> HashMap<char, ([&'a str; 1], usize)> {
    let data = vec![
        [ ":" ],
        [ "0" ],
        [ "1" ],
        [ "2" ],
        [ "3" ],
        [ "4" ],
        [ "5" ],
        [ "6" ],
        [ "7" ],
        [ "8" ],
        [ "9" ],
    ];
    let idx = vec![':', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
    let width: Vec<usize> = vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1];
    idx.iter()
        .zip(data.iter())
        .zip(width.iter())
        .map(|((idx, data), width)| (*idx, (*data, *width)))
        .collect()
}
