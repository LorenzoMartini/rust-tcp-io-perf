import runner
from threading import Thread
from time import sleep

CONST_ADDRESS1 = "euler01"
CONST_ADDRESS2 = "euler02"


def x(a, b):
    if a == 10:
        sleep(2)
    print(a, b)


def main():
    thread = Thread(target=runner.run, args=(CONST_ADDRESS1, CONST_ADDRESS2, ))
    thread2 = Thread(target=runner.run, args=(CONST_ADDRESS2, CONST_ADDRESS1, ))
    thread.start()
    thread2.start()
    thread.join()
    thread2.join()
    print("Done")


if __name__ == "__main__":
    main()
