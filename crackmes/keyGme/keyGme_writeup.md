##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem: keyGme

#### Solution: Key must be 16 characters, contain only capital letters and 0-9 digits, the final character acts as a checksum for the solution. The checksum algorithm is:

    x = 0
    for each key_character
        x += key_character
        x = x >> 1
        x = x % 0xf00 (3840)
        x += 0xa

    if key_character[15] == x
        key is valid
    otherwise
        key is invalid


##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### C rewrite of challange:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>


//Function Prototypes
int validate_key(char* key);
int is_capital_letter(char key);
int is_number(char key);


int main(int argc, char* argv[])
{
    if (argc != 2)
    {
        printf("Usage: %s [serial-key]\n", *argv);
        return 0;
    }

    if (strlen(argv[1]) != 16)
    {
        printf("Your key must be 16 characters.\n");
        return 0;
    }

    if (validate_key(argv[1]))
    {
        printf("Congratulation, You are registered now.\n");
    }
    else
        printf("Sorry, the code was invalid.\n");

    return 0;
}


int validate_key(char* key)
{
    int len, acc;
    len = strlen(key);

    for (int i = 0; i < len-1; i++)
    {
        if (!is_capital_letter(key[i]))
            if (!is_number(key[i]))
            {
                printf("Found invalid character!\n");
                exit(0);
            }


        acc += key[i];
        acc = acc >> 1;
        acc = acc % 0xf00;
        acc += 0xa;

    }

    if (acc == key[15])
        return 1;
    else
        return 0;
}


int is_capital_letter(char key)
{
    if (key >= 'A' && key <= 'Z') return 1;
    else return 0;
}


int is_number(char key)
{
    if ((key >= '0') && (key <= '9')) return 1;
    else return 0;
}

```

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Python Key Gen Script:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
```python
#! /usr/bin/python3
import subprocess, random, sys


def main():

    for i in range(10):

        key = ''

        # Generate first 15 characters
        for i in range(15):
            key += random.choice("0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ")

        # Find the last element of the keys
        key = last_element(key)

        if key:
            key = check_key(key)
            if key:
                print("Validated Key: ", key)

# if want the key without checking validating it with the binary uncomment
# these lines comment out lines above
#
#       if key:
#            print("Non-validated key: ", key)


# Calculates the final key element by performing the operations that occur in 
# 0x08048323 of the binary
def last_element(key):

    # initialize keySum
    keySum = 0

    # sum the key elements
    for i in key:
        keySum += ord(i)
        keySum = keySum >> 1
        keySum = keySum % 0xf00
        keySum += 0xa

    # check that key is valid
    if chr(keySum) in "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ":
        key += chr(keySum)
        return key
    else:
        return 0


# Checks that the keygen program is generating the correct keys by 
# validating it with the binary.
def check_key(key):
    # Expected failure code from program
    failure = (b'Sorry, the code was invalid.\n', None)

# open binary and give it key to validate, script and binary must be in same folder
    p = subprocess.Popen(["./keyGme", key.encode()], stdout=subprocess.PIPE, stdin=subprocess.PIPE)

    # get message
    stdout = p.communicate(input=None, timeout=5)

    # check if key was correct
    if stdout != failure:
        return key
    else:
        return 0


if __name__=='__main__':
    main()
