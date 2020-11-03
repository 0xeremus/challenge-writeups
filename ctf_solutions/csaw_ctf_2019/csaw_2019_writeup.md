## Csaw 2019 
### Write up for RE challenges.
### Challenge: **beleaf** re challenge
##### Solution: flag{we_beleaf_in_your_re_future}
##### Tools: radare2

##### _Explanation:_ 

String length was checked and had to be more then 32. One function mapped input character to a value by walking a data structure based on the input. By taking these values and feeding these values to the function we can get a mapping

The main function checked that value against the known flag value. We simply then walk this and setting the value to the expected one each time and we have the flag.

    {1: 'f',
     9: 'l',
     11: 'a',
     39: 'g',
     2: '{',
     0: 'w',
     4: 'n',
     3: '_',
     5: 'y',
     6: '}',
     8: 'b',
     10: 'r',
     12: 'e',
     13: 'i',
     15: 'o',
     46: 'u',
     22: 't'}

##### _Radare2 Script:_

    db main+0xd5 <- once we have map, we just want the rax value from here.

    dc; dr rax; ds; dr zf=1 <- gets value, then we continue.

    feed values recovered into the map and bam

    flag{we_beleaf_in_your_re_future}

##### _Commented Assembly:_


        | |           ; size_t strlen(const char *s)
        | |           0x55c34ccb88fe      e88dfdffff     call sym.imp.strlen;[1]
        | |           0x55c34ccb8903      48898560ffff.  mov qword [var_a0h], rax
        | |           0x55c34ccb890a      4883bd60ffff.  cmp qword [var_a0h], 0x20
        | |       ,=< 0x55c34ccb8912      7716           ja 0x55c34ccb892a
        | |       |   ; 0x55c34ccb8a7b
        | |       |   ; "Incorrect!"
        | |       |   0x55c34ccb8914      488d3d600100.  lea rdi, str.Incorrect
        | |       |   ; int puts(const char *s)
        | |       |   0x55c34ccb891b      e860fdffff     call sym.imp.puts;[2]
        | |       |   0x55c34ccb8920      bf01000000     mov edi, 1
        | |       |   ; void exit(int status)
        | |       |   0x55c34ccb8925      e8a6fdffff     call sym.imp.exit;[3]
        | |       `-> 0x55c34ccb892a      48c78558ffff.  mov qword [var_a8h], 0
        | |       ,=< 0x55c34ccb8935      eb66           jmp 0x55c34ccb899d
        | |      .--> 0x55c34ccb8937      488d9570ffff.  lea rdx, [var_90h]
        | |      :|   0x55c34ccb893e      488b8558ffff.  mov rax, qword [var_a8h]
        | |      :|   0x55c34ccb8945      4801d0         add rax, rdx
        | |      :|   0x55c34ccb8948      0fb600         movzx eax, byte [rax]
        | |      :|   0x55c34ccb894b      0fbec0         movsx eax, al
        | |      :|   0x55c34ccb894e      89c7           mov edi, eax
        THIS IS WHERE THE RETURN VALUE IS FROM
        | |      :|   0x55c34ccb8950      e8a5feffff     call fcn.55c34ccb87fa;[4]
        | |      :|   0x55c34ccb8955      48898568ffff.  mov qword [var_98h], rax
        | |      :|   0x55c34ccb895c      488b8558ffff.  mov rax, qword [var_a8h]
        | |      :|   0x55c34ccb8963      488d14c50000.  lea rdx, [rax*8]
        | |      :|   0x55c34ccb896b      488d056e0b20.  lea rax, [0x55c34ceb94e0]
        | |      :|   0x55c34ccb8972      488b0402       mov rax, qword [rdx + rax]
        | |      :|   0x55c34ccb8976 b    48398568ffff.  cmp qword [var_98h], rax
        THIS IS WHERE WE CAN RECOVER THE FLAG VALUE
        | |      :|   ;-- rip:
        | |     ,===< 0x55c34ccb897d      7416           je 0x55c34ccb8995
        | |     |:|   ; 0x55c34ccb8a7b
        | |     |:|   ; "Incorrect!"
        | |     |:|   0x55c34ccb897f      488d3df50000.  lea rdi, str.Incorrect
        | |     |:|   ; int puts(const char *s)
        | |     |:|   0x55c34ccb8986      e8f5fcffff     call sym.imp.puts;[2]
        | |     |:|   0x55c34ccb898b      bf01000000     mov edi, 1
        | |     |:|   ; void exit(int status)
        | |     |:|   0x55c34ccb8990      e83bfdffff     call sym.imp.exit;[3]
        | |     `---> 0x55c34ccb8995      48838558ffff.  add qword [var_a8h], 1
        | |      :|   ; CODE XREF from main @ 0x55c34ccb8935
        | |      :`-> 0x55c34ccb899d      488b8558ffff.  mov rax, qword [var_a8h]
        | |      :    0x55c34ccb89a4      483b8560ffff.  cmp rax, qword [var_a0h]
        | |      `==< 0x55c34ccb89ab      728a           jb 0x55c34ccb8937
        | |           ; 0x55c34ccb8a86
        | |           ; "Correct!"


