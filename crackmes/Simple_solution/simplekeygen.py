#! /usr/bin/python3
#
#Solution to Yuri's SimpleKeyGen0

import random, sys


def main():
    if len(sys.argv) == 2:
        count = int(sys.argv[1])
    else:
        count = 1

    key = ''
    for i in range(0, count):
        while(len(key) != 16):
            x = random.randrange(0x61, 0x7a)
            key += chr(x)
            key += chr(x + 1)
        print(key)
        key = ''


if __name__=='__main__':
    main()

