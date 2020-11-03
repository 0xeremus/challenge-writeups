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


/* Function checks length and then checks if each pair of values in the key when
subtracted from one another equals -1 */
int checkSerial(char *key)
{
    /* Checks the length of the key */
    if (strlen(key) != 0x10)
        return false;

    /* Checks that the key obeys the key algorithm, namely that each pair of 
    values is equal to -1 */
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
    printf("./SimpleKeyGen [SERIAL]\n");
    exit(0);
}

