import redis

DATA = "data.txt"

r = redis.Redis(host='localhost', port=6379, db=0)

with open(DATA, 'r') as file:
    for line in file:
        key, value = line.strip().split(' ')
        r.set(key, value)

r.bgsave()