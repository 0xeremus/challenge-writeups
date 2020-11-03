================================================================================

#### Solutions for PrettyDamnEasy ::
#### Answer: easycrack

================================================================================

##### Steps:
- Open the binary in r2
- Analyse binary: "aaa"
- Seek to main: s main
- At the very beginning it is possible to see that the string "easycrack" is 
    saved across two variables rbp-0x14 and rpb-0xc
- At this point if you try that as a password, you are done. However the binary continues
- The string "Input Password:" is passed to printf
- A "%s" string is expected by scanf and input is taken,
- The binary then tests the input string to the string that was stored in rbp-0x14 and rpb-0xc, 
    it does this by calling stringcmp after the strings are loaded into rsi and rdi respectively.
- If the result is not equal it jumps, and prints "Wrong Password, try another"
- Otherwise if the input string is equal to "easycrack" (the string we saw at 
    the beginning) it continues and prints "Correct Password"

================================================================================
##### Code:
================================================================================
```c
#include <stdio.h>
#include <string.h>

#define MAX 48 // Arbitary I have no idea what this should be

int main(void)
{
    char password[MAX] = "easycrack\n", test[MAX];
    
     printf("Input the password: ");
     scanf("%s", test);

     if (!strcmp(test, password)) printf("Wrong password, try another.\n");
     else printf("Correct password\n");
}
