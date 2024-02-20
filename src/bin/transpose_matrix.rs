// https://google.github.io/comprehensive-rust/tuples-and-arrays/exercise.html

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut new_matrix: [[i32; 3]; 3] = [[0; 3]; 3];
    for y in 0..matrix.len() {
        for x in 0..matrix[y].len() {
            new_matrix[x][y] = matrix[y][x];
        }
    }

    return new_matrix;
}

fn main() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("matrix: {:#?}", matrix);
    let transposed = transpose(matrix);
    println!("transposed: {:#?}", transposed);
}
