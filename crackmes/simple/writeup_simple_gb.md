##### ------------------------------------------------------------------------

#### PROBLEM : simple.gb 
I found this on MEGABEETS Blog, it looked really interesting so we revered it during a RE club session. GB is a intesting ISA, luckly my friend new z80, so was straight forwards. 
#### Solution: 95713

##### ------------------------------------------------------------------------
##### Working:
##### ------------------------------------------------------------------------

Started playing the game. Then looking at instruction information, strings and so. Then found our way to the interesting parts. Identified the paths through the functions and found the information below.

The first number is 3 based off:

 0x00000281      fe03           cp 0x03
 0x00000283      c2e402         jp nZ, 0x02e4     ; 0x02e4 is FAIL, jump taken if number not 3

The second number is 7 based off:

 0x00000294      fe07           cp 0x07
 0x00000296      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 7

The third number is 5 based off:

 0x000002a6      fe05           cp 0x05
 0x000002a8      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 5

The fourth number is 1 based off

0x000002bd      fe01           cp 0x01
0x000002bf      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 1

The fifth number is 9 :

0x000002ce      fe09           cp 0x09
0x000002d0      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 9

##### ------------------------------------------------------------------------
##### Functions Annotation:
##### ------------------------------------------------------------------------
```asm
/ (fcn) Input_check 107
|   Input_check ();
|           ; var int32_t var_2h @ sp+0x2
|           ; CALL XREF from fcn.00000274 (+0x2a0)
|           0x00000274      f802           ld hl, sp + 0x02
|           0x00000276      5e             ld e, [hl]
|           0x00000277      23             inc hl
|           0x00000278      56             ld d, [hl]           ; 16bit address loaded into two 8 bit registers
|           0x00000279      210400         ld hl, 0x0004        ; four loaded into register
|           0x0000027c      19             add hl, de           ; four added to address (index 5)
|           0x0000027d      4d             ld c, l
|           0x0000027e      44             ld b, h
|           0x0000027f      0a             ld a, [bc]
|           0x00000280      4f             ld c, a
|           0x00000281      fe03           cp 0x03
|       ,=< 0x00000283      c2e402         jp nZ, 0x02e4     ; 0x02e4 is FAIL, jump taken if number not 3 at index 4 of key
|      ,==< 0x00000286      1803           jr 0x03
..
|     |||   ; CODE XREF from Input_check (0x286)
|     |`--> 0x0000028b      f802           ld hl, sp + 0x02
|     | |   0x0000028d      4e             ld c, [hl]
|     | |   0x0000028e      23             inc hl
|     | |   0x0000028f      46             ld b, [hl]       ; address loaded
|     | |   0x00000290      03             inc bc           
|     | |   0x00000291      03             inc bc           ; address incremented twice (index 3)
|     | |   0x00000292      0a             ld a, [bc]
|     | |   0x00000293      4f             ld c, a
|     | |   0x00000294      fe07           cp 0x07
|     |,==< 0x00000296      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 7 at index 2
|    ,====< 0x00000299      1803           jr 0x03
..
|   |||||   ; CODE XREF from Input_check (0x299)
|   |`----> 0x0000029e      f802           ld hl, sp + 0x02
|   | |||   0x000002a0      4e             ld c, [hl]
|   | |||   0x000002a1      23             inc hl
|   | |||   0x000002a2      46             ld b, [hl]       ; address loaded 
|   | |||   0x000002a3      03             inc bc           ; incremented once 2nd number
|   | |||   0x000002a4      0a             ld a, [bc]   
|   | |||   0x000002a5      4f             ld c, a
|   | |||   0x000002a6      fe05           cp 0x05
|   |,====< 0x000002a8      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 5 at index 1
|  ,======< 0x000002ab      1803           jr 0x03
..
| |||||||   ; CODE XREF from Input_check (0x2ab)
| |`------> 0x000002b0      f802           ld hl, sp + 0x02
| | |||||   0x000002b2      5e             ld e, [hl]
| | |||||   0x000002b3      23             inc hl
| | |||||   0x000002b4      56             ld d, [hl]
| | |||||   0x000002b5      210300         ld hl, 0x0003        ; three added to address 4th number
| | |||||   0x000002b8      19             add hl, de
| | |||||   0x000002b9      4d             ld c, l
| | |||||   0x000002ba      44             ld b, h
| | |||||   0x000002bb      0a             ld a, [bc]
| | |||||   0x000002bc      4f             ld c, a
| | |||||   0x000002bd      fe01           cp 0x01
| |,======< 0x000002bf      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 1 at index 4
| ========< 0x000002c2      1803           jr 0x03
..
| |||||||   ; CODE XREF from Input_check (0x2c2)
| --------> 0x000002c7      f802           ld hl, sp + 0x02
| |||||||   0x000002c9      4e             ld c, [hl]
| |||||||   0x000002ca      23             inc hl
| |||||||   0x000002cb      46             ld b, [hl]
| |||||||   0x000002cc      0a             ld a, [bc]       ; address loaded unmodified 
| |||||||   0x000002cd      4f             ld c, a
| |||||||   0x000002ce      fe09           cp 0x09
| ========< 0x000002d0      c2e402         jp nZ, 0x02e4 ; 0x02e4 is FAIL, jump taken if number not 9 at index 0
| ========< 0x000002d3      1803           jr 0x03
..
| |||||||   ; WIN POINT
| |||||||   ; CODE XREF from Input_check (0x2d3)
| --------> 0x000002d8      21ee02         ld hl, 0x02ee  ; This is the WIN point if we got here then we are good.
| |||||||   0x000002db      e5             push hl
| |||||||   0x000002dc      cd660f         call fcn.00000f66
| |||||||   0x000002df      e802           add sp, 0x02
| ========< 0x000002e1      c3ed02         jp 0x02ed
| |||||||   ; FAIL POINT
| |||||||   ; XREFS: CODE 0x00000283  CODE 0x00000288  CODE 0x00000296  CODE 0x0000029b  CODE 0x000002a8  CODE 0x000002ad
| |||||||   ; XREFS: CODE 0x000002bf  CODE 0x000002c4  CODE 0x000002d0  CODE 0x000002d5
| ```````-> 0x000002e4      21f302         ld hl, 0x02f3  ; FAIL point if we are here then we are not so good.
|           0x000002e7      e5             push hl
|           0x000002e8      cd660f         call fcn.00000f66
|           0x000002eb      e802           add sp, 0x02
|           ; CODE XREF from Input_check (0x2e1)
\ --------> 0x000002ed      c9             ret
```
