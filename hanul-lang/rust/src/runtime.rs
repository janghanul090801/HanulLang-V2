use std::collections::VecDeque;
use std::fs;
use std::io::{self, Write};

pub struct Janghanul {
    data: Vec<i32>,
}

#[derive(Debug)]
pub enum JanghanulError {
    ParseError(String),
    RuntimeError(String),
    SyntaxError(String),
    RecursionError(String),
    IOError(String),
}

impl std::fmt::Display for JanghanulError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            JanghanulError::ParseError(msg) => write!(f, "Parse Error: {}", msg),
            JanghanulError::RuntimeError(msg) => write!(f, "Runtime Error: {}", msg),
            JanghanulError::SyntaxError(msg) => write!(f, "Syntax Error: {}", msg),
            JanghanulError::RecursionError(msg) => write!(f, "Recursion Error: {}", msg),
            JanghanulError::IOError(msg) => write!(f, "IO Error: {}", msg),
        }
    }
}

impl std::error::Error for JanghanulError {}

type Result<T> = std::result::Result<T, JanghanulError>;

#[derive(Debug, PartialEq)]
enum StatementType {
    If,
    Move,
    Print,
    Input,
    Def,
    PrintChar,
    End,
    Jump,
}

impl Janghanul {
    pub fn new() -> Self {
        Self {
            data: vec![0; 65536], // 2^16
        }
    }

    fn parse_num(&self, token: &str) -> Result<i32> {
        // 양수: "호 ... 엥" (내부 규칙: '에' 개수 + 2 - '.' 개수)
        if token.starts_with("호") && token.contains("엥") {
            let base = token.chars().filter(|&c| c == '에').count() as i32 + 2;
            let dots = token.chars().filter(|&c| c == '.').count() as i32;
            return Ok(base - dots);
        }

        // 음수: "하와..." (내부 규칙: '와' 개수 * -1 + '.' 개수)
        if token.starts_with("하와") {
            let base = -(token.chars().filter(|&c| c == '와').count() as i32);
            let dots = token.chars().filter(|&c| c == '.').count() as i32;
            return Ok(base + dots);
        }

        // 변수 참조: "디이...미"
        if token.starts_with("디") && token.ends_with("미") {
            let idx = token.chars().filter(|&c| c == '이').count();
            if idx >= self.data.len() {
                return Err(JanghanulError::RuntimeError(format!(
                    "인덱스 {}가 범위를 벗어났습니다",
                    idx
                )));
            }
            return Ok(self.data[idx]);
        }

        Err(JanghanulError::ParseError(format!(
            "{}도 에겐같이 하네;;",
            token
        )))
    }

    fn parse_op(&self, token: &str) -> Result<String> {
        match token {
            "21대3" => Ok("+".to_string()),
            "훌쩍" => Ok("*".to_string()),
            _ => Err(JanghanulError::ParseError(format!(
                "{}도 에겐같이 하네;;",
                token
            ))),
        }
    }

    fn get_index(&self, token: &str) -> Result<usize> {
        if !(token.starts_with("디") && token.ends_with("미")) {
            return Err(JanghanulError::ParseError(format!(
                "{}도 에겐같이 하네;;",
                token
            )));
        }
        let idx = token.chars().filter(|&c| c == '이').count();
        if idx >= self.data.len() {
            return Err(JanghanulError::RuntimeError(format!(
                "인덱스 {}가 범위를 벗어났습니다",
                idx
            )));
        }
        Ok(idx)
    }

    fn calculate(&self, code: &str) -> Result<i32> {
        let tokens: Vec<&str> = code.split_whitespace().collect();
        let mut seq = Vec::new();

        for tok in tokens {
            if tok == "21대3" || tok == "훌쩍" {
                seq.push(self.parse_op(tok)?);
            } else {
                seq.push(self.parse_num(tok)?.to_string());
            }
        }

        // 1) 곱셈 먼저 처리
        let mut stack: Vec<String> = Vec::new();
        let mut i = 0;
        while i < seq.len() {
            let cur = &seq[i];
            if cur == "*" {
                if stack.is_empty() {
                    return Err(JanghanulError::RuntimeError(
                        "'*'도 에겐같이 하네;;".to_string(),
                    ));
                }
                if i + 1 >= seq.len() {
                    return Err(JanghanulError::RuntimeError(
                        "'*'도 에겐같이 하네;;".to_string(),
                    ));
                }
                let prev: i32 = stack
                    .pop()
                    .unwrap()
                    .parse()
                    .map_err(|_| JanghanulError::RuntimeError("계산 오류".to_string()))?;
                let nxt: i32 = seq[i + 1]
                    .parse()
                    .map_err(|_| JanghanulError::RuntimeError("계산 오류".to_string()))?;
                stack.push((prev * nxt).to_string());
                i += 2;
            } else {
                stack.push(cur.clone());
                i += 1;
            }
        }

        // 2) 덧셈 처리
        let mut result = 0;
        let mut expect_num = true;
        for s in stack {
            if s == "+" {
                if expect_num {
                    return Err(JanghanulError::RuntimeError(
                        "'+'도 에겐같이 하네;;".to_string(),
                    ));
                }
                expect_num = true;
            } else {
                let val: i32 = s
                    .parse()
                    .map_err(|_| JanghanulError::RuntimeError("계산 오류".to_string()))?;
                result += val;
                expect_num = false;
            }
        }
        Ok(result)
    }

