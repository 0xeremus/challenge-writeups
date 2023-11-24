
with open("./flag_val_needed.txt", "r") as flag_val_unsep:
    vals = flag_val_unsep.readline().strip("\n").split(" ")

transformed_array = []
for i in vals:
     try:
         transformed_array.append(chr(int(i) >> 4))
     except:
         print(f"i:{i} not convertable to int")
print(''.join([i for i in transformed_array]))
