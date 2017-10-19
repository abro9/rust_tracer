#!venv/bin/python3

import math
import numpy as np
import os

def write_scn(t, base_name, scn_count):
	f_name = base_name + "/" + base_name + str("%03d" % scn_count) + ".scn"

	s_x = math.cos(t)
	s_y = math.sin(t)

	with open(f_name, 'w') as f:
		f.write("m 0.3 0.8 0.3 0.5 0.5 0.5 500 0.0 0.0 0.0\n")
		f.write("s %f %f -4.0 0.5\n" % (s_x, s_y))

		f.write("m 0.2 0.2 0.2 0.0 0.0 0.0 5 1.0 1.0 1.0\n")
		f.write("s 0.0 -101.5 -4.0 100.0\n")

		f.write("l p 0.0 0.0 0.0 1.0 1.0 1.0\n")
		f.write("l p -7.0 3.5 5.0 40.0 40.0 40.0\n")
		f.write("l p 7.0 3.5 5.0 40.0 40.0 40.0\n")

		f.write("c 0.0 0.0 0.0 0.0 0.0 -1.0 1.0 4.0 2.0 800 400\n")

def write_all():
	all_t = np.arange(0, 2 * math.pi, 0.1)
	base = "orig"

	if not os.path.exists(base):
		os.makedirs(base)

	for i, t in enumerate(all_t):
		write_scn(t, base, i)

def render_all(scn_dir):
	if not os.path.exists(scn_dir):
		print("SCN dir does not exist!")
		exit(0)

	for scn in os.listdir(scn_dir):
		print(scn)


if __name__ == "__main__":
	# write_all()
	render_all("orig")


# def parse_scnx(filename):