```

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
``` asm
0x0804841d----------------------------------------------------------------------

   int main (int argc, char **argv, char **envp);
           ; arg int arg_8h @ ebp+0x8                                   ; argv[]
           ; arg int arg_ch @ ebp+0xc                                   ; argc
           0x0804841d      55             push ebp
           0x0804841e      89e5           mov ebp, esp
           0x08048420      81ec00000000   sub esp, 0
           0x08048426      8b4508         mov eax, dword [arg_8h]       ;
           0x08048429      83f802         cmp eax, 2                    ; checks that argc is equal to 2
       ,=< 0x0804842c      0f841e000000   je 0x8048450                  ; jumps over usage section
       |   0x08048432      8b450c         mov eax, dword [arg_ch]       ;
       |   0x08048435      8b08           mov ecx, dword [eax]
       |   0x08048437      51             push ecx
       |   0x08048438      b8e9960408     mov eax, str.Usage            ; string =  "Usage: %s [serial-key]\n"
       |   0x0804843d      50             push eax
       |   0x0804843e      e869020000     call sub.printf_80486ac
       |   0x08048443      83c408         add esp, 8
       |   0x08048446      b801000000     mov eax, 1
      ,==< 0x0804844b      e972000000     jmp 0x80484c2
      ||   ; CODE XREF from main (0x804842c)
      |`-> 0x08048450      8b450c         mov eax, dword [arg_ch]       ; loads argv into eax
      |    0x08048453      83c004         add eax, 4                    ; switches from argv[0] -> argv[1] by adding 32 bits
      |    0x08048456      8b08           mov ecx, dword [eax]          ; moves into ecx
      |    0x08048458      51             push ecx                      ; pushes ecx onto stack and
      |    0x08048459      e81e020000     call sub.strlen_804867c       ; calls strlen
      |    0x0804845e      83c404         add esp, 4                    ; deallocated 4 bytes of the stack
      |    0x08048461      83f810         cmp eax, 0x10                 ; checks if strlen() returned a length of 16 characters
      |,=< 0x08048464      0f8418000000   je 0x8048482                  ; if it is it jumps the print statement etc
      ||   0x0804846a      b801970408     mov eax, str.Your_key_must_be_16_characters. ; kinda self explanatory
      ||   0x0804846f      50             push eax
      ||   0x08048470      e817020000     call sub.puts_804868c
      ||   0x08048475      83c404         add esp, 4
      ||   0x08048478      b802000000     mov eax, 2
     ,===< 0x0804847d      e940000000     jmp 0x80484c2                 ; jumps to return
     |||   ; CODE XREF from main (0x8048464)
     ||`-> 0x08048482      8b450c         mov eax, dword [arg_ch]       ; loads argv
     ||    0x08048485      83c004         add eax, 4                    ; ->argv[1]
     ||    0x08048488      8b08           mov ecx, dword [eax]          ; loads into ecx
     ||    0x0804848a      51             push ecx                      ; pushes to stack
     ||    0x0804848b      e893feffff     call sub.Found_invalid_Character ; calls function at 0x8048323
     ||    0x08048490      83c404         add esp, 4
     ||    0x08048493      83f800         cmp eax, 0                    ; Sees if the function returns true or not
     ||,=< 0x08048496      0f8413000000   je 0x80484af                  ; If so it prints success message
     |||   0x0804849c      b821970408     mov eax, str.Congratulation__You_are_registered_now. ; "Congratulation, You are registered now."
     |||   0x080484a1      50             push eax
     |||   0x080484a2      e8e5010000     call sub.puts_804868c
     |||   0x080484a7      83c404         add esp, 4
    ,====< 0x080484aa      e90e000000     jmp 0x80484bd
    ||||   ; CODE XREF from main (0x8048496)                                        ; Otherwise it prints a failure message
    |||`-> 0x080484af      b849970408     mov eax, str.Sorry__the_code_was_invalid. ; "Sorry, the code was invalid."
    |||    0x080484b4      50             push eax
    |||    0x080484b5      e8d2010000     call sub.puts_804868c
    |||    0x080484ba      83c404         add esp, 4
    |||    ; CODE XREF from main (0x80484aa)
    `----> 0x080484bd      b800000000     mov eax, 0
     ||    ; CODE XREFS from main (0x804844b, 0x804847d)
     ``--> 0x080484c2      c9             leave
           0x080484c3      c3             ret

