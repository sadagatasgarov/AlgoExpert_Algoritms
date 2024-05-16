def nonConstructibleChange(coins):
    coins.sort()
    currentChangeCreated = 0
    for coin in coins:
        if coin > currentChangeCreated +1:
            return currentChangeCreated+1
        currentChangeCreated += coin
    return currentChangeCreated + 1

a = nonConstructibleChange([5,7,1,1,2,3,22])
print(a)
