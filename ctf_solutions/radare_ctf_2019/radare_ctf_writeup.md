### Radare CTF Writeup
### Problem: babyr2
#### Solution: r2con{c0ffee} 
#### Tools: radare2 and r2ghidra

#### _Explanation:_


Set up qemu, (this took most of my time). Use r2ghidra to decompile the binary:

    undefined8 main(int iParm1,longlong lParm2)
    {
      int iVar1;
      uint uVar2;
      size_t sVar3;

      if (iParm1 != 4) {
        puts("wr0ng length?");
                        /* WARNING: Subroutine does not return */
        exit(1);
      }
      iVar1 = atoi(*(char **)(lParm2 + 8));
      if (iVar1 != 0xcafe) {
        puts("Sorry, you\'re a baby reverser. Is it your 1st r2con? :(");
                        /* WARNING: Subroutine does not return */
        exit(2);
      }
      uVar2 = atoi(*(char **)(lParm2 + 0x10));
      if ((uVar2 % 5 != 3) && (uVar2 % 0x11 == 8)) {
        iVar1 = strcmp("radare2c0n",*(char **)(lParm2 + 0x18));
        if (iVar1 != 0) {
          puts("so close, mate! How many r2con editions did you attend so far?");
                        /* WARNING: Subroutine does not return */
          exit(4);
        }
        puts("ok,ok! you are not a baby r2 reverser!");
        sVar3 = strlen(*(char **)(lParm2 + 0x18));
        printf("The key to be a g00d reverser is: r2con{%x}\n",
               (ulonglong)((uVar2 % 0x11) * 0xb + sVar3 + 0xc0ff8c));
        return 0;
      }
      puts("I\'d say you were in Barcelona before? :)");
                        /* WARNING: Subroutine does not return */
      exit(3);
    }

You can see it needs four arguments including its own name:


1. The first needs to be 0xcafe (51966). 
2. The second needs to be obey ( x % 5 != 3) && (x % 17 == 8). 
3. The third needs to be radare2c0n

run the binary and we get the flag


#### _Solution Script:_

The python script to solve the constraint:

    for x in range(0, 1000):
        if(( x % 5 != 3) && (x % 17 == 8)):
            print(x)
            break


### Problem: r2boy
#### Solution: r2con{FLAGHIDR3NM4P}
#### Tools: visualboyadvanced-m 

#### _Explanation:_

I built visualboyadvanced-m (which is 100000 times better then debians visualboy package. In playing around with the tile viewer I found the flag in the tile viewer. It really required no reversing just looking at the available map tiles in the tile viewer.


### Problem:  r2boy1.gb
#### Solution: r2con{yay_punk4ke_found}
#### Tools: radare2, visualboyadvanced 

#### _Explanation:_
Gameboy game is a clone of a limited part of pokemon goal is to find the flag. Easy to find either just call strings on the game OR walk through the walls to find the underground cavern under the boulder and get the flag.

#### _Solution Script:_
    
    izz

    02 0x00018001 0x00054001  19  20 (rombank06) ascii Welcome to\nr2World!
    903 0x00018016 0x00054016  36  37 (rombank06) ascii Find pancake\nthrough a game\nglitch! 
    904 0x0001803c 0x0005403c  29  30 (rombank06) ascii I am pancake, the\nr2 prophet.
    905 0x0001805b 0x0005405b  11  12 (rombank06) ascii Remember...
    906 0x00018068 0x00054068  11  12 (rombank06) ascii Keep calm\n 
    907 0x00018075 0x00054075  27  28 (rombank06) ascii and always use r2\nfrom git!
    908 0x00018092 0x00054092  34  35 (rombank06) ascii here the secret\nto never sleep:\nr2
    909 0x000180be 0x000540be  18  19 (rombank06) ascii yay_punk4ke_f0wnd\n <- this 
    910 0x000180d5 0x000540d5  17  18 (rombank06) ascii New Game\nContinue
    911 0x000180e8 0x000540e8  21  22 (rombank06) ascii No Save Data\nFound...


