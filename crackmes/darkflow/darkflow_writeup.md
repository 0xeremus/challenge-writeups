##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem: D4rkflow3.5
#### Solution: 4 2 7 3

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Explanation:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

main function gets password with a call to get input function and then calls function to check password. Based off the reversed assembly and comments I wrote a script to retrieve the password.

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### radare2 Script:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

``` python
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
```

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
```asm
/ (fcn) main 77
|   int main (int argc, char **argv, char **envp);
|           ; DATA XREF from entry0 (0x10bd)
|           0x00001427      55             push rbp
|           0x00001428      4889e5         mov rbp, rsp
|           ; 0x42a0
|           0x0000142b      488d3d6e2e00.  lea rdi, obj.p
|           ; "gets password from standard input, stores in obj.p"
|           0x00001432      e8a7000000     call sym.Password::get_input
|           ; 0x42a0
|           0x00001437      488d3d622e00.  lea rdi, obj.p
|           ; "takes input, tests it, returns 1 if correct, zero otherwise"
|           0x0000143e      e8d7feffff     call sym.Password::check_passcode
|           0x00001443      84c0           test al, al
|       ,=< 0x00001445      7413           je 0x145a
|       |   ; "Prints if password was correct, and check_password returned 1"
|       |   ; 0x2069
|       |   ; "\nWell done"
|       |   0x00001447      488d351b0c00.  lea rsi, str.Well_done
|       |   ; sym..bss
|       |   ; 0x4060
|       |   0x0000144e      488d3d0b2c00.  lea rdi, obj.std::cout
|       |   0x00001455      e8f6fbffff     call sym.std::basic_ostream_char_std::char_traits_char___std::operator___std::char_traits_char___std::basic_ostream_char_std::char_traits_char____charconst
|       |   ; "Prints either way"
|       |   ; 0x2074
|       |   ; "\n\nSORRY MAYBE NEXT TIME!!"
|       `-> 0x0000145a      488d35130c00.  lea rsi, str.SORRY_MAYBE_NEXT_TIME
|           ; sym..bss
|           ; 0x4060
|           0x00001461      488d3df82b00.  lea rdi, obj.std::cout
|           0x00001468      e8e3fbffff     call sym.std::basic_ostream_char_std::char_traits_char___std::operator___std::char_traits_char___std::basic_ostream_char_std::char_traits_char____charconst
|           0x0000146d      b800000000     mov eax, 0
|           0x00001472      5d             pop rbp
\           0x00001473      c3             ret


------------------------------------------------------