#### Challenge: **callsite** re challenge
##### Solution: flag{you_got_the_call_site} 
##### Tools: radare2, strace

##### _Explanation:_ 

This is a mass of functions in this binary, open it up in r2 and analyse it and there are hundreds of functions.

strace it and you see it needs two arguments, one an address the other a key, what address?

run strings on the binary seeking a flag

:> izz~flag
7193 0x000ba239 0x004ba239   8   9 (.rodata) ascii flag.txt
...

seek to location and get a cross reference
;-- "flag.txt":
; DATA XREF from prints_flag @ 0x400cc2
0x004ba239     .string "flag.txt" ; len=9

we know it is called at this address (i defined and named a function there) so now we pass the binary this address argument.

We get told there is not file named flag.txt, so we make one and wow we get the flag, and we get told after it is printed that we got the key wrong. Now we can try to pass this to the challenge server and

flag{you_got_the_call_site}

we get the flag.

#### Challenge: **gibberish** re challenge
##### Solution: flag{first_ever_challenge}
##### Tools: radare2, strace, pwntools

##### _Explanation:_

First thing the binary does is prevent ptrace from attaching. Stop that by making it return immediately from ptrace

        wa ret @ sym.imp.ptrace

Then in the first function called the binary calls ptrace on itself and if that returns null it exits. Alright so we make exit fail.

        wa ret @ sym.imp.exit

okay. Now the binary maps a bunch of strings into memory.


        140 0x000058e5 0x000058e5   8   9 (.rodata) ascii Correct!
        141 0x000058ee 0x000058ee   8   9 (.rodata) ascii Wrong D:
        142 0x000058f7 0x000058f7   8   9 (.rodata) ascii flag.txt
        143 0x00005900 0x00005900  20  21 (.rodata) ascii dqzkenxmpsdoe_qkihmd
        144 0x00005915 0x00005915  20  21 (.rodata) ascii jffglzbo_zghqpnqqfjs
        145 0x0000592a 0x0000592a  20  21 (.rodata) ascii kdwx_vl_rnesamuxugap
        146 0x0000593f 0x0000593f  20  21 (.rodata) ascii ozntzohegxagreedxukr
        147 0x00005954 0x00005954  20  21 (.rodata) ascii xujaowgbjjhydjmmtapo
        148 0x00005969 0x00005969  20  21 (.rodata) ascii pwbzgymqvpmznoanomzx
        149 0x0000597e 0x0000597e  20  21 (.rodata) ascii qaqhrjofhfiuyt_okwxn
        150 0x00005993 0x00005993  20  21 (.rodata) ascii a_anqkczwbydtdwwbjwi
        151 0x000059a8 0x000059a8  20  21 (.rodata) ascii zoljafyuxinnvkxsskdu
        152 0x000059bd 0x000059bd  20  21 (.rodata) ascii irdlddjjokwtpbrrr_yj
        153 0x000059d2 0x000059d2  20  21 (.rodata) ascii cecckcvaltzejskg_qrc
        154 0x000059e7 0x000059e7  20  21 (.rodata) ascii vlpwstrhtcpxxnbbcbhv
        155 0x000059fc 0x000059fc  20  21 (.rodata) ascii spirysagnyujbqfhldsk
        156 0x00005a11 0x00005a11  20  21 (.rodata) ascii bcyqbikpuhlwordznpth
        157 0x00005a26 0x00005a26  20  21 (.rodata) ascii _xkiiusddvvicipuzyna
        158 0x00005a3b 0x00005a3b  20  21 (.rodata) ascii wsxyupdsqatrkzgawzbt
        159 0x00005a50 0x00005a50  20  21 (.rodata) ascii ybg_wmftbdcvlhhidril
        160 0x00005a65 0x00005a65  20  21 (.rodata) ascii ryvmngilaqkbsyojgify
        161 0x00005a7a 0x00005a7a  20  21 (.rodata) ascii mvefjqtxzmxf_vcyhelf
        162 0x00005a8f 0x00005a8f  20  21 (.rodata) ascii hjhofxwrk_rpwli_mxv_
        163 0x00005aa4 0x00005aa4  20  21 (.rodata) ascii enupmannieqqzcyevs_w
        164 0x00005ab9 0x00005ab9  20  21 (.rodata) ascii uhmvvb_cfgjkggjpavub
        165 0x00005ace 0x00005ace  20  21 (.rodata) ascii gktdphqiswomuwzvjtog
        166 0x00005ae3 0x00005ae3  20  21 (.rodata) ascii lgoehepwclbaifvtfoeq
        167 0x00005af8 0x00005af8  20  21 (.rodata) ascii nm_uxrukmof_fxsfpcqz
        168 0x00005b0d 0x00005b0d  20  21 (.rodata) ascii ttsbclzyyuslmutcylcm
        169 0x00005b22 0x00005b22  13  14 (.rodata) ascii Find the Key!

