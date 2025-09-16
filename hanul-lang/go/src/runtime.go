package main

import (
	"errors"
	"fmt"
	"io/ioutil"
	"log"
	"os"
	"strconv"
	"strings"
	"unicode/utf8"
)

type Stack[T any] struct {
	data []T
}

// Push : 스택에 값 추가
func (s *Stack[T]) Push(v T) {
	s.data = append(s.data, v)
}

// Pop : 스택에서 마지막 값 꺼내기
func (s *Stack[T]) Pop() (T, bool) {
	if len(s.data) == 0 {
		var zero T
		return zero, false // 비어있으면 false
	}
	val := s.data[len(s.data)-1]
	s.data = s.data[:len(s.data)-1]
	return val, true
}

// Peek : 마지막 값 보기 (제거하지 않음)
func (s *Stack[T]) Peek() (T, bool) {
	if len(s.data) == 0 {
		var zero T
		return zero, false
	}
	return s.data[len(s.data)-1], true
}

// IsEmpty Empty 확인
func (s *Stack[T]) IsEmpty() bool {
	return len(s.data) == 0
}

func countRune(runes []rune, target rune) (count int) {
	count = 0

	for _, r := range runes {
		if r == target {
			count++
		}
	}
	return
}

type Janghanul struct {
	data [65536]int
}

func (j *Janghanul) ParseNum(token string) (int, error) {
	runes := []rune(token)
	if runes[0] == '호' && strings.Contains(token, "엥") {
		base := countRune(runes, '에') + 2
		value := base - countRune(runes, '.')
		return value, nil
	} else if runes[0] == '하' && runes[1] == '와' {
		base := countRune(runes, '와') * -1
		value := base + countRune(runes, '.')
		return value, nil
	} else if runes[0] == '디' && runes[utf8.RuneCountInString(token)-1] == '미' {
		idx := countRune(runes, '이')
		return j.data[idx], nil
	} else {
		return 0, errors.New(token + "도 에겐같이 하네;;")
	}
}

func (j *Janghanul) ParseOp(token string) (string, error) {
	if token == "21대3" {
		return "+", nil
	} else if token == "훌쩍" {
		return "*", nil
	} else {
		return "", errors.New(token + "도 에겐같이 하네;;")
	}
}

func (j *Janghanul) GetIndex(token string) (int, error) {
	runes := []rune(token)
	if !(runes[0] == '디' && runes[utf8.RuneCountInString(token)-1] == '미') {
		return 0, errors.New(token + "도 에겐같이 하네;;")
	}
	index := countRune(runes, '이')
	return index, nil
}

func (j *Janghanul) Calculate(code string) (int, error) {
	tokens := strings.Fields(code)
	var seq []string
	for _, token := range tokens {
		if token == "21대3" || token == "훌쩍" {
			s, err := j.ParseOp(token)
			if err != nil {
				return 0, err
			}
			seq = append(seq, s)
		} else {
			s, err := j.ParseNum(token)
			if err != nil {
				return 0, err
			}
			seq = append(seq, strconv.Itoa(s))
		}
	}

	var stack Stack[string]
	i := 0
	for i < len(seq) {
		cur := seq[i]
		if cur == "*" {
			prevS, isNotEmpty := stack.Pop()
			if !isNotEmpty {
				return 0, errors.New("'*'도 에겐같이 하네;;")
			}
			if i+1 >= len(seq) {
				return 0, errors.New("'*'도 에겐같이 하네;;")
			}
			prev, err := strconv.Atoi(prevS)
			if err != nil {
				return 0, err
			}
			nxt, err := strconv.Atoi(seq[i+1])
			if err != nil {
				return 0, err
			}
			stack.Push(strconv.Itoa(nxt * prev))
			i += 2
		} else {
			stack.Push(cur)
			i += 1
		}
	}

	result := 0
	expectNum := true

	for _, s := range stack.data {
		if s == "+" {
			if expectNum {
				return 0, errors.New("'+'도 에겐같이 하네;;")
			}
			expectNum = true
		} else {
			intS, err := strconv.Atoi(s)
			if err != nil {
				return 0, err
			}
			result += intS
			expectNum = false
		}
	}
	return result, nil
}

func Type(code string) (result string) {
	tokens := strings.Fields(code)
	if len(tokens) == 0 {
		return ""
	}

	var head string
	if strings.TrimSpace(code) != "" {
		parts := strings.Fields(code) // 공백 기준 split (여러 공백 무시)
		if len(parts) > 0 {
			head = parts[0] // 첫 번째 단어
		}
	} else {
		head = ""
	}
	if strings.HasPrefix(head, "디") && strings.HasSuffix(head, "미") {
		result = "DEF"
		return
	}

	for _, token := range tokens {

		switch token {
		case "가을야구?":
			result = "IF"
			return
		case "디떨!":
			result = "MOVE"
			return
		case "서류제출":
			result = "PRINT"
			return
		case "키움아래":
			result = "INPUT"
			return
		case "에겐":
			result = "PRINTCHAR"
			return
		case "탈선린":
			result = "END"
			return
		case "30실점":
			result = "JUMP"
			return
		}
	}
	return
}

func (j *Janghanul) StripComment(line string) string {
	markers := []string{"#", "ㅋㅋ"}
	for _, marker := range markers {
		if strings.Contains(line, marker) {
			return strings.SplitN(line, marker, 2)[0]
		}
	}
	return line
}

