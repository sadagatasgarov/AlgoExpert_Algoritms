fn main() {
    let mat = vec![vec![1,2,3], vec![4,5,6], vec![7,8,9]];
    transpose_matrix(&mat);
    transpose(mat);
}


fn transpose_matrix(matrix: &Vec<Vec<i32>>) {
    let mut transposed_matrix = vec![vec![]];
    for ridx in 0..matrix.len(){
        let mut rmat = vec![];
        for cidx in 0..matrix.len(){
            rmat.push(matrix[cidx][ridx]);
            //println!("{:?}", rmat)
        }
        transposed_matrix.push(rmat)
    }
    println!("{:?}",transposed_matrix);
}


fn transpose(matrix: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
    let mut transposed_matrix = vec![vec![0; matrix.len()]; matrix[0].len()];
    for i in 0..matrix.len() {
        for j in 0..matrix[0].len() {
            transposed_matrix[j][i] = matrix[i][j];
        }
    }
    println!("{:?}",transposed_matrix);

    transposed_matrix
}