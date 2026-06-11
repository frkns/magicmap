import random

N = 333

st: set[int] = set()

while len(st) < N:
    st.add(random.getrandbits(64))

with open('in', 'w') as f:
    f.write('\n'.join(map(str, st)))
