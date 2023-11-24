### Mitre CTF 2019
#### Problem: Rebase re challenge
#### Solution: MCA{Th15_wUz_EaZy_Pe@Zy_L3m0n_SqU33z3}
#### Tools: radare2 etc

#### _Explanations:_

Steps to solve:
Challenge is packed with UPX packer, discovered by calling strings on binary when the dissasembly made no sense and the entry was high. There is a reference to UPX. So unpack with

		upx-cl -d mitre2019_REbase-fix

Then analyse in radare2. Seek to main. The file seems to have a lot of inline libc functions like printf, exit, strncmp etc.

           ; 'ZXFWtmKg'
           0x00401fc5      48b85a584657.  movabs rax, 0x674b6d745746585a
           ; 'DZCyrmC5'
           0x00401fcf      48ba445a4379.  movabs rdx, 0x35436d7279435a44
           0x00401fd9      488945b0       mov qword [var_50h], rax
           0x00401fdd      488955b8       mov qword [var_48h], rdx
           ; 'B+CiVfsy'
           0x00401fe1      48b8422b4369.  movabs rax, 0x7973665669432b42
           ; 'XUCQVfsy'
           0x00401feb      48ba58554351.  movabs rdx, 0x7973665651435558
           0x00401ff5      488945c0       mov qword [var_40h], rax
           0x00401ff9      488955c8       mov qword [var_38h], rdx
           ; 'ZRFzDU4y'
           0x00401ffd      48b85a52467a.  movabs rax, 0x793455447a46525a
           ; 'X2YCD/F5'
           0x00402007      48ba58325943.  movabs rdx, 0x35462f4443593258
           0x00402011      488945d0       mov qword [var_30h], rax
           0x00402015      488955d8       mov qword [var_28h], rdx


It loads a bunch of strings onto the stack and pases this as arguments to what I assume to be strncmp. These strings look exactly like base64 strings.	I try to base decode it but it doesn't make sense. So I look at the first function directly after the check for commandline arguments. But before the	strcmp.

           0x00401f83      55             push rbp
           |           0x00401f84      4889e5         mov rbp, rsp
           |           0x00401f87      4883ec60       sub rsp, 0x60
           |           ; argc
           |           0x00401f8b      897dac         mov dword [var_54h], edi
           |           ; argv
           |           0x00401f8e      488975a0       mov qword [var_60h], rsi
           |           0x00401f92      837dac02       cmp dword [var_54h], 2
           |       ,=< 0x00401f96      7416           je 0x401fae
           |       |   ; 0x47d04d
           |       |   ; "Usage: ./REbase flag"
           |       |   0x00401f98      488d3daeb007.  lea rdi,
           str.Usage:_._REbase_flag
           |       |   0x00401f9f      e80c790000     call printf_guess
           |       |   0x00401fa4      bf01000000     mov edi, 1
           |       |   0x00401fa9      e892640000     call exit_guess
           |       |   ; CODE XREF from main @ 0x401f96
           |       `-> 0x00401fae      488b45a0       mov rax, qword [var_60h]
           |           0x00401fb2      4883c008       add rax, 8
           |           0x00401fb6      488b00         mov rax, qword [rax]
           |           0x00401fb9      4889c7         mov rdi, rax
    This one   --->    0x00401fbc      e82cfcffff     call fcn.00401bed
           |           0x00401fc1      488945f8       mov qword [var_8h], rax

Inside it is a custom base64 encoding.

			|           0x00401bed      55             push rbp
			|           0x00401bee      4889e5         mov rbp, rsp
			|           0x00401bf1      4883ec70       sub rsp, 0x70
			|           ; arg1
			|           0x00401bf5      48897d98       mov qword [var_68h], rdi
			|           ; 0x47d008
			|           ;
    this-->	 "QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbn/+m1234567890"
			|           0x00401bf9      488d0508b407.  lea rax,
			str.QWERTYUIOPASDFGHJKLZXCVBNMqwertyuiopasdfghjklzxcvbn__m1234567890
			|           0x00401c00      488945e8       mov qword [var_18h], rax
			|           0x00401c04      488b4598       mov rax, qword [var_68h]
			|           0x00401c08      4889c7         mov rdi, rax

As luck would have it I just implemented a base64 encoding algorithm for cryptopals. So I knew it wasn't worth doing and would take to long. A quick google and this enables custom base64 decoding

		https://www.malwaretracker.com/decoder_base64.php

	MCA{Th15_wUz_EaZy_Pe@Zy_L3m0n_SqU33z3}

The last two character was messed up so I guessed the second last and the last had to be }.
