fn main() {
    simple_logger::init().unwrap();

    let code = r#"

# 这里是注释,
# 注释以# 开始, #前面不能有空格
# if 和 for 里面的表达式运算结果都是int类型 0 为假  非0 为真

i = 9
for 0!=i {
    j = 10- i
    for 0!=j {
        m = 10 - i

# 注释
# 下面这行代码有点奇怪 由于目前没有做运算符优先级 默认会以第一个运算符分为两边做运算
# 11 -i+j  会被解析为 11 - (i+j)

        n = 11 - i + j
        print(n)
        print(" x ")
        print(m)
        print(" = ")
        print(m*n)
        print("    ")
        j = j - 1
    }
    println("")
    i = i - 1
}





"#
    .to_string();

    chen_lang::run(code).unwrap();
}
