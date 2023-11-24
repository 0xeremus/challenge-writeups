### Plaid Parliment of Pwning
#### Problem: Plaid Party Planning 3
#### Solution: PCTF{1 l1v3 1n th3 1nt3rs3ct1on of CSP and s3cur1ty and parti3s!}

#### _Explanation:_

I'm sure this was the 'easy bypass' that everyone talks about. I did not find it easy. I realized there was racy threads that were locking up the binary, OR abort was being called. If abort wasn't called it ended up with all of the threads being locked up waiting for each other.

So I just patched it all out. I tried patching out various pthread functions to less or greater success. The first time I got the result, it was unreproducible, but this script works every time.

bluepichu: It's a flag!
strikeskids: Let me take a look. It seems to say
	PCTF{1 l1v3 1n th3 1nt3rs3ct1on of CSP and s3cur1ty and parti3s!}.
strikeskids: Hopefully that's useful to someone.


#### _Solution Script:_

    aaa
    wa ret @ sym.imp.abort
    wa ret @ sym.imp.nanosleep
    wa ret @ sym.imp.pthread_mutex_lock
    dc