0x08048323----------------------------------------------------------------------

   sub.Found_invalid_character_8048323 (int arg_8h);
           ; var int var_ch @ ebp-0xc
           ; var int var_8h @ ebp-0x8
           ; var int var_4h @ ebp-0x4
           ; arg int arg_8h @ ebp+0x8                                   ; not a local variable argv
           ; CALL XREF from main (0x804848b)
           0x08048323      55             push ebp                      ; sets up stack frame
           0x08048324      89e5           mov ebp, esp
           0x08048326      81ec0c000000   sub esp, 0xc                  ; allocates 12 bytes on the stack
           0x0804832c      8b4508         mov eax, dword [arg_8h]       ; move argv into eax
           0x0804832f      50             push eax
           0x08048330      e847030000     call sub.strlen_804867c       ; call strlen on argv
           0x08048335      83c404         add esp, 4                    ; deallocate stack
           0x08048338      8945fc         mov dword [var_4h], eax       ; Move string length into eax
           0x0804833b      b800000000     mov eax, 0                    ; 0 into eax
           0x08048340      8945f8         mov dword [var_8h], eax       ; 0 ebp-0x8
           0x08048343      b800000000     mov eax, 0
           0x08048348      8945f4         mov dword [var_ch], eax       ; 0 ebp-0xc
       .-> 0x0804834b      8b45fc         mov eax, dword [var_4h]       ; move string length into eax (which has to be 16)
       :   0x0804834e      48             dec eax                       ; -1 eax
       :   0x0804834f      8b4df4         mov ecx, dword [var_ch]       ; 0 into ecx
       :   0x08048352      39c1           cmp ecx, eax                  ; compare strings to test if either func.0x... returned 0 into eax
      ,==< 0x08048354      0f8da8000000   jge 0x8048402                 ; this continues the loop by jumping over the previous jump statement
     ,===< 0x0804835a      e90b000000     jmp 0x804836a                 ; This jumps at the end of the loop to return
    .----> 0x0804835f      8b45f4         mov eax, dword [var_ch]
    :||:   0x08048362      89c1           mov ecx, eax
    :||:   0x08048364      40             inc eax
    :||:   0x08048365      8945f4         mov dword [var_ch], eax
    :||`=< 0x08048368      ebe1           jmp 0x804834b
    :||    ; CODE XREF from sub.Found_invalid_character_8048323
    :`---> 0x0804836a      8b45f4         mov eax, dword [var_ch]
    : |    0x0804836d      8b4d08         mov ecx, dword [arg_8h]     ;
    : |    0x08048370      01c1           add ecx, eax
    : |    0x08048372      0fbe01         movsx eax, byte [ecx]
    : |    0x08048375      50             push eax
    : |    0x08048376      e846ffffff     call fcn.080482c1             ; Tests if the current character is a capital letter
    : |    0x0804837b      83c404         add esp, 4
    : |    0x0804837e      83f800         cmp eax, 0                    ; if it is a capital letter its return value is non zero
    : |,=< 0x08048381      0f8522000000   jne 0x80483a9                 ; it jumps, if isn't it continue through
    : ||   0x08048387      8b45f4         mov eax, dword [var_ch]
    : ||   0x0804838a      8b4d08         mov ecx, dword [arg_8h]
    : ||   0x0804838d      01c1           add ecx, eax
    : ||   0x0804838f      0fbe01         movsx eax, byte [ecx]
    : ||   0x08048392      50             push eax
    : ||   0x08048393      e85affffff     call fcn.080482f2             ; and checks its a number by calling this function which returns 0 if it is not 0-9
    : ||   0x08048398      83c404         add esp, 4
    : ||   0x0804839b      83f800         cmp eax, 0
    :,===< 0x0804839e      0f8505000000   jne 0x80483a9                 ;
   ,=====< 0x080483a4      e938000000     jmp 0x80483e1                 ; this jumps to print "Found invalid character!" string
   |:`-`-> 0x080483a9      8b45f4         mov eax, dword [var_ch]     ; This section is responsible for calculating the required value of the final character
   |: |    0x080483ac      8b4d08         mov ecx, dword [arg_8h]     ;
   |: |    0x080483af      01c1           add ecx, eax                ;
   |: |    0x080483b1      8b45f8         mov eax, dword [var_8h]     ; moves the accumulated value into eax
   |: |    0x080483b4      0fbe11         movsx edx, byte [ecx]       ;
   |: |    0x080483b7      01d0           add eax, edx                ; this is the acculmuator addition
   |: |    0x080483b9      8945f8         mov dword [var_8h], eax
   |: |    0x080483bc      8b45f8         mov eax, dword [var_8h]
   |: |    0x080483bf      c1f801         sar eax, 1                  ; this is the shift right by 1
   |: |    0x080483c2      8945f8         mov dword [var_8h], eax
   |: |    0x080483c5      8b45f8         mov eax, dword [var_8h]
   |: |    0x080483c8      b9000f0000     mov ecx, 0xf00              ; this is the modulation value being loaded
   |: |    0x080483cd      99             cdq                         ; and extended to be a quad word
   |: |    0x080483ce      f7f9           idiv ecx                    ; then divides eax
   |: |    0x080483d0      8955f8         mov dword [var_8h], edx
   |: |    0x080483d3      8b45f8         mov eax, dword [var_8h]
   |: |    0x080483d6      83c00a         add eax, 0xa                ; this is the addition of 0xa
   |: |    0x080483d9      8945f8         mov dword [var_8h], eax
   |: |,=< 0x080483dc      e91c000000     jmp 0x80483fd
   |: ||   ; CODE XREF from sub.Found_invalid_character_8048323 (0x80483a4)
   `-----> 0x080483e1      b8d0960408     mov eax, str.Found_invalid_character ; loads "Found invalid character!"
    : ||   0x080483e6      50             push eax
    : ||   0x080483e7      e8a0020000     call sub.puts_804868c
    : ||   0x080483ec      83c404         add esp, 4
    : ||   0x080483ef      b803000000     mov eax, 3
    : ||   0x080483f4      50             push eax
    : ||   0x080483f5      e8a2020000     call sub.exit_804869c
    : ||   0x080483fa      83c404         add esp, 4
    | ||   ; CODE XREF from sub.Found_invalid_character_8048323 (0x80483dc)
    `==`-> 0x080483fd      e95dffffff     jmp 0x804835f
      |    ; CODE XREF from sub.Found_invalid_character_8048323 (0x8048354)
      `--> 0x08048402      8b45fc         mov eax, dword [var_4h]
           0x08048405      48             dec eax
           0x08048406      8b4d08         mov ecx, dword [arg_8h]     ; [0x8:4]=-1 ; 8
           0x08048409      01c1           add ecx, eax
           0x0804840b      8b45f8         mov eax, dword [var_8h]
           0x0804840e      0fbe11         movsx edx, byte [ecx]
           0x08048411      39d0           cmp eax, edx                 ; this is the final comparison statement between the final letter of the key
           0x08048413      b800000000     mov eax, 0                   ; and the accumulated value
           0x08048418      0f94c0         sete al
           0x0804841b      c9             leave
           0x0804841c      c3             ret

