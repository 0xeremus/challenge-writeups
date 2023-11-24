#! /usr/bin/python3
import subprocess, random, sys


def main():

    for i in range(10):

        key = ''

        # Generate first 15 characters
        for i in range(15):
            key += random.choice("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")

        # Find the last element of the keys
        key = last_element(key)

        if key:
            key = check_key(key)
            if key:
                print("Validated Key: ", key)

        # if want the key without checking validating it with the binary
#        if key:
#            print("Non-validated key: ", key)


def last_element(key):

    # initialize key_sum
    key_sum = 0

    # sum the key elements
    for i in key:
        key_sum += ord(i)
        key_sum = key_sum >> 1
        key_sum = key_sum % 0xf00
        key_sum += 0xa

    # check that key is valid
    if chr(key_sum) in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ":
        key += chr(key_sum)
        return key
    else:
        return 0


def check_key(key):

    # Expected failure code from program
    failure = (b'Sorry, the code was invalid.\n', None)

    # open binary and give it key to validate, relative script and binary must be in same folder
    p = subprocess.Popen(["./keyGme", key.encode()], stdout=subprocess.PIPE, stdin=subprocess.PIPE)

    # get message
    stdout = p.communicate(input=None, timeout=5)

    # check if key was correct
    if stdout != failure:
        return key
    else:
        return 0


if __name__=='__main__':
    main()
