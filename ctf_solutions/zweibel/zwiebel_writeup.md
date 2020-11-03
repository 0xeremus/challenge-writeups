#### Problem: Zweibel Scripting Challenge 
#### Tools: Radare2

I saw on Liveroverflows channel that he covered a really cool binary challenge and thought I would take a crack at it. Zwiebel is a binary that takes an input string then initially maps a section of data (shellcode) into memory, then it jumps to this section.


           ; void*mmap(void*addr, size_t length, int prot, int flags, int fd, size_t offset)
           0x00400856      e8f5fdffff     call sym.imp.mmap
           0x0040085b      4989c6         mov r14, rax
           ; 0x601310
           0x0040085e      be10136000     mov esi, obj.shc
           0x00400863      ba8d4c0200     mov edx, 0x24c8d
           0x00400868      4c89f7         mov rdi, r14
           ; void *memcpy(void *s1, const void *s2, size_t n)
           0x0040086b      e820feffff     call sym.imp.memcpy
           0x00400870      4c89fb         mov rbx, r15
           0x00400873      31c0           xor eax, eax
           0x00400875      41ffd6         call r14  <- Jump occurs here
           0x00400878      31c0           xor eax, eax

After which it proceeds to and check individual bytes in the memory to see if they are 1 or zero. After each check it decrypts a portion of itself to continue the analysis.

            ;-- rip:
            0x7f6edc118000      8db600000000   lea esi, [rsi]
            0x7f6edc118006      8d742600       lea esi, [rsi]
            0x7f6edc11800a      8db426000000.  lea esi, [rsi]
            0x7f6edc118011      8dbc27000000.  lea edi, [rdi]
            0x7f6edc118018      4889d8         mov rax, rbx
            0x7f6edc11801b      8a4000         mov al, byte [rax]  ; this is the index into the key
            0x7f6edc11801e      2440           and al, 0x40        ; this is the bit that is being checked
        ,=< 0x7f6edc118020      7416           je 0x7f6edc118038   ; if the bit isn't there jump
        |   0x7f6edc118022      488d35340000.  lea rsi, [0x7f6edc11805d]
        |   0x7f6edc118029      ad             lodsd eax, dword [rsi]
        |   0x7f6edc11802a      4889c1         mov rcx, rax
        |   0x7f6edc11802d      ad             lodsd eax, dword [rsi]
       .--> 0x7f6edc11802e      3106           xor dword [rsi], eax  ; this is a loop where it unpacks the next section
       :|   0x7f6edc118030      4883c604       add rsi, 4
       `==< 0x7f6edc118034      e2f8           loop 0x7f6edc11802e
       ,==< 0x7f6edc118036      eb2d           jmp 0x7f6edc118065   ; this jumps to the next section of the code
       |`-> 0x7f6edc118038      683a280a00     push 0xa283a      ; this is an exit sequence
       |    0x7f6edc11803d      b801000000     mov eax, 1
       |    0x7f6edc118042      bf00000000     mov edi, 0
       |    0x7f6edc118047      4889e6         mov rsi, rsp
       |    0x7f6edc11804a      ba03000000     mov edx, 3   ;syscall 3 == exit()
       |    0x7f6edc11804f      0f05           syscall

Obviously there are also some checks to see if the bit in memory is NOT set, otherwise we could just provide a string of all ones. These are differentiated with a jne check rather then a je and a OR operation rather then a AND operation. This is a case that is acounted for in the solution script.

This is a solution script written in python using Radare2 (my favorite binary analysis tool). 

Solution Script:

    #!/usr/bin/python3
    # Inspired by liveoverflows solution to this challenge.


    import r2pipe
    import sys, os
    from pprint import pprint as pp

    bz = r2pipe.open('./zwiebel')
    key = [0x20] * 50
    key_answers = []

    def main():
        # commands to execute for setup gets to
        # the self modifying code section of the binary
        print("\n[+] Seeking to shell code\n")
        bz.cmd('e dbg.profile = profile.rr2')
        bz.cmd('ood')
        bz.cmd('db main+0x75')
        bz.cmd('dc')

        print("\n[+] Stepping into shell code\n")
        i = step()
        i = i[0]['opcode'].split(' ')[0]
        prv = i

        c = 1
        while (c>0):
         #step to the key comparison
           print("[+] Stepping to key comparison\n")
           while (i != 'je' and i != 'jne' and prv != 'and' and prv != 'or'):
                prv = i
                i = step()
                i = i[0]['opcode'].split(' ')[0]

            #correct flags
            print("[+] Setting zf accordingly\n")
            if i == 'je':
                bz.cmd('dr zf=0')
            elif i == 'jne':
                bz.cmd('dr zf=1')
            else:
                er = bz.cmd('pd 1')
                print(f'ERROR at {er}\n')

            #get value to mutate key with
            print("\n[+] Getting key value\n")
            bz.cmd('s- 2')
            x, y, z = bz.cmdj('pdj 1')[0]['opcode'].split(' ')

            print(x, z)

            #get index for mutation
            print("[+] Getting index for mutation\n")
            bz.cmd('s- 3')
            index = bz.cmdj('pdj 1')[0]['opcode'].split(' ')[-1]
            DEBUG
            print(index)
            if index == '[rax]':
                index = 0
            else:
                index = index.strip(']')
                index = int(index, 16)

            print("[+] Applying key value to key")
            z = int(z, 16)
            if i == 'je':
                key[index] = key[index] | z
            if i == 'jne':
                key[index] = key[index] & (0xFF ^ z)
            print(key)

            print('[+] Finding jump condition to next comparison\n')

            i = seek(1)

            print(i)
            i = i[0]['disasm'].split(' ')[0]
            prv = ''

            count = 2
            while (i != 'jmp' and prv != 'loop'):
                prv = i
                i = seek(count)
                i = i[0]['disasm'].split(' ')[0]

            print(f'Prev = {prv} and i = {i}')
                count += 1

            print('Found next jump at {}'.format(bz.cmd('pd 1')))
            add = str(bz.cmd('s:').strip(b'\n').decode())

            print(add)
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


When I initially wrote this script I was trying to operate based of the condition of the and/or statement and ended up with nonsense strings, where I either slowly set all the bytes in the string to ascii value that were too high. When I went and had a look at the script liveoverflow constructed I realized that the operations need to occur based of the jne/je condition and everything was golden. I also borrowed how to deal with the 'or' operations from them. So credit where credit is due.
