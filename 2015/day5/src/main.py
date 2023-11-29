file = open(".\\src\\input.txt")
lines = file.read().split('\n')


vowels = set(['a', 'e', 'i', 'o', 'u'])
invalid_pairs = set(['ab', 'cd', 'pq', 'xy'])


nice_count = 0


def is_nice(line):
    vowel_count = len([x for x in line if x in vowels])
    vowel_count_met = vowel_count >= 3

    found_invaid_pair = any([pair in line for pair in invalid_pairs])

    found_dupes = False
    for c in set(line):
        if c + c in line:
            found_dupes = True

    return vowel_count_met and not found_invaid_pair and found_dupes


for line in lines:

    if is_nice(line):
        nice_count += 1

print(nice_count)