0x080482c1----------------------------------------------------------------------

    fcn.080482c1 (int arg_8h);
      ; arg int arg_8h @ ebp+0x8
      ; CALL XREF from sub.Found_invalid_character_8048323 (0x8048376)
      0x080482c1      55             push ebp
      0x080482c2      89e5           mov ebp, esp
      0x080482c4      81ec00000000   sub esp, 0
      0x080482ca      0fbe4508       movsx eax, byte [arg_8h]    ; checks if byte is
      0x080482ce      83f841         cmp eax, 0x41               ; less then 'A'
  ,=< 0x080482d1      0f8c14000000   jl 0x80482eb                ; and jumps to the end returning 0
  |   0x080482d7      0fbe4508       movsx eax, byte [arg_8h]    ; checks if the byte is
  |   0x080482db      83f85a         cmp eax, 0x5a               ; greater then 'Z
 ,==< 0x080482de      0f8f07000000   jg 0x80482eb                ; and jumps to the end returning 0
 ||   0x080482e4      b801000000     mov eax, 1
,===< 0x080482e9      eb05           jmp 0x80482f0
|||   ; CODE XREFS from fcn.080482c1 (0x80482d1, 0x80482de)
|``-> 0x080482eb      b800000000     mov eax, 0
|     ; CODE XREF from fcn.080482c1 (0x80482e9)
`---> 0x080482f0      c9             leave                      ; otherwise it does not return 0
       0x080482f1      c3             ret

0x080482c1----------------------------------------------------------------------
                                                                ; This function does the exact same
   fcn.080482f2 (int arg_8h);                                   ; but for the number 0 and 9
      ; arg int arg_8h @ ebp+0x8
      ; CALL XREF from sub.Found_invalid_character_8048323 (0x8048393)
      0x080482f2      55             push ebp
      0x080482f3      89e5           mov ebp, esp
      0x080482f5      81ec00000000   sub esp, 0
      0x080482fb      0fbe4508       movsx eax, byte [arg_8h]    ; [0x8:1]=255 ; 8
      0x080482ff      83f830         cmp eax, 0x30               ; '0' ; 48
  ,=< 0x08048302      0f8c14000000   jl 0x804831c
  |   0x08048308      0fbe4508       movsx eax, byte [arg_8h]    ; [0x8:1]=255 ; 8
  |   0x0804830c      83f839         cmp eax, 0x39               ; '9' ; 57
 ,==< 0x0804830f      0f8f07000000   jg 0x804831c
 ||   0x08048315      b801000000     mov eax, 1
,===< 0x0804831a      eb05           jmp 0x8048321
|||   ; CODE XREFS from fcn.080482f2 (0x8048302, 0x804830f)
|``-> 0x0804831c      b800000000     mov eax, 0
|     ; CODE XREF from fcn.080482f2 (0x804831a)
`---> 0x08048321      c9             leave
           0x08048322      c3             ret

```