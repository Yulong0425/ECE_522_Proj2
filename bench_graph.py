import matplotlib.pyplot as plt
import numpy as np
from numpy.linalg import norm

avl_baseline_ins_res = [3.8721, 17.934, 32.461, 48.502, 64.176] #ms
avl_baseline_search_res = [52.882, 290.25, 678.79, 835.45, 1156.7] #us

avl_ins_res = [1.9749, 8.7007, 16.736, 23.526, 34.149] #ms
avl_search_res = [62.971, 300.81, 533.49, 783.13, 1023.7] #us


rb_baseline_ins_res = [11.638, 61.780, 121.39, 181.46, 244.64] #ms
rb_baseline_search_res = [40.127, 244.32, 468.19, 688.72, 912.02] #us

rb_ins_res = [2.2894, 10.589, 19.381, 29.538, 37.954] #ms
rb_search_res = [37.654, 182.85, 339.36, 515.37, 671.71] #us


def draw_insertion():
    x = np.linspace(0, 4, 5)

    plt.plot(x, avl_baseline_ins_res, label="AVL baseline insertion")
    plt.plot(x, avl_ins_res, label="AVL insertion")
    plt.plot(x, rb_baseline_ins_res, label="RB baseline insertion")
    plt.plot(x, rb_ins_res, label="RB insertion")
    plt.ylabel("time(ms)")
    plt.xlabel("input size")
    plt.xticks([0, 1, 2, 3, 4],["10,000", "40,000", "70,000", "100,000", "130,000"])
    plt.legend()
    plt.savefig("insertion.png")

def draw_search():
    x = np.linspace(0, 4, 5)

    plt.plot(x, avl_baseline_search_res, label="AVL baseline search")
    plt.plot(x, avl_search_res, label="AVL search")
    plt.plot(x, rb_baseline_search_res, label="RB baseline search")
    plt.plot(x, rb_search_res, label="RB search")
    plt.ylabel("time(us)")
    plt.xlabel("input size")
    plt.xticks([0, 1, 2, 3, 4],["1,000", "4,000", "7,000", "10,000", "13,000"])
    plt.legend()
    plt.savefig("search.png")

# draw_insertion()
draw_search()