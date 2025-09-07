from runtime import janghanul   # runtime.py 안에 있는 클래스 import

# VM 인스턴스 만들기
vm = janghanul()

print("===== HanulLang 테스트 시작 =====")

vm.compileLine("디미 호에엥")
vm.compileLine("디이미 호에에엥")
vm.compileLine("디이이이미 하와와와")
vm.compileLine("서류제출 디미")
vm.compileLine("서류제출 디이미")
vm.compileLine("서류제출 디이이이미")


print("\n===== HanulLang 테스트 끝 =====")