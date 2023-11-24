
#include <stdio.h>

void set_variables(int array[]);
void check_password(int array[]);

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
