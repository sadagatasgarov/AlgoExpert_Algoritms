
def transposeMatrix(matrix):
    transposedMatrix = []
    for col in range(len(matrix)):
        newRow = []
        for row in range(len(matrix)):
            newRow.append(matrix[row][col])
        transposedMatrix.append(newRow)
    return transposedMatrix


print(transposeMatrix([[1,2,3],[4,5,6],[7,8,9]]))