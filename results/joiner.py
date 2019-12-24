#!/usr/bin/env python3

import os
import itertools as it

def main():
    with open('whole.tsv', 'a') as writer:
        for f_name in filter(lambda f: f.endswith('.tsv') and f.startswith("simple_life"), os.listdir('.')):
            s = f_name.split('_')
            with open(f_name, 'r') as inp:
                writer.write("{},{},{},{}\t".format(s[2],s[3],s[4],s[5]))
                inside = inp.read()
                if inside and inside.endswith('\n'):
                    writer.write(inside)

if __name__ == "__main__":
    main()
