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
