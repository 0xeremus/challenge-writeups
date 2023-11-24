##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem: crackmenot
#### Solution: Each byte of the input'd name is increased by 5, giving the password.

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Explanation:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-


Solution: The program asks you to enter your name, then it asks you to enter your password. The password is derived from adding 5 to each character of your name.

        Example;
            Name = Calo
            Password =  Hfqt

The program will then print "Great Haxor Skillz" if the password and modified name do not match it will print

        "Wrong Credentials, "

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Key Gen Script:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

``` python
#! /usr/bin/python3

def main():
    # Get name
    name = input("What is your name: ")

    # Get password from name
    password = ''
    for i in name:
        password += chr(ord(i) + 5)

    print("For name {} your password is {}".format(name, password))


if __name__=='__main__':
    main()
```
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Commented Assembly:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

``` asm
(fcn) entry0 301
entry0 ();
       0x00401000      b801000000     mov eax, 1
       0x00401005      bf01000000     mov edi, 1
       0x0040100a      48be00204000.  movabs rsi, str.Please_enter_your_name:   ; "Please enter your name: "
       0x00401014      ba19000000     mov edx, 0x19                             ; Prints ""Please enter your name: "
       0x00401019      0f05           syscall
       0x0040101b      b800000000     mov eax, 0
       0x00401020      bf00000000     mov edi, 0
       0x00401025      48be74204000.  movabs rsi, loc.buf
       0x0040102f      ba20000000     mov edx, 0x20
       0x00401034      0f05           syscall                                   ; Takes input of name stores it in loc.buf
       0x00401036      4883f800       cmp rax, 0
   ,=< 0x0040103a      0f8cd4000000   jl loc._start.error                       ; Makes sure that a name was given
   |   0x00401040      4989c6         mov r14, rax
   |   0x00401043      4983c606       add r14, 6
   |   0x00401047      488b04251920.  mov rax, qword str.Hello                  ; moves Hello into rax
   |   0x0040104f      488904259420.  mov qword [loc.welcome], rax              ; moves Hello into loc.welcome
   |   0x00401057      488b04257420.  mov rax, qword [loc.buf]                  ; moves name into rax
   |   0x0040105f      488904259a20.  mov qword [0x40209a], rax                 ; moves name into an offset of loc.welcome
   |   0x00401067      b801000000     mov eax, 1
   |   0x0040106c      bf01000000     mov edi, 1
   |   0x00401071      48be94204000.  movabs rsi, loc.welcome     ; 0x402094
   |   0x0040107b      4c89f2         mov rdx, r14
   |   0x0040107e      0f05           syscall                                   ; Prints "Hello Name"
   |   0x00401080      b801000000     mov eax, 1
   |   0x00401085      bf01000000     mov edi, 1
   |   0x0040108a      48be20204000.  movabs rsi, str.Enter_your_Password:      ; "Enter your Password: "
   |   0x00401094      ba16000000     mov edx, 0x16
   |   0x00401099      0f05           syscall                                   ; Prints "Enter your Password: "
   |   0x0040109b      b800000000     mov eax, 0
   |   0x004010a0      bf00000000     mov edi, 0
   |   0x004010a5      48be74204000.  movabs rsi, loc.buf                       ; Moves Name into rsi
   |   0x004010af      ba20000000     mov edx, 0x20
   |   0x004010b4      0f05           syscall                                   ; Takes input of password
   |   0x004010b6      4989c7         mov r15, rax                              ; Moves length of password into r15
   |   0x004010b9      49ffcf         dec r15
  .--> 0x004010bc      4d89fe         mov r14, r15
  :|   0x004010bf      4983c605       add r14, 5
  :|   0x004010c3      418a86942040.  mov al, byte [r14 + loc.welcome]          ; Moves a byte of the loc.welcome buffer into al
  :|   0x004010ca      0405           add al, 5                                 ; adds 5 to it ('C' becomes 'H')
  :|   0x004010cc      413a87732040.  cmp al, byte [r15 + 0x402073]             ; Compares it to the password
 ,===< 0x004010d3      7522           jne loc._start.wrong                      ; Branches to error and return if not equal
 |:|   0x004010d5      49ffcf         dec r15                                   ; Continues for length of password
 |`==< 0x004010d8      75e2           jne loc._start.l1
 | |   0x004010da      b801000000     mov eax, 1
 | |   0x004010df      bf01000000     mov edi, 1
 | |   0x004010e4      48be53204000.  movabs rsi, str.e_32mGreat_H4x0r_Skillz   ; If password and name matches then it prints "Great H4x0r Skillz!"
 | |   0x004010ee      ba18000000     mov edx, 0x18
 | |   0x004010f3      0f05           syscall
 |,==< 0x004010f5      eb25           jmp loc._start.exit                       ; and exits
 |||   ;-- _start.wrong:
 |||   ; CODE XREF from entry0 (0x4010d3)
 `---> 0x004010f7      b801000000     mov eax, 1
  ||   0x004010fc      bf01000000     mov edi, 1
  ||   0x00401101      48be36204000.  movabs rsi, str.e_31mWrong_Credentials    ; If wrong it prints "Wrong Credentials, "
  ||   0x0040110b      ba18000000     mov edx, 0x18               ; 24
  ||   0x00401110      0f05           syscall
 ,===< 0x00401112      eb08           jmp loc._start.exit                       ; and exits
 |||   ;-- _start.error:
 |||   ; CODE XREF from entry0 (0x40103a)
 ||`-> 0x00401114      488904257020.  mov qword [loc.ret_code], rax ; [0x402070:8]=0
 ||    ;-- _start.exit:
 ||    ; CODE XREFS from entry0 (0x4010f5, 0x401112)
 ``--> 0x0040111c      b83c000000     mov eax, 0x3c               ; '<' ; 60
       0x00401121      48bf70204000.  movabs rdi, loc.ret_code    ; 0x402070
       0x0040112b      0f05           syscall
```
