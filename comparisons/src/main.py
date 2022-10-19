
def main():
    user_input: str = get_user_input();
    print("You typed: {}".format(user_input));


def get_user_input() -> str:
    string_from_stdin = ''
    print("Please type a number between 0-100 and press enter:")
    while True:
        string_from_stdin = input() 
        trimmed = string_from_stdin.strip()
        try: # try to convert to int
            parsed = int(trimmed)
            if parsed < 0 or parsed > 100:
                print("Please type a number between 0-100 and press enter:")
                continue
            else:
                return str(trimmed)
        except ValueError:
            print("Please type a number between 0-100 and press enter:");
            continue

if __name__ == "__main__":
    main()