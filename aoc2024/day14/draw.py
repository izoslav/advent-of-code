import os
import matplotlib.pyplot as plt

dir = os.path.dirname(os.path.realpath(__file__)) + "/drawings"
files = [f for f in os.listdir(dir) if f.endswith(".txt")]

for filename in files:
    filepath = "{}/{}".format(dir, filename)
    outputpath = "{}/images/{}.png".format(dir, filename)

    table = [["0" for x in range(101)] for y in range(103)]

    with open(filepath, "r") as f:
        points = [[int(p[0]), int(p[1])] for p in [l.split(" ") for l in f.readlines()]]
        xs = [p[0] for p in points]
        ys = [p[1] for p in points]

        for point in points:
            table[point[1]][point[0]] = "1"

        output = "\n".join(["".join(line) for line in table])
        if "11111111" in output:
            print("Saving", filename)
            plt.figure()
            plt.plot(xs, ys, "go")
            plt.savefig(outputpath)
            plt.close()
