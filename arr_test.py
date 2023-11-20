a = [1, 2]


def change(a):
    a.append(3)
    return a


change(a.copy())
print(a)
