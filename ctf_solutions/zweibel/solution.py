#!/usr/bin/python3
# Inspired by liveoverflows video and solution to this challenge. It took me
# forever to write this but luckily was an awesome to learn using r2pipe to script radare.


import r2pipe
import sys, os
from pprint import pprint as pp

bz = r2pipe.open('./zwiebel')
key = [0x20] * 50
key_answers = []

def main():
    # commands to execute for setup gets to
    # the self modifying code section of the binary
#    print("\n[+] Seeking to shell code\n")
    bz.cmd('e dbg.profile = profile.rr2')
    bz.cmd('ood')
    bz.cmd('db main+0x75')
    bz.cmd('dc')

#    print("\n[+] Stepping into shell code\n")
    i = step()
    i = i[0]['opcode'].split(' ')[0]
    prv = i

    c = 1
    while (c>0):
        #step to the key comparison
#        print("[+] Stepping to key comparison\n")
        while (i != 'je' and i != 'jne' and prv != 'and' and prv != 'or'):
            prv = i
            i = step()
            i = i[0]['opcode'].split(' ')[0]

        #correct flags
#        print("[+] Setting zf accordingly\n")
        if i == 'je':
            bz.cmd('dr zf=0')
        elif i == 'jne':
            bz.cmd('dr zf=1')
        else:
            er = bz.cmd('pd 1')
            print(f'ERROR at {er}\n')

        #get value to mutate key with
#        print("\n[+] Getting key value\n")
        bz.cmd('s- 2')
        x, y, z = bz.cmdj('pdj 1')[0]['opcode'].split(' ')
#DEBUG
#        print(x, z)

        #get index for mutation
#        print("[+] Getting index for mutation\n")
        bz.cmd('s- 3')
        index = bz.cmdj('pdj 1')[0]['opcode'].split(' ')[-1]
#DEBUG
#        print(index)
        if index == '[rax]':
            index = 0
        else:
            index = index.strip(']')
            index = int(index, 16)

#        print("[+] Applying key value to key")
        z = int(z, 16)
        if i == 'je':
            key[index] = key[index] | z
        if i == 'jne':
            key[index] = key[index] & (0xFF ^ z)
#DEBUG
#       print(key)

#        print('[+] Finding jump condition to next comparison\n')

        i = seek(1)
#DEBUG
#        print(i)
        i = i[0]['disasm'].split(' ')[0]
        prv = ''

        count = 2
        while (i != 'jmp' and prv != 'loop'):
            prv = i
            i = seek(count)
            i = i[0]['disasm'].split(' ')[0]
#DEBUG
#            print(f'Prev = {prv} and i = {i}')
            count += 1

#        print('Found next jump at {}'.format(bz.cmd('pd 1')))
        add = str(bz.cmd('s:')).strip(b'\n').decode()
#DEBUG
#        print(add)
        bz.cmd(f'db {add}')
        bz.cmd('dc')
        pp(bz.cmd('pd 1'))

        output = ''
        os.system('clear')
        for i in key:
            if i > 0x19 & i < 0x7b:
                output += chr(i)
            else:
                output += '_'
        print(output)


def step():
    bz.cmd('ds')
    bz.cmd('s rip')
    return bz.cmdj('pdj 1')

def seek(n):
    bz.cmd(f'so {n}')
    return bz.cmdj('pdj 1')


if __name__=='__main__':
    main()
