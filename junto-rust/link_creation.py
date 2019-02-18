"""Basic python code to show how linking of items will work
runs on assumption that 'thing_list' (list of channels/times) is already ordered correctly: alphabetically and then time ordered
"""

def get_first_links(current, thing_list):
    out = []
    for i, thing in enumerate(thing_list):
        if thing != current and i > thing_list.index(current):
            if current+thing not in out:
                out.append(current+":"+thing) 

            for n in thing_list[i:]:
                if n != thing:
                    out.append(current+":"+thing+":"+n)

    return out

thing_list = ["thing1", "thing2", "thing3", "thing4", "thing5"]
thing_linked = []

for thing in thing_list:
    current_links = get_first_links(thing, thing_list)
    for link in current_links:
        thing_linked.append(link)

    thing_linked.append("\n")

for x in thing_linked:
    print x

print len([x for x in thing_linked if x != "\n"])