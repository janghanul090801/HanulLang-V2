import HanulLang2

code = """대체 누가

디미 호엥 호에엥.. 훌쩍 호에엥... 훌쩍 호에엥.... 호엥 # 이건 주식임
디이미 호엥 # 주석임
디이이이미 디미고
디이이이이이미 디이이이미고 호엥..# 이렇게 붙어있어도 인식할까?

디미고를 서류로 떨어짐?"""

# HanulLang2.run(code)
# HanulLang2.printVars()
print(HanulLang2.calculate("호엥 뿌엑 호에엥 뿌엑 호에에에엥 뿌엑 호에에에에에엥 뿌엑 호에엥.. 호에엥..".split(" ")))