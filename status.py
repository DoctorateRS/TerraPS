with open("LINKS.md", "r") as list:
    links = list.readlines()

notDone = 0
done = 0
for link in links:
    if link.startswith("- [ ]"):
        notDone += 1
    elif link.startswith("- [x]"):
        done += 1

percentage = done / (done + notDone) * 100
print(f"{done} done, {notDone} not done, {percentage:.2f}% done")
