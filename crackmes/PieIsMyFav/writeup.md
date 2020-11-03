================================================================================

#### Challange: PieIsMyFav
#### Solution : 3

================================================================================

Walkthrough:
- In the main function the solution is found:
- There are 4 local variables initialized, ebp-0x4 = 0x64, ebp-0x8 = 0xe,
    ebp-0x10 = 0, and ebp-0xc = 0;
- The binary prints "Welcome to the wonderful world of assembly", through a call to puts.
- Then "Qual o numero magico? " via a call to printf
- The binary passes a %d decimal format string to scanf and stores the input (i think) on the stack in what ends up being the ebp-0x10 local variable.
- The ebp-0x4 value (0x64) is loaded into edx, this is copied into eax.
- eax is added to itself yielding 0xC8
- This is then added to edx (which contains 0x64), this yields 0x12c
- The ebp-0x8 is then moved into eax and added to edx.
- eax is then sign extended into the edx register with cdq.
- This is then divided by 0x64 (ebp-0x4)
- This should leave the quotient in eax which should be 3 and the remainder in EDX.
- The value in eax is moved into to ebp-0xc.
- The input variable that was stored at ebp-0x10 is loaded into eax
- This is then compared to 3.
- If the values not equal a jump occurs and the string "Try Harder" is past to puts.
- Else, the string "Essa eh a sua flag!" is printed.
- The main function then returns.

================================================================================
##### Code:
================================================================================
``` c
#include <stdio.h>

int main(void)
{
    int a = 0x64, b = 0xe, c = 0, input = 0;

    printf("Welcome to the wonderful world of assembly\n");
    printf("Qual o numero magico?: ");
    scanf("%d", &input);

    c = a;
    a += a;
    a += c;
    a += b;
    a /= c;

    if (input == a) printf("Essa eh a sua flag!\n");
    else printf("Try Harder\n");

    return 0;
}
```
