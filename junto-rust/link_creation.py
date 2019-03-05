"""Basic python code to show how linking of items will work
runs on assumption that 'thing_list' (list of channels/times) is already ordered correctly: alphabetically and then time ordered
"""
import itertools

link_list = ["channel1", "channel2", "channel3", "user", "time1", "time2", "time3", "time4", "type"]
out = []
for L in range(0, len(link_list)+1):
    for subset in itertools.combinations(link_list, L):
        print subset
        out.append(subset)

print len(out)