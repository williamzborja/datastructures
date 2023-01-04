def highest_freq(input: str) -> str:
    freq = {}  # unordered hash map

    for c in input:  # fill map with frequencies
        if c in freq:
            freq[c] += 1
        else:
            freq[c] = 1

    return max(freq, key=freq.get)


res = highest_freq("aab")
print(res)
