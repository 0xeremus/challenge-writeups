##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
#### Problem: easy_reversing
#### Solution: ./a.out [10 character string with 5th character =='@'] eg."pass@words"

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Rewrite of challange:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
``` c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

/* Function Prototypes */
void usage(char argv[]);

int main(int argc, char* argv[])
{
    if (argc != 2)
        usage(argv[0]);
    else if (strlen(argv[1]) != 0xa)
        usage(argv[0]);
    else if (argv[1][4] != 0x40)
        usage(argv[0]);
    else
        printf("Nice Job!!\n");
        printf("flag{%s}\n", argv[1]);
}

void usage(char argv[])
{
    printf("USAGE: %s <password>\n", argv);
    printf("try again!\n");
    exit(0);
}
```

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Flag Generator:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
```python
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
```
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly: (assembly and some comments from radare2)
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
``` asm
/ (fcn) main 169
|   int main (int argc, char **argv, char **envp);
|           ; var char **s @ rbp-0x10
|           ; var uint32_t var_4h @ rbp-0x4
|           ; arg uint32_t argc @ rdi
|           ; arg char **argv @ rsi
|           ; DATA XREF from entry0 (0x109d)
|           0x000011c4      55             push rbp
|           0x000011c5      4889e5         mov rbp, rsp
|           0x000011c8      4883ec10       sub rsp, 0x10
|           0x000011cc      897dfc         mov dword [var_4h], edi     ; argc
|           0x000011cf      488975f0       mov qword [s], rsi          ; argv
|           0x000011d3      837dfc02       cmp dword [var_4h], 2       ; Checks argc == 2
|       ,=< 0x000011d7      757e           jne 0x1257                  ; if not jump to call to usage() function
|       |   0x000011d9      488b45f0       mov rax, qword [s]          ; moves argv into rax
|       |   0x000011dd      4883c008       add rax, 8                  ; moves to second argument
|       |   0x000011e1      488b00         mov rax, qword [rax]        ; extends to quad word
|       |   0x000011e4      4889c7         mov rdi, rax                ; save string in rax
|       |   0x000011e7      e854feffff     call sym.imp.strlen         ; check length of string
|       |   0x000011ec      4883f80a       cmp rax, 0xa                ; if not 10 fail
|      ,==< 0x000011f0      7554           jne 0x1246                  ; will call usage()
|      ||   0x000011f2      488b45f0       mov rax, qword [s]          ;reload argv
|      ||   0x000011f6      4883c008       add rax, 8                  ; switch to argv[1]
|      ||   0x000011fa      488b00         mov rax, qword [rax]        ; extend to quad word
|      ||   0x000011fd      4883c004       add rax, 4                  ; halve string
|      ||   0x00001201      0fb600         movzx eax, byte [rax]       ; most last byte into eax
|      ||   0x00001204      3c40           cmp al, 0x40                ; see if it equals '@'
|     ,===< 0x00001206      752d           jne 0x1235                  ; if it doesn't fail
|     |||   0x00001208      488d3d160e00.  lea rdi, str.Nice_Job       ; if it does load "Nice Job!!"
|     |||   0x0000120f      e81cfeffff     call sym.imp.puts           ; and print it
|     |||   0x00001214      488b45f0       mov rax, qword [s]
|     |||   0x00001218      4883c008       add rax, 8                  ; do the same sequence as already commented
|     |||   0x0000121c      488b00         mov rax, qword [rax]
|     |||   0x0000121f      4889c6         mov rsi, rax
|     |||   0x00001222      488d3d070e00.  lea rdi, str.flag__s        ; to print the string"flag{STRING}\n"t
|     |||   0x00001229      b800000000     mov eax, 0
|     |||   0x0000122e      e81dfeffff     call sym.imp.printf         ; calll to printf to print above string
|    ,====< 0x00001233      eb31           jmp 0x1266                  ; jumps to end of function and returns
|    ||||   ; CODE XREF from main (0x1206)
|    |`---> 0x00001235      488b45f0       mov rax, qword [s]          ; all subsequent functions do the same thing
|    | ||   0x00001239      488b00         mov rax, qword [rax]        ; they all eventually call usage()
|    | ||   0x0000123c      4889c7         mov rdi, rax
|    | ||   0x0000123f      e846ffffff     call sym.usage
|    |,===< 0x00001244      eb20           jmp 0x1266
|    ||||   ; CODE XREF from main (0x11f0)
|    ||`--> 0x00001246      488b45f0       mov rax, qword [s]
|    || |   0x0000124a      488b00         mov rax, qword [rax]
|    || |   0x0000124d      4889c7         mov rdi, rax
|    || |   0x00001250      e835ffffff     call sym.usage
|    ||,==< 0x00001255      eb0f           jmp 0x1266
|    ||||   ; CODE XREF from main (0x11d7)
|    |||`-> 0x00001257      488b45f0       mov rax, qword [s]
|    |||    0x0000125b      488b00         mov rax, qword [rax]
|    |||    0x0000125e      4889c7         mov rdi, rax
|    |||    0x00001261      e824ffffff     call sym.usage
|    |||    ; CODE XREFS from main (0x1233, 0x1244, 0x1255)
|    ```--> 0x00001266      b800000000     mov eax, 0
|           0x0000126b      c9             leave
\           0x0000126c      c3             ret
```
