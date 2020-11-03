#!/usr/bin/python3

import r2pipe
from pprint import pprint as pp

b = r2pipe.open('./D4RK_FL0W-3.5')

def main():

    #open and instantiate debugging
    print("[+] Setting up debugging\n")
    com('e dbg.profile = run.rr2')
    com('ood')
    com('aa')

    #set breakpoints at needed comparisons
    print("[+] Setting break points on important functions\n")
    offsets = ['b0', 'c9', 'e2', 'fb']
    for i in offsets:
        b.cmd('s sym.Password::check_passcode+0x{}'.format(i))
        current_add = b.cmd('s')
        current_add = current_add.decode().strip('\n')
        b.cmd('db {}'.format(current_add))
        pp(b.cmd('pd 1'))

    #Retrieve password
    print("[+] Retrieving password\n")
    b.cmd('dc')
    password = []
    #loop through 4 times and get password comparison figures
    for i in range(0, 4):
        correct = pcom('dr eax')
        correct = int(correct.strip(b'\n').strip(b'0x0'))
        print(correct)
        password.append(correct)
        com('dr rdx={}'.format(correct))
        com('dc')

    print(password)
    b.cmd('dc')

    return 0

def com(x):
    b.cmd(x)

def pcom(x):
    output = b.cmd(x)
    pp(output)
    return output

if __name__=='__main__':
    main()
