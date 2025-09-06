__vars = []
__codeList = [] # 줄 단위로 나뉜 코드
__reading = 0 # 읽고있는 코드 인덱스


# [ Public Functions ]

# 한울랭 실행
def run(code: str) -> None:
    global __vars, __codeList, __reading

    __codeList = code.split("\n")

    # 시작-끝 조건 확인
    if not((__codeList[0] == "대체 누가") and __codeList[-1] == "디미고를 서류로 떨어짐?"):
        print("대체 누가 한울랭을 이렇게 써버림?")
        return -1
    __codeList = __codeList[1:-1] # 시작 끝 제거

    while __reading < len(__codeList):
        tokens = __codeList[__reading].split(" ")
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

# [ Private Functions ]

# 여러 문자가 문자열 안에 있는지 확인
def __isIn(checkings: list, target: str) -> bool:
    for checking in checkings:
        if checking not in target: break
    else: return True # for문이 정상종료됨
    return False # for문이 break로 종료됨


def __createVar(varName: str, value: int) -> None:
    varIndex = len(varName) - 2
    if varIndex <= len(__vars):
        for _ in range(len(__vars) - varIndex + 1): __vars.append(None)
    __vars[varIndex] = value

def __calculate(tokens: list[str]) -> int:
    if len(tokens) == 0: raise Exception(f"숫자가 없음 {tokens}")
    
    isDecimal = False
    isPositive = True
    resultValue = 1
    
    if "훌쩍" in tokens: 
        isDecimal = True
        resultValue = ""
    
    for token in tokens:
        # 양수
        if __isIn(list("호엥"), token):
            value = len(token) - token.count(".") * 2
            if isDecimal:
                if value >= 0: resultValue += str(value)
                else:
                    resultValue += str(value * -1)
                    isPositive = not(isPositive)
            else:
                resultValue *= value
        # 음수
        elif __isIn(list("하와"), token):
            value = -1 * len(token) + token.count(".") * 2
            if isDecimal:
                if value >= 0: resultValue += str(value)
                else:
                    resultValue += str(value * -1)
                    isPositive = not(isPositive)
            else:
                resultValue *= value
        # 얜 뭘까
        elif token == "훌쩍": pass
        else: raise Exception(f"이건 숫자가 아님! : {token}")
    
    if isDecimal:
        return int(resultValue) if isPositive else -1 * int(resultValue)
    return resultValue