pub fn matrix_determinant_2x2(matrix: [[isize; 2]; 2]) -> isize {
    matrix[0][0] * matrix[1][1] - matrix[1][0] * matrix[0][1]
}

pub fn matrix_determinant(m: [[isize; 3]; 3]) -> isize {
    let a = m[0][0];
    let b = m[0][1];
    let c = m[0][2];
    a * matrix_determinant_2x2([[m[1][1], m[1][2]], [m[2][1], m[2][2]]])
        - b * matrix_determinant_2x2([[m[1][0], m[1][2]], [m[2][0], m[2][2]]])
        + c * matrix_determinant_2x2([[m[1][0], m[1][1]], [m[2][0], m[2][1]]])
}

fn main() {
    let matrix = [[1, 2, 4], [2, -1, 3], [4, 0, 1]];

    println!(
        "The determinant of the matrix:\n|1  2  4|\n|2 -1  3|  = {}\n|4  0  1|",
        matrix_determinant(matrix)
    );
}

// The determinant of the matrix:
// |1  2  4|
// |2 -1  3|  = 35
// |4  0  1|
