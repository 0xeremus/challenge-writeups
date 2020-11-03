##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem: just_see
#### Solution: Flag{E4sy_ch4ll}
#### Explanation: Super easy challenge, take input of a string and compares it to the string "Flag{E4sy_ch4ll}"if it is the same it prints "G00d" otherwise it prints "Bad"

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Rewrite of challange:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

```c
#include <stdio.h>

#define MAX 64

int main(void)
{
    char *a = "Flag{E4sy_ch4ll}", b[MAX];

    printf("Hello !\n");
    printf("Give Me Your Flag\n");
    printf("Check Flag: ");
    scanf("%s", b);

    if (*b == *a) printf("G00d\n");
    else printf("Bad\n");
}
```
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly: (assembly and some comments from radare2)
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

``` asm
int main (int argc, char **argv, char **envp);
           ; var int var_40h @ rbp-0x40
           ; var int var_20h @ rbp-0x20
           ; var int var_18h @ rbp-0x18
           ; var int var_10h @ rbp-0x10
           ; var int var_8h @ rbp-0x8
           ; DATA XREF from entry0 (0x4005ad)
           0x00400686      55             push rbp                          ; set up stack frame
           0x00400687      4889e5         mov rbp, rsp
           0x0040068a      4883ec40       sub rsp, 0x40                     ; allocate space on the stack
           0x0040068e      64488b042528.  mov rax, qword fs:[0x28]
           0x00400697      488945f8       mov qword [var_8h], rax
           0x0040069b      31c0           xor eax, eax
           0x0040069d      bfd4074000     mov edi, str.Hello
           0x004006a2      e879feffff     call sym.imp.puts                 ; prints string "Hello !"
           0x004006a7      bfdd074000     mov edi, str.Give_Me_Your_Flag    ;
           0x004006ac      e86ffeffff     call sym.imp.puts                 ; prints string "Give Me Your Flag"
           0x004006b1      bfef074000     mov edi, str.Check_Flag:          ;
           0x004006b6      b800000000     mov eax, 0
           0x004006bb      e880feffff     call sym.imp.printf               ; prints string "Check Flag: "
           0x004006c0      488d45c0       lea rax, [var_40h]
           0x004006c4      4889c6         mov rsi, rax
           0x004006c7      bffc074000     mov edi, 0x4007fc
           0x004006cc      b800000000     mov eax, 0
           0x004006d1      e89afeffff     call sym.imp.__isoc99_scanf       ; call scanf which loads input onto the stack at rbp-0x40
           0x004006d6      48b8466c6167.  movabs rax, 0x7334457b67616c46    ; move into rax 'Flag{E4s'
           0x004006e0      488945e0       mov qword [var_20h], rax          ; store at rbp-0x20
           0x004006e4      48b8795f6368.  movabs rax, 0x7d6c6c3468635f79    ; move string 'y_ch4ll}' into rax
           0x004006ee      488945e8       mov qword [var_18h], rax          ; store at rbp-0x18
           0x004006f2      c745f0000000.  mov dword [var_10h], 0
           0x004006f9      488d55e0       lea rdx, [var_20h]                ; loads 'Flag{E4s' into rdx
           0x004006fd      488d45c0       lea rax, [var_40h]                ; loads input into rax
           0x00400701      4889d6         mov rsi, rdx                      ; loads rdx into rsi
           0x00400704      4889c7         mov rdi, rax                      ; loads rax into rdi
           0x00400707      e854feffff     call sym.imp.strcmp               ; calls strcmp which compares the values of rsi and rdi
           0x0040070c      85c0           test eax, eax                     ; test if they are the same
       ,=< 0x0040070e      750c           jne 0x40071c                      ; jump to 0x40071c and print "Bad " if not
       |   0x00400710      bfff074000     mov edi, str.G00d                 ; move "G00d " into edi
       |   0x00400715      e806feffff     call sym.imp.puts                 ; print string "G00d "
      ,==< 0x0040071a      eb0a           jmp 0x400726
      |`-> 0x0040071c      bf05084000     mov edi, str.Bad                  ; move string "Bad " into edi
      |    0x00400721      e8fafdffff     call sym.imp.puts                 ; call puts on "Bad "
      |    ; CODE XREF from main (0x40071a)
      `--> 0x00400726      b800000000     mov eax, 0
           0x0040072b      488b4df8       mov rcx, qword [var_8h]
           0x0040072f      6448330c2528.  xor rcx, qword fs:[0x28]
       ,=< 0x00400738      7405           je 0x40073f
       |   0x0040073a      e8f1fdffff     call sym.imp.__stack_chk_fail     ;
       `-> 0x0040073f      c9             leave
           0x00400740      c3             ret
```
