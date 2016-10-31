from itertools import permutations
N, M = map(int, input().split(' '))
rules = [list(map(int, input().split(' '))) for i in range(M)]
print(max([sum([k if t[i] < t[j] else 0 for i, j, k in rules]) for t in permutations(range(N), N)]))
