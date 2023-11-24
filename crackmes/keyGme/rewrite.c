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


