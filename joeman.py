import sys


def hello(*args):
    print("called")
    print(args)


if __name__ == "__main__":
    print(*sys.argv)
