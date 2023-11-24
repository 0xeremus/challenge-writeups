##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem: cedece 
#### Solution: ./cedece NKRI_1

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Explanation:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

This is definitely a 1 for difficulty, the flag is directly compared by a strncmp function that if the flag matches jumps to print out FLAG BENAR (TRUE) if not doesn't jump and prints FLAG SALAH (FALSE)

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Rewrite of challange:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

``` code 
    #include <stdio.h>
    #include <stdlib.h>
    #include <string.h>

    int main(int argc, const char *argv[])
    {
        char *FLAG = "NKRI_1";
        int len;

        if (argc < 2)
        {
            printf("CDC2016{flag}: %s flag\n", *argv);
            exit(1);
        }

        len = strlen(argv[1]);

        if (strncmp(argv[1], FLAG, len) != 0)
        {
            printf("FLAG SALAH\n"); // FLAG FALSE
            exit(1);
        }
        else
            printf("FLAG BENAR\n");


        return 0;
    }
```
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

``` asm
   int sym.main (int argc, char **argv, char **envp);
       ; var char *s2 @ ebp-0xf
       ; var int var_bh @ ebp-0xb
       ; var int var_9h @ ebp-0x9
       ; var int var_8h @ ebp-0x8
       ; arg int arg_4h @ esp+0x4
       ; DATA XREF from entry0 (0x80483d7)
       0x080484bb b    8d4c2404       lea ecx, [arg_4h]           ; 4
       0x080484bf      83e4f0         and esp, 0xfffffff0
       0x080484c2      ff71fc         push dword [ecx - 4]
       0x080484c5      55             push ebp
       0x080484c6      89e5           mov ebp, esp
       0x080484c8      53             push ebx
       0x080484c9      51             push ecx
       0x080484ca      83ec10         sub esp, 0x10
       0x080484cd      89cb           mov ebx, ecx
       0x080484cf      c745f14e4b52.  mov dword [s2], 0x49524b4e  ; 'NKRI' This is the flag being loaded onto the stack
       0x080484d6      66c745f55f31   mov word [var_bh], 0x315f   ; '_1'
       0x080484dc      c645f700       mov byte [var_9h], 0
       0x080484e0      833b01         cmp dword [ebx], 1          ; this is checking if there was a cmdline argument
   ,=< 0x080484e3      7f20           jg 0x8048505
   |   0x080484e5      8b4304         mov eax, dword [ebx + 4]
   |   0x080484e8      8b00           mov eax, dword [eax]
   |   0x080484ea      83ec08         sub esp, 8
   |   0x080484ed      50             push eax
   |   0x080484ee      6800860408     push str.CDC2016_flag_:__s_flag ; If not is prints this "CDC2016{flag}: %s flag\n"
   |   0x080484f3      e858feffff     call sym.imp.printf         ; as a usage message
   |   0x080484f8      83c410         add esp, 0x10
   |   0x080484fb      83ec0c         sub esp, 0xc
   |   0x080484fe      6a01           push 1
   |   0x08048500      e87bfeffff     call sym.imp.exit           ; and exits
   |   ; CODE XREF from sym.main (0x80484e3)
   `-> 0x08048505      83ec0c         sub esp, 0xc                ; allocates stack
       0x08048508      8d45f1         lea eax, [s2]               ; this contains the flag
       0x0804850b      50             push eax
       0x0804850c      e87ffeffff     call sym.imp.strlen         ; calls strlen used in the call to strncmp
       0x08048511      83c410         add esp, 0x10               ; deallocates stack
       0x08048514      89c2           mov edx, eax                ; moves strlen (6) into edx
       0x08048516      8b4304         mov eax, dword [ebx + 4]    ; moves a pointer to commandline args into eax
       0x08048519      83c004         add eax, 4                  ; increments the pointer to argv[1]
       0x0804851c      8b00           mov eax, dword [eax]        ; sign extends it
       0x0804851e      83ec04         sub esp, 4                  ; deallocates stack
       0x08048521      52             push edx                    ; pushes the strlen onto the stack
       0x08048522      8d55f1         lea edx, [s2]               ; loads the flag from memory
       0x08048525      52             push edx                    ; pushes the known flag value "NKRI_1"
       0x08048526      50             push eax                    ; pushes user supplied value
       0x08048527      e884feffff     call sym.imp.strncmp        ; int strncmp(const char *s1, const char *s2, size_t n)
       0x0804852c      83c410         add esp, 0x10               ; deallocates stack memory
       0x0804852f      85c0           test eax, eax               ; if strncmp returned 0 (flags matched) then jmp
   ,=< 0x08048531      741a           je 0x804854d
   |   0x08048533      83ec0c         sub esp, 0xc
   |   0x08048536      6818860408     push str.FLAG_SALAH         ; "FLAG SALAH " "FLAG FALSE"
   |   0x0804853b      e820feffff     call sym.imp.puts           ; prints FLAG FALSE
   |   0x08048540      83c410         add esp, 0x10
   |   0x08048543      83ec0c         sub esp, 0xc
   |   0x08048546      6a01           push 1
   |   0x08048548      e833feffff     call sym.imp.exit           ; call sys exit
   |   ; CODE XREF from sym.main (0x8048531)
   `-> 0x0804854d      83ec0c         sub esp, 0xc                ; allocate stack
       0x08048550      6824860408     push str.FLAG_BENAR         ; push string "FLAG BENAR" "FLAG TRUE"
       ;-- eip:
       0x08048555      e806feffff     call sym.imp.puts           ; print acceptance string
       0x0804855a      83c410         add esp, 0x10               ; clean up and return
       0x0804855d      b800000000     mov eax, 0
       0x08048562      8d65f8         lea esp, [var_8h]
       0x08048565      59             pop ecx
       0x08048566      5b             pop ebx
       0x08048567      5d             pop ebp
       0x08048568      8d61fc         lea esp, [ecx - 4]
       0x0804856b      c3             ret
```