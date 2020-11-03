import r2pipe
from pprint import pprint as pp

p = r2pipe.open('./crackMe-by-m00ny-1')
p.cmd('ood 2222222222222222')
p.cmd('aa')

# break points on function calls
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
p.cmd('dr rsi=rdi')
pp(p.cmd('dc'))

pp(p.cmd('dr'))
pp(p.cmd('dc'))
