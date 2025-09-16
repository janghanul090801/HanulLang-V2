package main

import (
	"fmt"
	"os"
	"strings"
)

func main() {
	if len(os.Args) < 2 {
		fmt.Println("사용법: 프로그램 <옵션> <파일명>")
		os.Exit(1)
	}

	filename := os.Args[1]
	if !strings.HasSuffix(filename, ".eagen") {
		fmt.Println("확장자는 .eagen으로 되어야 함")
		os.Exit(1)
	}

	data, err := os.ReadFile(filename)
	if err != nil {
		fmt.Println("파일을 열 수 없습니다:", err)
		os.Exit(1)
	}
	code := string(data)

	interpreter := Janghanul{}
	err = interpreter.Compile(code, true, 10000) // check=true, errors_=10000
	if err != nil {
		fmt.Println("컴파일 오류:", err)
		os.Exit(1)
	}
}