    fn get_type(&self, code: &str) -> Option<StatementType> {
        let code = code.trim();
        if code.is_empty() {
            return None;
        }
        let head = code.split_whitespace().next().unwrap_or("");

        if code.contains("가을야구?") {
            return Some(StatementType::If);
        }
        if code.contains("디떨!") {
            return Some(StatementType::Move);
        }
        if code.contains("서류제출") {
            return Some(StatementType::Print);
        }
        if code.contains("키움아래") {
            return Some(StatementType::Input);
        }
        if head.starts_with("디") && head.ends_with("미") {
            return Some(StatementType::Def);
        }
        if code.contains("에겐") {
            return Some(StatementType::PrintChar);
        }
        if code.contains("탈선린") {
            return Some(StatementType::End);
        }
        if code.contains("30실점") {
            return Some(StatementType::Jump);
        }
        None
    }

    fn strip_comment(&self, line: &str) -> String {
        if let Some(pos) = line.find('#') {
            line[..pos].to_string()
        } else {
            line.to_string()
        }
    }

    fn remove_suffix_safe(&self, text: &str, suffix: &str) -> String {
        if text.ends_with(suffix) {
            text.strip_suffix(suffix).unwrap_or(text).to_string()
        } else {
            text.to_string()
        }
    }

    fn compile_line(&mut self, code: &str) -> Result<Option<i32>> {
        let code = self.strip_comment(code).trim().to_string();
        if code.is_empty() {
            return Ok(None);
        }

        let stmt_type = self.get_type(&code);

        match stmt_type {
            Some(StatementType::Def) => {
                let parts: Vec<&str> = code.splitn(2, ' ').collect();
                if parts.len() != 2 {
                    return Err(JanghanulError::RuntimeError(
                        "대입도 에겐같이 하네;;".to_string(),
                    ));
                }
                let var = parts[0];
                let expr = parts[1];
                let idx = self.get_index(var)?;
                self.data[idx] = self.calculate(expr)?;
                Ok(None)
            }

            Some(StatementType::Input) => {
                let expr = code.replace("키움아래", "").trim().to_string();
                let idx = self.get_index(&expr)?;
                print!("");
                io::stdout().flush().unwrap();
                let mut input = String::new();
                io::stdin()
                    .read_line(&mut input)
                    .map_err(|e| JanghanulError::IOError(e.to_string()))?;
                let val: i32 = input.trim().parse().map_err(|_| {
                    JanghanulError::RuntimeError("입력도 에겐같이 하네;;".to_string())
                })?;
                self.data[idx] = val;
                Ok(None)
            }

            Some(StatementType::Print) => {
                let mut expr = code.replace("서류제출", "").trim().to_string();
                let mut newline = false;
                if expr.ends_with("제발") {
                    newline = true;
                    expr = self.remove_suffix_safe(&expr, "제발").trim().to_string();
                }
                let val = self.calculate(&expr)?;
                if newline {
                    println!("{}", val);
                } else {
                    print!("{}", val);
                    io::stdout().flush().unwrap();
                }
                Ok(None)
            }

            Some(StatementType::PrintChar) => {
                let mut expr = code.replace("에겐", "").trim().to_string();
                let mut newline = false;
                if expr.ends_with("제발") {
                    newline = true;
                    expr = self.remove_suffix_safe(&expr, "제발").trim().to_string();
                }
                let val = self.calculate(&expr)?;
                let ch = char::from_u32(val as u32).ok_or_else(|| {
                    JanghanulError::RuntimeError("문자도 에겐같이 하네;;".to_string())
                })?;
                if newline {
                    println!("{}", ch);
                } else {
                    print!("{}", ch);
                    io::stdout().flush().unwrap();
                }
                Ok(None)
            }

            Some(StatementType::Move) => {
                let body = code.replace("디떨!", "").trim().to_string();
                let (src_tok, dst_tok) = if body.contains("->") {
                    let parts: Vec<&str> = body.splitn(2, "->").collect();
                    if parts.len() != 2 {
                        return Err(JanghanulError::RuntimeError(
                            "MOVE도 에겐같이 하네;;".to_string(),
                        ));
                    }
                    (parts[0].trim().to_string(), parts[1].trim().to_string())
                } else {
                    let parts: Vec<&str> = body.split_whitespace().collect();
                    if parts.len() != 2 {
                        return Err(JanghanulError::RuntimeError(
                            "MOVE도 에겐같이 하네;;".to_string(),
                        ));
                    }
                    (parts[0].to_string(), parts[1].to_string())
                };
                let src_idx = self.get_index(&src_tok)?;
                let dst_idx = self.get_index(&dst_tok)?;
                self.data[dst_idx] = self.data[src_idx];
                Ok(None)
            }

            Some(StatementType::If) => {
                if !code.contains("그러면") {
                    return Err(JanghanulError::RuntimeError(
                        "IF도 에겐같이 하네;;".to_string(),
                    ));
                }
                let parts: Vec<&str> = code.splitn(2, "그러면").collect();
                let cond_expr = parts[0].replace("가을야구?", "").trim().to_string();
                let mut then_code = parts[1].trim().to_string();
                let mut else_code = None;
                if then_code.contains("아니면") {
                    let tc_clone = then_code.clone();
                    let else_parts: Vec<&str> = tc_clone.splitn(2, "아니면").collect();
                    then_code = else_parts[0].trim().to_string();
                    else_code = Some(else_parts[1].trim().to_string());
                }
                let cond_val = self.calculate(&cond_expr)?;
                if cond_val != 0 {
                    let r = self.compile_line(&then_code)?;
                    Ok(r)
                } else if let Some(else_code) = else_code {
                    let r = self.compile_line(&else_code)?;
                    Ok(r)
                } else {
                    Ok(None)
                }
            }

            Some(StatementType::Jump) => {
                let expr = code.replace("30실점", "").trim().to_string();
                let target: i32 = expr.parse().map_err(|_| {
                    JanghanulError::RuntimeError(format!("{}도 에겐같이 하네;;", expr))
                })?;
                Ok(Some(target))
            }

            Some(StatementType::End) => {
                println!("\n탈선린해도 디미는 못간다 한울한울아");
                std::process::exit(0);
            }

            None => Ok(None),
        }
    }

