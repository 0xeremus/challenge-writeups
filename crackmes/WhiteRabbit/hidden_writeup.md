##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem:hidden by White Rabbit
#### Solution:flag{3sc0nd1d0_3h_M41s_G0st0S0}

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Explanation: Patch the lea
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

There is a function sym.secret which prints out a flag. Read it and you see the value of the flag is computed by a series of sequential instructions. Either reconstruct it. OR patch

    0x5629ee0e61e5      488d3d6c0e00.  lea rdi, [0x5629ee0e7058] ; "..."

in the main function that will fit a call to the function secret in it. Which simply print the flag. I was feeling lazy so I patched it. Hence my rewrite is incomplete.

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Rewrite of challange:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

```c
#include <stdio.h>

int secret(void);

int main(void)
{
    char s[] = {"The only way out is inwards\n\n\n\n\n\n"};
    char t[] = {"..."};
    char u[] = {"Voce consegue achar a funcao escondida?"};

    puts("%s", s);
    printf("%s", t);
    printf("%s", t);

    return 0;
}

int secret(void)
{
    int a = 5;
    int b = 3;
    int c = 4;
    char d = 's';
    char e = 'd';
    char f = 'c';

    //do magic stuff to generate flag

    printf("flag{%d%c%c%dn%c%d%c%d_%dh_M%d%ds_G%d%ct%dS%d}\n", print the flag variables.);

    return 0;
}
```

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

Before patch:
```asm
/ (fcn) main 52
|   int main (int argc, char **argv, char **envp);
|           ; DATA XREF from entry0 (0x5629ee0e607d)
|           0x5629ee0e61d5      55             push rbp
|           0x5629ee0e61d6      4889e5         mov rbp, rsp
|           0x5629ee0e61d9      488d3d580e00.  lea rdi, str.The_only_way_out_is_inward ; 0x5629ee0e7038 ; "The only way out is inward\n\n\n\n\n"
|           0x5629ee0e61e0      e84bfeffff     call sym.imp.puts       ; int puts(const char *s)
|           0x5629ee0e61e5      488d3d6c0e00.  lea rdi, [0x5629ee0e7058] ; "..."
|           0x5629ee0e61ec      b800000000     mov eax, 0
|           0x5629ee0e61f1      e84afeffff     call sym.imp.printf     ; int printf(const char *format)
|           0x5629ee0e61f6      488d3d630e00.  lea rdi, str.Voce_consegue_achar_a_funcao_escondida ; 0x5629ee0e7060 ; "Voce consegue achar a funcao escondida?"
|           0x5629ee0e61fd      e82efeffff     call sym.imp.puts       ; int puts(const char *s)
|           0x5629ee0e6202      b800000000     mov eax, 0
|           0x5629ee0e6207      5d             pop rbp
\           0x5629ee0e6208      c3             ret
```

After patch:

```
|           ; DATA XREF from entry0 (0x5629ee0e607d)
|           0x5629ee0e61d5      55             push rbp
|           0x5629ee0e61d6      4889e5         mov rbp, rsp
|           0x5629ee0e61d9      488d3d580e00.  lea rdi, str.The_only_way_out_is_inward ; 0x5629ee0e7038 ; "The only way out is inward\n\n\n\n\n"
|           0x5629ee0e61e0      e84bfeffff     call sym.imp.puts       ; int puts(const char *s)
|           0x5629ee0e61e5      e85bffffff     call sym.secret
|           0x5629ee0e61ec      b800000000     mov eax, 0
|           0x5629ee0e61f1      e84afeffff     call sym.imp.printf     ; int printf(const char *format)
|           0x5629ee0e61f6      488d3d630e00.  lea rdi, str.Voce_consegue_achar_a_funcao_escondida ; 0x5629ee0e7060 ; "Voce consegue achar a funcao escondida?"
|           0x5629ee0e61fd      e82efeffff     call sym.imp.puts       ; int puts(const char *s)
|           0x5629ee0e6202      b800000000     mov eax, 0
|           0x5629ee0e6207      5d             pop rbp
\           0x5629ee0e6208      c3             ret
```

Function Called:
This function clearly just computes the flag. Is possible to just read it and
write out the flag. Which is flag{3sc0nd1d0_3h_M41s_G0st0S0}, but you can also
just call the function, i was feeling lazy so that's what I did.

```asm
/ (fcn) sym.secret 144
|   sym.secret ();
|           ; var int32_t var_fh @ rbp-0xf
|           ; var int32_t var_eh @ rbp-0xe
|           ; var int32_t var_dh @ rbp-0xd
|           ; var int32_t var_ch @ rbp-0xc
|           ; var int32_t var_8h @ rbp-0x8
|           ; var int32_t var_4h @ rbp-0x4
|           0x00001145      55             push rbp
|           0x00001146      4889e5         mov rbp, rsp
|           0x00001149      4883ec10       sub rsp, 0x10
|           0x0000114d      c745fc050000.  mov dword [var_4h], 5
|           0x00001154      c745f8030000.  mov dword [var_8h], 3
|           0x0000115b      c745f4040000.  mov dword [var_ch], 4
|           0x00001162      c645f373       mov byte [var_dh], 0x73     ; 's'
|           0x00001166      c645f264       mov byte [var_eh], 0x64     ; 'd'
|           0x0000116a      c645f163       mov byte [var_fh], 0x63     ; 'c'
|           0x0000116e      440fbe4df3     movsx r9d, byte [var_dh]
|           0x00001173      8b45fc         mov eax, dword [var_4h]
|           0x00001176      2b45f4         sub eax, dword [var_ch]
|           0x00001179      4189c0         mov r8d, eax
|           0x0000117c      0fbe7df2       movsx edi, byte [var_eh]
|           0x00001180      8b45fc         mov eax, dword [var_4h]
|           0x00001183      2b45f4         sub eax, dword [var_ch]
|           0x00001186      89c6           mov esi, eax
|           0x00001188      440fbe55f2     movsx r10d, byte [var_eh]
|           0x0000118d      0fbe4df1       movsx ecx, byte [var_fh]
|           0x00001191      0fbe55f3       movsx edx, byte [var_dh]
|           0x00001195      8b45f8         mov eax, dword [var_8h]
|           0x00001198      6a00           push 0
|           0x0000119a      6a00           push 0
|           0x0000119c      4151           push r9
|           0x0000119e      6a00           push 0
|           0x000011a0      4150           push r8
|           0x000011a2      448b45f4       mov r8d, dword [var_ch]
|           0x000011a6      4150           push r8
|           0x000011a8      448b45f8       mov r8d, dword [var_8h]
|           0x000011ac      4150           push r8
|           0x000011ae      6a00           push 0
|           0x000011b0      57             push rdi
|           0x000011b1      56             push rsi
|           0x000011b2      4589d1         mov r9d, r10d
|           0x000011b5      41b800000000   mov r8d, 0
|           0x000011bb      89c6           mov esi, eax
|           0x000011bd      488d3d440e00.  lea rdi, str.flag__d_c_c_dn_c_d_c_d__dh_M_d_ds_G_d_ct_dS_d ; 0x2008 ; "flag{%d%c%c%dn%c%d%c%d_%dh_M%d%ds_G%d%ct%dS%d}\n"
|           0x000011c4      b800000000     mov eax, 0
|           0x000011c9      e872feffff     call sym.imp.printf         ; int printf(const char *format)
|           0x000011ce      4883c450       add rsp, 0x50               ; 'P'
|           0x000011d2      90             nop
|           0x000011d3      c9             leave
\           0x000011d4      c3             ret
```
