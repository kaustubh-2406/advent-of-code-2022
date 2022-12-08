def parse(input: str):
    mat = []
    lines = input.strip().splitlines()

    for line in lines:
        nums = []
        for char in line:
            nums.append(int(char))
        mat.append(nums)

    return mat


def solve(mat: [[int]]):
    ans = 0
    col_len = len(mat)
    row_len = len(mat[0])

    for i in range(1, col_len-1):
        for j in range(1, row_len-1):
            height = mat[i][j]
            top = all([mat[k][j] < height for k in range(i)])
            left = all([mat[i][k] < height for k in range(j)])
            right = all([mat[i][k] < height for k in range(j+1, row_len)])
            bottom = all([mat[k][j] < height for k in range(i+1, col_len)])

            if top or left or right or bottom:
                ans = ans + 1

    return ans + 2 * (row_len + col_len) - 4


if __name__ == "__main__":
    input = open('input.txt').read()
    mat = parse(input)  # array of array of ints..
    ans = solve(mat)
    print(ans)
