use std::fs::File;
use std::io::{BufReader, BufRead};

fn trees_for_path(map: Vec<Vec<char>>, row_stride: u32, col_stride: u32) -> u32 {
    let size_rows = map.len() as u32;
    let size_cols = map.get(0).unwrap().len() as u32;
    let mut row = 0u32;
    let mut col = 0u32;
    let mut num_trees = 0;
    while row < size_rows {
        let relative_col = col % size_cols;
        let map_val = map.get(row as usize)
            .unwrap()
            .get(relative_col as usize)
            .unwrap();
        if map_val.to_string() == "#".to_string() {
            num_trees += 1;
        }
        row += row_stride;
        col += col_stride;
    }
    return num_trees;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let reader = BufReader::new(file);
    let mut rows = Vec::new();
    let mut size_rows = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let chars = line.chars();
        let mut row = Vec::new();
        for map_value in chars {
            row.push(map_value.clone());
        }
        rows.push(row);
        size_rows += 1;
    }
    let num_trees_1 = trees_for_path(rows.clone(), 1, 1);
    let num_trees_2 = trees_for_path(rows.clone(), 1, 3);
    let num_trees_3 = trees_for_path(rows.clone(), 1, 5);
    let num_trees_4 = trees_for_path(rows.clone(), 1, 7);
    let num_trees_5 = trees_for_path(rows.clone(), 2, 1);
    println!("Product: {:?}", num_trees_1 * num_trees_2 * num_trees_3 * num_trees_4 * num_trees_5);
}