Then it asks for a key.

I used these scripts/commands to step through the binary looking at the relationships, etc.


            db main+0x92a; db main+0x8b5; dc; dr rax; dc; px 4 @ rbp-0x40c
            dc; px 4 @ rbp-0x40c;


The key is then somehow scored against these strings, and for each string you get 20 or 19. This all seemed quite dense and so after playing around for a minute and realizing that there was slight discrepencies in scoring and that the score had to equal 0x1f9, most characters I entered were coming back between ~0x205 to ~0x1c0. I can brute force this.

      `-> 0x000026c2      81bdf4fbffff.  cmp dword [var_40ch], 0x1f9

So I joined all the strings and wrote a script to brute force it. Then just netcat into server and submit flag done!            

Find the Key!
xzfntjdpgkwuwsozihltw
Correct!
flag{first_ever_challenge}

##### _Solution Script:_

     import random
     from pwn import *
     ans = "Wrong D:\n"
     while(ans == "Wrong D:\n"):
         proc = process('./gib_mod')
         print(proc.recvline())
         proc.sendline(''.join(random.sample(t, 5)))
         ans = proc.recvline()
         print(ans)
         proc.close()


    Script Results:

    [*] Process './gib_mod' stopped with exit code 0 (pid 25887)
    [x] Starting local process './gib_mod'
    [+] Starting local process './gib_mod': pid 25888
    Find the Key!

    mynng
    Wrong D:

    [*] Stopped process './gib_mod' (pid 25888)
    [x] Starting local process './gib_mod'
    [+] Starting local process './gib_mod': pid 25889
    Find the Key!

    pnpttuunzxbcg
    Wrong D:

    [*] Process './gib_mod' stopped with exit code 0 (pid 25889)
    [x] Starting local process './gib_mod'
    [+] Starting local process './gib_mod': pid 25890
    Find the Key!

    xzfntjdpgkwuwsozihltw
    Correct!

    [*] Stopped process './gib_mod' (pid 25890)

    Then just netcat into server and submit flag done!            

    Find the Key!
    xzfntjdpgkwuwsozihltw
    Correct!
    flag{first_ever_challenge}


#### _Commented Assembly:_


    |      |:   0x00002648      e8fdefffff     call fcn.0000164a
    |      |:   0x0000264d      0185f4fbffff   add dword [var_40ch], eax <- summing here
    |      |:   0x00002653      488d8570fcff.  lea rax, [var_390h]
                ....

    |       ,=< 0x000026aa      7416           je 0x26c2
    |       |   ; const char *s
    |       |   ; "Rip"
    |       |   0x000026ac      488d3d7d3400.  lea rdi, [0x00005b30]
    |       |   ; int puts(const char *s)
    |       |   0x000026b3      e8e8edffff     call sym.imp.puts
    |       |   0x000026b8      bf01000000     mov edi, 1
    |       |   0x000026bd      e86eedffff     call fcn.00001430
    |       |   ; CODE XREF from main @ 0x26aa

    ; This is summed based of the key and the strings in the binary.

    |       `-> 0x000026c2      81bdf4fbffff.  cmp dword [var_40ch], 0x1f9
    |       ,=< 0x000026cc      750c           jne 0x26da
    |       |   0x000026ce      e8b0f4ffff     call fcn.00001b83	<- This one prints correct
    |       |   0x000026d3      e88af5ffff     call fcn.00001c62	<- This function prints flag
    |      ,==< 0x000026d8      eb05           jmp 0x26df
    |      ||   ; CODE XREF from main @ 0x26cc
    |      |`-> 0x000026da      e8d3f4ffff     call fcn.00001bb2  <- this function prints wrong
    |      |    ; CODE XREF from main @ 0x26d8
    |      `--> 0x000026df      488d8530fcff.  lea rax, [var_3d0h]
