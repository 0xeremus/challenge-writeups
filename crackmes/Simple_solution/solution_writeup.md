##### -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
##### Problem: SimpleKeyGenerator
##### Solution: The key must be 16 characters long and each pair (0-1,2-3 etc) of characters in the key must subtract to equal -1 eg key abababababababab or abcdefghijklmnop

##### -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
##### Problem C rewrite
##### -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

```c
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdbool.h>

/*function prototypes*/
int checkSerial(char *a);
void usage(void);

/* main function takes input and checks its length then calls checkSerial */
int main(int argc, char *argv[])
{
    char *key;
    /* Check that a serial number was received or print warning and return */
    if (argc != 2) usage();

    key = argv[1];

    /* call checkSerial function */
    if (checkSerial(key))
        printf("Good Serial\n");
    else
        printf("Bad Serial\n");
}


// Function checks length and then checks if each pair of values in the key 
// when subtracted from one another equals -1
int checkSerial(char *key)
{
    /* Checks the length of the key */
    if (strlen(key) != 0x10)
        return false;

    /* Checks that the key obeys the key algorithm, namely that 
    each pair of values is equal to -1 */
    for (int i = 0; i < strlen(key); i += 2)
    {
        if (key[i]-key[i+1] == -1) continue;
        else return false;
    }

    /* returns 0 meaning key is correct */
    return true;
}

void usage(void)
{
    printf("./SimpleKeyGen [SERIAL]);
    exit(0);
}
```

##### -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
##### Key Generator Solution:
##### -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

```python3
#! /usr/bin/python3
#
#Solution to Yuri's SimpleKeyGen0

import random, sys


def main():
    if len(sys.argv) == 2:
        count = int(sys.argv[1])
    else:
        count = 1


    key = ''
    for i in range(0, count):
        while(len(key) != 16):
            x = random.randrange(0x61, 0x7a)
            key += chr(x)
            key += chr(x + 1)
        print(key)
        key = ''


if __name__=='__main__':
    main()
```
##### -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=
##### Commented Assembly Code: [This is probably pretty unclear commenting]
##### -=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=