    pub fn compile(&mut self, code: &str, check: bool, errors: i32) -> Result<()> {
        let splitter = if code.contains('\n') { '\n' } else { '~' };
        let lines: Vec<&str> = code.trim_end().split(splitter).collect();

        if lines.is_empty() {
            return Ok(());
        }

        if check {
            let head = lines[0].replace(" ", "");
            let tail = lines.last().unwrap().trim();
            if !head.starts_with("대체누가") || tail != "디미고를 서류로 떨어짐?" {
                return Err(JanghanulError::SyntaxError(
                    "이게 어떻게 에겐이냐 ㅋㅋ".to_string(),
                ));
            }
        }

        let mut index = 1i32; // 1부터 시작 (1-based 인덱싱)
        let mut steps = 0i32;
        let max_lines = lines.len() as i32;

        while index <= max_lines {
            let array_index = (index - 1) as usize;
            if array_index >= lines.len() {
                break;
            }

            let c = lines[array_index].trim();
            let res = self.compile_line(c)?;

            // int 리턴만 점프 (1-based)
            if let Some(target) = res {
                if target < 1 || target > max_lines {
                    return Err(JanghanulError::RuntimeError(format!(
                        "점프 대상 {}가 유효하지 않습니다 (1-{})",
                        target, max_lines
                    )));
                }
                index = target;
            } else {
                index += 1;
            }

            steps += 1;
            if steps >= errors {
                return Err(JanghanulError::RecursionError(format!(
                    "{}번째 줄에서 무한 루프가 감지되었습니다.",
                    index
                )));
            }
        }
        Ok(())
    }

    pub fn compile_path(&mut self, path: &str) -> Result<()> {
        let code = fs::read_to_string(path).map_err(|e| JanghanulError::IOError(e.to_string()))?;
        self.compile(&code, true, 100000)
    }
}

impl Default for Janghanul {
    fn default() -> Self {
        Self::new()
    }
}
