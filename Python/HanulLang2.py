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

    # 코드 읽기
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

# 숫자 계산하는거
def __calculate(tokens: list[str]) -> int:
    """
    호[.]*엥.* : 호에엥 - count(".")
    하[.]*와.* : 하와와 + count(".")

    "훌쩍"이 하나라도 있으면 십진수 표기법으로, 없으면 곱셈 표기법으로 계산함.

    [훌쩍, 호에엥, 하와와] 세 개에서 판단 안되는 토큰은 에러로 처리함.
    """
    if len(tokens) == 0: raise Exception(f"숫자가 없음 {tokens}")
    
    isDecimal = False
    value = None
    resultValue = 1 # return값
    
    for token in tokens:
        if token == "훌쩍":
            isDecimal = True

        # 양수
        elif __isIn(list("호엥"), token):
            term = len(token) - token.count(".") * 2 # 수학에서 "항" 뜻함
            if isDecimal:
                value = str(value)
                if term < 0:
                    value += str(term * -1)
                    value = int(value) * -1
                else:
                    value += str(term)
                    value = int(value)
                isDecimal = False
            else:
                if value != None:
                    resultValue *= value
                value = term

        # 음수
        elif __isIn(list("하와"), token):
            term = len(token) * -1 + token.count(".") * 2 # 수학에서 "항" 뜻함
            if isDecimal:
                value = str(value)
                if term < 0:
                    value += str(term * -1)
                    value = int(value) * -1
                else:
                    value += str(term)
                    value = int(value)
                isDecimal = False
            else:
                if value != None:
                    resultValue *= value
                value = term
        
        else:
            raise Exception(f"이건 숫자가 아님 : {token}/{tokens}")
    resultValue *= value

    return resultValue