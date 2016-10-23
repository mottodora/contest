from collections import defaultdict
memo = {1:1}

def sumbinary(n):
    if n == 0:
        return 0
    elif n % 2 == 0:
        return sumbinary(n//2)
    else:
        return sumbinary(n//2) + 1

target = int(input())

Q = [(1, 1)]

while True:
    if len(Q) == 0:
        result = -1
        break
    else:
        now, walk = Q.pop(0)
        if now == target:
            result = walk
            break
        d = sumbinary(now)
        if 0 < now-d <= target and not(now-d in memo):
            memo[now-d] = walk+1
            Q.append((now-d, walk+1))

        if 0 < now+d <= target and not(now+d in memo):
            memo[now+d] = walk+1
            Q.append((now+d, walk+1))

print(result)
