================================================================================

##### Challange : Sh4ll1 
##### Answer : 540

================================================================================

##### Steps to solving:
- Open the binary in radare2 [r2 ./crackme1.bin]
- Analyze all [aaa]
- view available functions with [afl]
- seek to main [s sym.main]
- main calls two functions; first sym.systemv; then sym.systemo;
- seek to sym.systemv; 
- see that it sets three variables; rbp-0x4 = 5; rbp-0x8 = 7; and rbp-0xc = 501.
- seek to sym.stemo; This is where the password is tested.
- We can see that 7 is loaded into eax, and this is then added to rbp-0x4 yielding 0xc.
- This value at rbp-0x4 is then moved into eax.
- This value is then multiplied by 0x2d yielding 540 and the result is stored in rbp-0xc.
- rbp-0x10 is set to 0, the string "Password:" is printed; and then input is taken via stdin (or whatever the c++ equivalent is).
- The input is moved into eax and tested against the value 540 (that is stored in rbp-0xc).
- If the input is not equal it jumps to 0x55c6f63e4a62 and prints "Bad Password".
- Otherwise it falls through to 0x55c6f63e4a38 prints "Good Password" and returns.

================================================================================
##### Code:
================================================================================
```c
int main(void)
{   
    int array[4];

    set_variables(array);

    check_password(array);

    return 0;
}

// sym.systemv: Sets the variables x, y and z and pushes them onto the stack
void set_variables(int array[])
{
   array[0] = 5;
   array[1] = 7;
   array[2] = 501;
}

// computes tested password variable with x and y and pushes it into the z variable 
// stack position; then tests the user inputed password against this value.
void check_password(int array[])
{
    int input;



    array[1] += array[0];
    array[1] *= 45;
    array[2] = array[1];

    input = 0;

    printf("Password: ");
    scanf("%d", &input);

    if (input != array[2]) printf("Bad password\n");
    else printf("Good password\n");
}
```