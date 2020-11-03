##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-

#### Problem: crackME_by_M00ny
#### Solution: patch call to string compare, since there is lots of solutions this is easiest.

##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Explanation:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
*this is the laziest crackme solution I've ever given "mea culpa"

Call to rand, returns a series of values that are pushed onto the stack, this is the flag there is no variation regardless of the password given, it can just be read from the stack.

Eventually there is a call to string compare to see is the password is the same as a value that is computed based of  the password input, if it isn't we get an error message.


##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
##### Key Gen Script:
##### =-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-
``` python
#! /usr/bin/python3

import r2pipe
from pprint import pprint as pp

p = r2pipe.open('./crackMe-by-m00ny-1')
p.cmd('ood 2222222222222222')
p.cmd('aa')

# break points on function calls
# aren't needed just give you insight into the binary
p.cmd('db main+0xda')
p.cmd('db main+0x135')
p.cmd('db main+0x13a')

#dump function call values
for i in range(0, 31):
    pp(p.cmd('dr'))
    pp(p.cmd('pd 1'))
    p.cmd('S')
    pp(p.cmd('dr rax'))
    p.cmd('dc')
    print('\n')

p.cmd('dc')

x = p.cmdj('drj')

#this is pretty much the whole solution
p.cmd('dr rsi=rdi')
pp(p.cmd('dc'))

pp(p.cmd('dr'))
pp(p.cmd('dc'))
```
