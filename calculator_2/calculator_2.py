from secret import FLAG

# FLAG = int.from_bytes(b"erstictf{REDACTED}", "big")

ops = {"add": "+", "sub": "-", "mul": "*", "div": "//"}

unchecked_program = []
checked_program = ""

def add_instruction():
    print("""
    ==== Instructions ====
    1. Get value of variable
    2. Set variable to value
    3. Add the second variable to the first
    4. Substract the second variable from the first
    5. Multiply the second variable to the first
    6. Divide the first variable by the second
    7. Login
    8. Logout
    ==== Instructions ====
    """)
    match(int(input("Instruction: "))):
        case 1:
            var = input("Variable: ")
            unchecked_program.append(("get", var))
        case 2:
            var = input("Variable: ")
            val = int(input("Value: "))
            unchecked_program.append(("set", var, val))
        case 3:
            first = input("First Variable: ")
            second = input("Second Variable: ")
            unchecked_program.append(("add", first, second))
        case 4:
            first = input("First Variable: ")
            second = input("Second Variable: ")
            unchecked_program.append(("sub", first, second))
        case 5:
            first = input("First Variable: ")
            second = input("Second Variable: ")
            unchecked_program.append(("mul", first, second))
        case 6:
            first = input("First Variable: ")
            second = input("Second Variable: ")
            unchecked_program.append(("div", first, second))
        case 7:
            unchecked_program.append("login")
        case 8:
            unchecked_program.append("logout")
        case _:
            print("Unknown instruction.")

def compile_program():
    global checked_program
    authenticated = False
    overwritten = False
    program = []
    for i, instruction in enumerate(unchecked_program):
        match instruction:
            case ("get", var):
                if var == "FLAG" and not authenticated and not overwritten:
                    break
                elif not var.isalnum():
                    break
                program.append(f"print(f'{var} = {{{var}}}')")
            case "login":
                authenticated = True
                program.append(f"0 if int(input('password: ')) == {FLAG} else exit(1)")
            case "logout":
                authenticated = False
            case ("set", var, val):
                if var == "FLAG":
                    overwritten = True
                elif not var.isalnum():
                    break
                program.append(f"{var} := {val}")
            case (op, first, second):
                if (first == "FLAG" or second == "FLAG") and not authenticated and not overwritten:
                    break
                elif not first.isalnum:
                    break
                elif not second.isalnum:
                    break
                program.append(f"{first} := {first} {ops[op]} {second}")
    else:
        checked_program = "[" + ",".join(program) + "]"
        print("The program was successfully compiled and can now be executed.")
        return
    print(f"The checker found an error in instruction {i}.")

def execute_program():
    eval(checked_program)

def print_program():
    for i, instruction in enumerate(unchecked_program):
        print(f"{i}: ", end="")
        match instruction:
            case ("get", var):
                print(f"get({var})")
            case "login":
                print("login")
            case "logout":
                print("logout")
            case ("set", var, val):
                print(f"{var} = {val}")
            case (op, first, second):
                print(f"{first} {ops[op]}= {second}")

def delete_instruction():
    print_program()
    index = int(input("Instruction to delete: "))
    if index < 0 or index >= len(unchecked_program):
        print("The selected index does not exist.")
    unchecked_program.pop(index)

def menu():
    print("""
    ==== Menu ====
    1. Add Instruction
    2. Delete Instruction
    3. Compile Program
    4. Execute Program
    5. Print Program
    6. Exit
    ==== Menu ====
    """)
    return int(input("choice: "))


while True:
    match menu():
        case 1:
            add_instruction()
        case 2:
            delete_instruction()
        case 3:
            compile_program()
        case 4:
            execute_program()
        case 5:
            print_program()
        case _:
            exit()
