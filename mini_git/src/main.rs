//src/main.rs
#![allow(clippy::bool_assert_comparison)]//允许if a == true 这种写法
#![allow(clippy::bool_comparison)]       //允许if a == false 这种写法
                                         //声明模块
mod cli;                                 //在同目录下找clis.rs文件
mod commands;                            //在同目录下找commands.rs文件
mod utils;
mod models;

fn main(){
    color_backtrace::install();  //安装彩色回溯，方便调试(来自cargo.toml中的color-backtrace依赖)
    cli::handle_commands();  //调用cli模块中的handle_commands函数，处理命令行输入
}