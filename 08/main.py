import functools


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
    scenic_score = 0
    col_len = len(mat)
    row_len = len(mat[0])

    for i in range(1, col_len-1):
        for j in range(1, row_len-1):
            height = mat[i][j]
            te = [mat[k][j] for k in range(i)]
            le = [mat[i][k] for k in range(j)]
            re = [mat[i][k] for k in range(j+1, row_len)]
            be = [mat[k][j] for k in range(i+1, col_len)]

            top = all([h < height for h in te])
            left = all([h < height for h in le])
            right = all([h < height for h in re])
            bottom = all([h < height for h in be])

            # part 1
            if top or left or right or bottom:
                ans = ans + 1

            # part 2
            def fn(tup, val):
                (acc, cond) = tup
                if val < height:
                    return (acc + 1, True)
                return (acc, cond)

            (ts, _) = functools.reduce(fn, te, (0, True))
            (ls, _) = functools.reduce(fn, le, (0, True))
            (rs, _) = functools.reduce(fn, re, (0, True))
            (bs, _) = functools.reduce(fn, be, (0, True))

            newscore = ts * rs * bs * ls
            scenic_score = max(scenic_score, newscore)
            return ans + 2 * (row_len + col_len) - 4, scenic_score


if __name__ == "__main__":
    input = open('sample.txt').read()
    mat = parse(input)  # array of array of ints..
    ans, ans2 = solve(mat)
    print(ans, ans2)