```asm
0000000000001226 <main>:
    1226:	55                   	push   rbp                              # sets up the stack frame
    1227:	48 89 e5             	mov    rbp,rsp
    122a:	48 83 ec 10          	sub    rsp,0x10
    122e:	89 7d fc             	mov    DWORD PTR [rbp-0x4],edi          # loads argc (should be 2)
    1231:	48 89 75 f0          	mov    QWORD PTR [rbp-0x10],rsi         # loads argv (the Serial string to test)
    1235:	83 7d fc 02          	cmp    DWORD PTR [rbp-0x4],0x2          # if there are not two arguments falls through and calls usage()
    1239:	74 0f                	je     124a <main+0x24>                 # otherwise jumps to #%
    123b:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]
    123f:	48 8b 00             	mov    rax,QWORD PTR [rax]
    1242:	48 89 c7             	mov    rdi,rax
    1245:	e8 1f ff ff ff       	call   1169 <usage>
    124a:	48 8b 45 f0          	mov    rax,QWORD PTR [rbp-0x10]         #% here
    124e:	48 83 c0 08          	add    rax,0x8
    1252:	48 8b 00             	mov    rax,QWORD PTR [rax]
    1255:	48 89 c7             	mov    rdi,rax
    1258:	e8 3a ff ff ff       	call   1197 <checkSerial>               # calls checkSerial
    125d:	85 c0                	test   eax,eax                          # if checkSerial doesn't return 0, everything fails by jumping to #!
    125f:	75 13                	jne    1274 <main+0x4e>
    1261:	48 8d 3d a9 0d 00 00 	lea    rdi,[rip+0xda9]
    1268:	e8 c3 fd ff ff       	call   1030 <puts@plt>                  # otherwise it prints "Good Serial"
    126d:	b8 00 00 00 00       	mov    eax,0x0
    1272:	eb 11                	jmp    1285 <main+0x5f>                 # returns after here
    1274:	48 8d 3d a2 0d 00 00 	lea    rdi,[rip+0xda2]
    127b:	e8 b0 fd ff ff       	call   1030 <puts@plt>                  #! here and printing "Bad Serial"
    1280:	b8 ff ff ff ff       	mov    eax,0xffffffff
    1285:	c9                   	leave
    1286:	c3                   	ret

0000000000001197 <checkSerial>:
    1197:	55                   	push   rbp
    1198:	48 89 e5             	mov    rbp,rsp
    119b:	53                   	push   rbx
    119c:	48 83 ec 28          	sub    rsp,0x28
    11a0:	48 89 7d d8          	mov    QWORD PTR [rbp-0x28],rdi
    11a4:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]         # loads the string into rax
    11a8:	48 89 c7             	mov    rdi,rax
    11ab:	e8 90 fe ff ff       	call   1040 <strlen@plt>                # checks that it is equal to 0x10 by a call to strlen
    11b0:	48 83 f8 10          	cmp    rax,0x10                         # Jumps to #%
    11b4:	74 07                	je     11bd <checkSerial+0x26>
    11b6:	b8 ff ff ff ff       	mov    eax,0xffffffff
    11bb:	eb 62                	jmp    121f <checkSerial+0x88>
    11bd:	c7 45 ec 00 00 00 00 	mov    DWORD PTR [rbp-0x14],0x0
    11c4:	eb 3d                	jmp    1203 <checkSerial+0x6c>
    11c6:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]         #$ zeros out eax
    11c9:	48 63 d0             	movsxd rdx,eax                          # zeros rdx
    11cc:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]         # moves string into rax
    11d0:	48 01 d0             	add    rax,rdx
    11d3:	0f b6 00             	movzx  eax,BYTE PTR [rax]               # reduces string to a single byte
    11d6:	0f be d0             	movsx  edx,al                           # that is moved into edx
    11d9:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]         # rax is zeroed
    11dc:	48 98                	cdqe                                    # and extended
    11de:	48 8d 48 01          	lea    rcx,[rax+0x1]                    # 1 is moved into rcx
    11e2:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]         # string address is loaded into rax
    11e6:	48 01 c8             	add    rax,rcx                          # and incremented by rcx
    11e9:	0f b6 00             	movzx  eax,BYTE PTR [rax]               # the next letter is moved into rax
    11ec:	0f be c0             	movsx  eax,al
    11ef:	29 c2                	sub    edx,eax                          # edx - eax
    11f1:	89 d0                	mov    eax,edx                          # result is moved into eax
    11f3:	83 f8 ff             	cmp    eax,0xffffffff                   # needs to equal minus one
    11f6:	74 07                	je     11ff <checkSerial+0x68>          # if it is equal the loop continues otherwise
    11f8:	b8 ff ff ff ff       	mov    eax,0xffffffff
    11fd:	eb 20                	jmp    121f <checkSerial+0x88>          # it jumps from here to the end and returns -1
    11ff:	83 45 ec 02          	add    DWORD PTR [rbp-0x14],0x2
    1203:	8b 45 ec             	mov    eax,DWORD PTR [rbp-0x14]         #% TO HERE: zero's eax
    1206:	48 63 d8             	movsxd rbx,eax                          # sets rbx up as a counter for looping through the string
    1209:	48 8b 45 d8          	mov    rax,QWORD PTR [rbp-0x28]
    120d:	48 89 c7             	mov    rdi,rax
    1210:	e8 2b fe ff ff       	call   1040 <strlen@plt>
    1215:	48 39 c3             	cmp    rbx,rax                          # Checks that the counter is equal to the length of the string otherwise continues to loop
    1218:	72 ac                	jb     11c6 <checkSerial+0x2f>          # by jumping to #$
    121a:	b8 00 00 00 00       	mov    eax,0x0
    121f:	48 83 c4 28          	add    rsp,0x28
    1223:	5b                   	pop    rbx
    1224:	5d                   	pop    rbp
    1225:	c3                   	ret

```