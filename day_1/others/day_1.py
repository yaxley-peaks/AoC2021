with open('../input.txt', 'r') as f:
    arr = list(map(int, f.read().splitlines()))

print(sum(1 for x in range(1, len(arr)) if arr[x] > arr[x-1]))
print(sum(1 for x in range(3, len(arr)) if arr[x] > arr[x-3]))