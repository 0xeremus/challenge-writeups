##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem: sh4ll2
#### Solution: 46
(First use of Ghidra, it makes it EASY, which I guess is good).

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Explanation:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

- Variable (Vin) taken by scanf, added to an initialized variable (Var1) = 3.56..
- Var1 added to Vin and cast to integer as V3
- V2 cleared to zero
- while loop is entered that loops while V2 is less then V3
    Vacc = (float) V3 + V2 + Vacc
    V2 = V2 + 0.8
- print the Vacc
- check if Vacc is equal to 4550.799805
- if so print "Good Password"
- else print "Bad Password"

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Rewrite of challange:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
```c
#include <stdio.h>

int main(void)
{
    int Vin;
    float V1 = 3.56;
    int V3;
    float Vacc = 0.0;
    float V2 = 0.0;
    float check = 4550.799805;

    scanf("%d", &Vin);
    V3 = (int)((float) Vin + V1);

    while(V2 < (float) V3)
    {
        Vacc = (float) V3 + V2 + Vacc;
        V2 = V2 + 0.8000001;
    }

    printf("%f\n", Vacc); // added \n

    if (Vacc == check)
        printf("Good Password\n"); // added \n
    else
        printf("Bad Password\n");


    return 0;
}
```
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
```asm
   int main (int argc, char **argv, char **envp);
       ; var int var_14h @ rbp-0x14
       ; var int var_10h @ rbp-0x10
       ; var int var_ch @ rbp-0xc
       ; var int var_8h @ rbp-0x8
       ; var int var_4h @ rbp-0x4
       ; DATA XREF from entry0 (0x60d)
       0x00000720      55             push rbp                          ; Stack Preamble
       0x00000721      4889e5         mov rbp, rsp
       0x00000724      4883ec20       sub rsp, 0x20
       0x00000728      f30f10058401.  movss xmm0, dword [0x000008b4]    ; 3.564 V1
       0x00000730      f30f1145f4     movss dword [var_ch], xmm0
       0x00000735      488d45ec       lea rax, [var_14h]
       0x00000739      4889c6         mov rsi, rax
       0x0000073c      488d3d510100.  lea rdi, [0x00000894]
       0x00000743      b800000000     mov eax, 0
       0x00000748      e883feffff     call sym.imp.__isoc99_scanf       ; Take input of integer Vin
       0x0000074d      8b45ec         mov eax, dword [var_14h]
       0x00000750      660fefc0       pxor xmm0, xmm0
       0x00000754      f30f2ac0       cvtsi2ss xmm0, eax                ; converts to float
       0x00000758      f30f5845f4     addss xmm0, dword [var_ch]        ; V3 = Vin + V1
       0x0000075d      f30f2cc0       cvttss2si eax, xmm0               ; convert to integer
       0x00000761      8945f0         mov dword [var_10h], eax          ; stre at [var_10h]
       0x00000764      660fefc0       pxor xmm0, xmm0                   ; zero xmm0
       0x00000768      f30f1145f8     movss dword [var_8h], xmm0
   ,=< 0x0000076d      eb32           jmp 0x7a1                         ; jmp into while loop
   |   ; CODE XREF from main (0x7ae)
  .--> 0x0000076f      660fefc0       pxor xmm0, xmm0
  :|   0x00000773      f30f2a45f0     cvtsi2ss xmm0, dword [var_10h]    ; Convert V3 to float
  :|   0x00000778      f30f5845f8     addss xmm0, dword [var_8h]        ; add V2 to xmm0
  :|   0x0000077d      f30f104dfc     movss xmm1, dword [var_4h]        ; move Vacc to xmm1
  :|   0x00000782      f30f58c1       addss xmm0, xmm1                  ; add V2 + Vacc
  :|   0x00000786      f30f1145fc     movss dword [var_4h], xmm0        ; mov to var_4h (Vacc)
  :|   0x0000078b      f30f104df8     movss xmm1, dword [var_8h]        ; mov V2 to xmm1
  :|   0x00000790      f30f10052001.  movss xmm0, dword [0x000008b8]    ; move 0.8 constant value into xmm0
  :|   0x00000798      f30f58c1       addss xmm0, xmm1                  ; add together
  :|   0x0000079c      f30f1145f8     movss dword [var_8h], xmm0        ; move xmm0 into V2
  :`-> 0x000007a1      660fefc0       pxor xmm0, xmm0                   ; and so on
  :    0x000007a5      f30f2a45f0     cvtsi2ss xmm0, dword [var_10h]
  :    0x000007aa      0f2e45f8       ucomiss xmm0, dword [var_8h]     ; Check if V2 is less then V3
  `==< 0x000007ae      77bf           ja 0x76f                         ; if so continue while loop
       0x000007b0      f30f5a45fc     cvtss2sd xmm0, dword [var_4h]
       0x000007b5      488d3ddb0000.  lea rdi, [0x00000897]
       0x000007bc      b801000000     mov eax, 1
       0x000007c1      e8fafdffff     call sym.imp.printf              ; printf Vacc
       0x000007c6      f30f1045fc     movss xmm0, dword [var_4h]        ; Check values and stuff print Good bad etc.
       0x000007cb      0f2e05ea0000.  ucomiss xmm0, dword [0x000008bc]
   ,=< 0x000007d2      7a21           jp 0x7f5
   |   0x000007d4      f30f1045fc     movss xmm0, dword [var_4h]
   |   0x000007d9      0f2e05dc0000.  ucomiss xmm0, dword [0x000008bc]
  ,==< 0x000007e0      7513           jne 0x7f5
  ||   0x000007e2      488d3db10000.  lea rdi, str.Good_pasword         ; "Good pasword"
  ||   0x000007e9      b800000000     mov eax, 0
  ||   0x000007ee      e8cdfdffff     call sym.imp.printf               ; int printf(const char *format)
 ,===< 0x000007f3      eb11           jmp 0x806
 |||   ; CODE XREFS from main (0x7d2, 0x7e0)
 |``-> 0x000007f5      488d3dab0000.  lea rdi, str.Bad_password   ; 0x8a7 ; "Bad password"
 |     0x000007fc      b800000000     mov eax, 0
 |     0x00000801      e8bafdffff     call sym.imp.printf         ; int printf(const char *format)
 |     ; CODE XREF from main (0x7f3)
 `---> 0x00000806      b800000000     mov eax, 0
       0x0000080b      c9             leave
       0x0000080c      c3             ret
```
