pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut found = Vec::new();

    let row_count = input.len();

    for (row_num, row) in input.iter().enumerate() {
        for (col_num, item) in row.iter().enumerate() {
            if row.iter().all(|x| x <= item) && (0..row_count).all(|x| input[x][col_num] >= *item) {
                found.push((row_num, col_num));
            }
        }
    }

    found
}
