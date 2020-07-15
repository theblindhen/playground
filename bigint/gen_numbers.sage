# Generate a json file with random numbers using SageMath
N = 100 # Number of pairs
B = 10  # Bitsize of numbers
json = "["
for i in range(N):
    a = randint(1, 2^B)
    b = randint(1, 2^B)
    json += '{ "a": "%s", "b": "%s"}' % (a,b)
    if i < N-1:
        json += ","
json += "]"
print(json)
