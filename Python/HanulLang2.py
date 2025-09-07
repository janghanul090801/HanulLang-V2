__vars = []
__reading = 0 # 읽고있는 코드 인덱스

__ERROR_MSG = "대체 누가 한울랭을 이렇게 써버림?"

# [ Public Functions ]

## 한울랭 실행
def run(code: str) -> None:
    global __vars, __reading
    code = __removeComments(code) # 주석 제거
    codeList = code.split("\n") # 줄로 나뉜 코드

    
    # 시작-끝 조건 확인
    if not((codeList[0] == "대체 누가") and codeList[-1] == "디미고를 서류로 떨어짐?"):
        print(__ERROR_MSG)
        return -1
    codeList = codeList[1:-1] # 시작 끝 제거

    # 코드 읽기
    while __reading < len(codeList):
        tokens = codeList[__reading].split(" ")
        command = tokens[0]
        
        if __isIn(list("디미"), command):
            value = __calculate(tokens[1:])
            __createVar(command, value)
        



        __reading += 1

            

# [ Test Functions ]
def calculate(token: str) -> int:
    return __calculate(token)

def printVars() -> None:
    print(__vars)

def symbolizeCalculation(tokens: list[str]) -> list[str]:
    return __symbolizeCalculation(tokens)

# [ Private Functions ]

def __catchError(function):
    def wrapper(*args, **kwargs):
        try:
            return function(*args, **kwargs)
        except Exception as e:
            print(e)
            print(f"{__ERROR_MSG} {function.__name__}")
            exit()
            
    return wrapper

def __removeNull(tokens: list[str]) -> list[str]:
    result = []
    for token in tokens:
        if token != "": result.append(token)
    return result

## 주석제거
def __removeComments(code: str) -> str:
    result = ""
    isComment = False
    for char in code:
        if char == "#":
            isComment = True
        elif char == "\n":
            isComment = False
        if not(isComment): result += char
    
    return result

## 여러 문자가 문자열 안에 있는지 확인
def __isIn(checkings: list, target: str) -> bool:
    for checking in checkings:
        if checking not in target: break
    else: return True # for문이 정상종료됨
    return False # for문이 break로 종료됨

## 변수 생성
@__catchError
def __createVar(varName: str, value: int) -> None:
    varIndex = len(varName) - 2
    if varIndex >= len(__vars):
        for _ in range(varIndex - len(__vars) + 1): __vars.append(None)
    __vars[varIndex] = value
## 변수 가져왹
@__catchError
def __getVar(varIndex: int) -> int:
    if varIndex >= len(__vars): raise
    return __vars[varIndex]

## 숫자 계산하는거
@__catchError
def __calculate(tokens: list[str]) -> int:
    if len(tokens) == 0: raise Exception(f"숫자가 없음 {tokens}")
    
    firstSymbols = __symbolizeCalculation(tokens)
    secondSymbols = []
    index = 0
    while index < len(firstSymbols):
        symbol = firstSymbols[index]
        value = 0
        if symbol == "*":
            secondSymbols.pop()
            value = int(firstSymbols[index-1]) * int(firstSymbols[index+1])
            value = str(value)
            index += 1
        else: value = symbol
        secondSymbols.append(value)
        index += 1
    result = 0
    for symbol in secondSymbols:
        if symbol != "+": result += int(symbol)
    
    return result
    
def __symbolizeCalculation(tokens: list[str]) -> list[str]:
    number = "" # 항
    symbols = [] # 계산할 값을 기호화를 먼저 함
    for token in tokens:
        value = None # 호엥, 하와, 디미고 계산값
        if __isIn(list("디미고"), token): # 변수
            value = __getVar(len(token) - 3)
        elif __isIn(list("호엥"), token): # 양수
            value = len(token) - token.count(".") * 2
        elif __isIn(list("하와"), token): # 음수
            value = token.count(".") * 2 - len(token)
        
        elif token == "뿌엑": # 더하기
            symbols.append(number)
            symbols.append("+")
            number = ""
        elif token == "훌쩍": # 곱하기
            symbols.append(number)
            symbols.append("*")
            number = ""
        else: raise Exception(f"token : {token}")
        
        if value == None: continue
        elif value < 0:
            number += str(value * -1)
            number = str( -1 * int(number) )
        else: number += str(value)
    if number != "": symbols.append(number)
    return symbols
        