func (j *Janghanul) CompileLine(code string) (string, error) {
	code = strings.TrimSpace(j.StripComment(code))
	if len(code) == 0 {
		return "", nil
	}

	TYPE := Type(code)

	switch TYPE {
	case "DEF":
		parts := strings.SplitN(strings.TrimSpace(code), " ", 2)
		if len(parts) != 2 {
			return "", errors.New("대입도 에겐같이 하네;;")
		}
		variable := parts[0]
		expr := parts[1]
		idx, err := j.GetIndex(variable)

		if err != nil {
			return "", err
		}
		j.data[idx], err = j.Calculate(expr)

		if err != nil {
			return "", err
		}
		return "", nil
	case "INPUT":
		if i := strings.Index(code, "키움아래"); i != -1 {
			code = code[:i] + code[i+len("키움아래"):]
		}

		expr := strings.TrimSpace(code)
		idx, err := j.GetIndex(expr)
		if err != nil {
			return "", err
		}
		var tmp int
		_, err = fmt.Scanf("%d", &tmp)
		if err != nil {
			return "", errors.New("입력도 에겐같이 하네;;")
		}
		j.data[idx] = tmp
		return "", nil
	case "PRINT":
		if i := strings.Index(code, "서류제출"); i != -1 {
			code = code[:i] + code[i+len("서류제출"):]
		}

		expr := strings.TrimSpace(code)

		newLine := strings.HasSuffix(expr, "제발")
		if newLine {
			expr = strings.Replace(expr, "제발", "", 1)
			expr = strings.TrimSpace(expr)
		}
		val, err := j.Calculate(expr)
		if err != nil {
			return "", err
		}
		if newLine {
			fmt.Println(val)
		} else {
			fmt.Print(val)
		}
		return "", nil
	case "PRINTCHAR":
		if i := strings.Index(code, "에겐"); i != -1 {
			code = code[:i] + code[i+len("에겐"):]
		}

		expr := strings.TrimSpace(code)

		newLine := strings.HasSuffix(expr, "제발")
		if newLine {
			expr = strings.Replace(expr, "제발", "", 1)
			expr = strings.TrimSpace(expr)
		}

		val, err := j.Calculate(expr)
		if err != nil {
			return "", err
		}
		if newLine {
			fmt.Printf("%c\n", rune(val))
		} else {
			fmt.Printf("%c", rune(val))
		}
		return "", nil
	case "IF":
		if !strings.Contains(code, "그러면") {
			return "", errors.New("IF도 에겐같이 하네;;")
		}
		body := strings.SplitN(code, "그러면", 2)
		head := body[0]
		tail := body[1]

		condExpr := strings.TrimSpace(strings.Replace(head, "가을야구?", "", 1))
		thenCode := strings.TrimSpace(tail)
		elseCode := ""

		if strings.Contains(thenCode, "아니면") {
			thenCode, elseCode, _ = strings.Cut(thenCode, "아니면")
			thenCode = strings.TrimSpace(thenCode)
			elseCode = strings.TrimSpace(elseCode)
		}

		condVal, err := j.Calculate(condExpr)
		if err != nil {
			return "", err
		}

		if condVal != 0 {
			r, err := j.CompileLine(thenCode)
			if err != nil {
				return "", err
			}
			return r, nil
		} else {
			if len(elseCode) > 0 {
				r, err := j.CompileLine(elseCode)
				if err != nil {
					return "", err
				}
				return r, nil
			}
		}
	case "JUMP":
		expr := strings.TrimSpace(strings.Replace(code, "30실점", "", 1))

		return expr, nil
	case "END":
		fmt.Println("\n탈선린해도 디미는 못간다 한울한울아")
		os.Exit(1)
	}

	return "", nil
}

func (j *Janghanul) Compile(code string, check bool, errors_ int) error {
	spliter := ""
	if strings.Contains(code, "\n") {
		spliter = "\n"
	} else {
		spliter = "~"
	}

	tokens := strings.Split(strings.TrimRight(code, " "), spliter)

	if len(tokens) <= 0 {
		return nil
	}

	if check {
		head := strings.Replace(tokens[0], " ", "", -1)
		tail := strings.TrimSpace(tokens[len(tokens)-1])
		if !(strings.Contains(head, "대체누가") && tail != "디미고를 서류로 떨어짐?") {
			return errors.New("이게 어떻게 에겐이냐 ㅋㅋ")
		}
	}

	index := 0
	steps := 0
	for index < len(tokens) {
		t := strings.TrimSpace(tokens[index])
		res, err := j.CompileLine(t)
		if err != nil {
			return err
		}

		intRes, err := strconv.Atoi(res)
		if err != nil {
			index += 1
			steps += 1
		} else {
			index = intRes - 1
		}
		if steps >= errors_ {
			return errors.New(fmt.Sprintf("%d번째 줄에서 무한 루프가 감지되었습니다.", index))
		}
	}
	return nil
}

func (c *Janghanul) CompilePath(path string) {
	// 파일 읽기 (UTF-8 기본)
	data, err := ioutil.ReadFile(path)
	if err != nil {
		log.Fatal(err)
	}
	code := string(data)

	// compile 메서드 호출
	c.Compile(code, true, 10000)
}
