// 重影（Shadowing）
// 重影的概念与其他面向对象语言里的"重写"（Override）或"重载"（Overload）是不一样的。
// 重影就是指变量的名称可以被重新使用的机制：
// 重影与可变变量的赋值不是一个概念，重影是指用同一个名字重新代表另一个变量实体，其类型、可变属性和值都可以变化。但可变变量赋值仅能发生值的变化。
// 这一点类似Python

pub fn shadowing_foo() {
    let x = 5;
    let x = x + 1;
    let x = x * 2;
    println!("The value of x is: {}", x);
}
pub fn shadowing_foo1() {
    let mut s = "123";
    let mut s = "1234";
    let s = s.len();
    println!("shadowing_foo1,The value of x is: {}", s);
}
