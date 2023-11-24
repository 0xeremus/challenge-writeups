#! /usr/bin/python3

import random


def main():
    x = ''
    for i in range(10):
        if i == 4:
            x += '@'
        else:
            x += chr(random.randrange(0x61, 0x7a))
    print(x)


if __name__=='__main__':
    main()