;-- method.Password.check_passcode:
/ (fcn) sym.Password::check_passcode 269
|   sym.Password::check_passcode (int32_t arg1);
|           ; var int32_t var_8h @ rbp-0x8
|           ; arg int32_t arg1 @ rdi
|           ; CALL XREF from main (0x143e)
|           0x0000131a      55             push rbp
|           0x0000131b      4889e5         mov rbp, rsp
|           0x0000131e      4883ec10       sub rsp, 0x10
|           ; arg1
|           0x00001322      48897df8       mov qword [var_8h], rdi
|           ; "\n\n"
|           0x00001326      488d35390d00.  lea rsi, [0x00002066]
|           ; sym..bss
|           ; 0x4060
|           0x0000132d      488d3d2c2d00.  lea rdi, obj.std::cout
|           0x00001334      e817fdffff     call sym.std::basic_ostream_char_std::char_traits_char___std::operator___std::char_traits_char___std::basic_ostream_char_std::char_traits_char____charconst
|           0x00001339      4889c2         mov rdx, rax
|           0x0000133c      488b45f8       mov rax, qword [var_8h]
|           ; [0x10:4]=0x3e0003
|           0x00001340      8b4010         mov eax, dword [rax + 0x10]
|           0x00001343      89c6           mov esi, eax
|           0x00001345      4889d7         mov rdi, rdx
|           0x00001348      e833fdffff     call sym.std::ostream::operator___int
|           0x0000134d      be0a000000     mov esi, 0xa
|           0x00001352      4889c7         mov rdi, rax
|           0x00001355      e806fdffff     call sym.std::basic_ostream_char_std::char_traits_char___std::operator___std::char_traits_char___std::basic_ostream_char_std::char_traits_char____char
|           0x0000135a      4889c2         mov rdx, rax
|           0x0000135d      488b45f8       mov rax, qword [var_8h]
|           ; [0x14:4]=1
|           0x00001361      8b4014         mov eax, dword [rax + 0x14]
|           0x00001364      89c6           mov esi, eax
|           0x00001366      4889d7         mov rdi, rdx
|           0x00001369      e812fdffff     call sym.std::ostream::operator___int
|           0x0000136e      be0a000000     mov esi, 0xa
|           0x00001373      4889c7         mov rdi, rax
|           0x00001376      e8e5fcffff     call sym.std::basic_ostream_char_std::char_traits_char___std::operator___std::char_traits_char___std::basic_ostream_char_std::char_traits_char____char
|           0x0000137b      4889c2         mov rdx, rax
|           0x0000137e      488b45f8       mov rax, qword [var_8h]
|           ; [0x18:4]=0x10a0 rip
|           0x00001382      8b4018         mov eax, dword [rax + 0x18]
|           0x00001385      89c6           mov esi, eax
|           0x00001387      4889d7         mov rdi, rdx
|           0x0000138a      e8f1fcffff     call sym.std::ostream::operator___int
|           0x0000138f      be0a000000     mov esi, 0xa
|           0x00001394      4889c7         mov rdi, rax
|           0x00001397      e8c4fcffff     call sym.std::basic_ostream_char_std::char_traits_char___std::operator___std::char_traits_char___std::basic_ostream_char_std::char_traits_char____char
|           0x0000139c      4889c2         mov rdx, rax
|           0x0000139f      488b45f8       mov rax, qword [var_8h]
|           ; [0x1c:4]=0
|           0x000013a3      8b401c         mov eax, dword [rax + 0x1c]
|           0x000013a6      89c6           mov esi, eax
|           0x000013a8      4889d7         mov rdi, rdx
|           0x000013ab      e8d0fcffff     call sym.std::ostream::operator___int
|           0x000013b0      be0a000000     mov esi, 0xa
|           0x000013b5      4889c7         mov rdi, rax
|           0x000013b8      e8a3fcffff     call sym.std::basic_ostream_char_std::char_traits_char___std::operator___std::char_traits_char___std::basic_ostream_char_std::char_traits_char____char
|           0x000013bd      488b45f8       mov rax, qword [var_8h]
|           ; "Checks first number"
|           ; [0x10:4]=0x3e0003
|           0x000013c1      8b5010         mov edx, dword [rax + 0x10]
|           0x000013c4      488b45f8       mov rax, qword [var_8h]
|           0x000013c8      8b00           mov eax, dword [rax]
|           0x000013ca      39c2           cmp edx, eax
|       ,=< 0x000013cc      7407           je 0x13d5
|       |   0x000013ce      b800000000     mov eax, 0
|      ,==< 0x000013d3      eb50           jmp 0x1425
|      |`-> 0x000013d5      488b45f8       mov rax, qword [var_8h]
|      |    ; "Checks third number"
|      |    ; [0x18:4]=0x10a0 rip
|      |    0x000013d9      8b5018         mov edx, dword [rax + 0x18]
|      |    0x000013dc      488b45f8       mov rax, qword [var_8h]
|      |    ; [0x8:4]=0
|      |    0x000013e0      8b4008         mov eax, dword [rax + 8]
|      |    0x000013e3      39c2           cmp edx, eax
|      |,=< 0x000013e5      7407           je 0x13ee
|      ||   0x000013e7      b800000000     mov eax, 0
|     ,===< 0x000013ec      eb37           jmp 0x1425
|     ||`-> 0x000013ee      488b45f8       mov rax, qword [var_8h]
|     ||    ; "Checks second number"
|     ||    ; [0x14:4]=1
|     ||    0x000013f2      8b5014         mov edx, dword [rax + 0x14]
|     ||    0x000013f5      488b45f8       mov rax, qword [var_8h]
|     ||    ; [0x4:4]=0x10102
|     ||    0x000013f9      8b4004         mov eax, dword [rax + 4]
|     ||    0x000013fc      39c2           cmp edx, eax
|     ||,=< 0x000013fe      7407           je 0x1407
|     |||   0x00001400      b800000000     mov eax, 0
|    ,====< 0x00001405      eb1e           jmp 0x1425
|    |||`-> 0x00001407      488b45f8       mov rax, qword [var_8h]
|    |||    ; "Checks fourth number"
|    |||    ; [0x1c:4]=0
|    |||    0x0000140b      8b501c         mov edx, dword [rax + 0x1c]
|    |||    0x0000140e      488b45f8       mov rax, qword [var_8h]
|    |||    ; [0xc:4]=0
|    |||    0x00001412      8b400c         mov eax, dword [rax + 0xc]
|    |||    0x00001415      39c2           cmp edx, eax
|    |||,=< 0x00001417      7407           je 0x1420
|    ||||   0x00001419      b800000000     mov eax, 0
|   ,=====< 0x0000141e      eb05           jmp 0x1425
|   |||||   ; "returns one if password was correct 0 otherwise"
|   ||||`-> 0x00001420      b801000000     mov eax, 1
|   ||||    ; CODE XREFS from sym.Password::check_passcode (0x13d3, 0x13ec, 0x1405, 0x141e)
|   ````--> 0x00001425      c9             leave
\           0x00001426      c3             ret
```
