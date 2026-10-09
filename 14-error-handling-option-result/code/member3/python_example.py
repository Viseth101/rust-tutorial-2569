def parse_and_double(text: str) -> int:
    number = int(text)
    return number * 2

input = "abc"
try:
    result = parse_and_double(input)
    print(f"Success: {result}")
except ValueError as err:
    print(f"Parsing Failure: {err}")
