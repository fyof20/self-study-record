//src/cli.rs
use clap::{Parser, Subcommand, ArgGroup};
use super::commands as cmd;

#[derive(Parser)] //必须作用在结构体上
#[command(author, version, about, long_about = None)] //自动从Cargo.toml中获取作者、版本和简介信息
struct Cli {
    #[clap(subcommand)]   //让clap知道这个字段是一个子命令
    command: Command,
}

//实现enum Command，包含git的常用命令
#[derive(Subcommand)]
enum Comand {
    ///初始化仓库
    //mit Init
    Init,
    //mit Add -A, mit Add -u, mit Add file1 file2
    Add {
        ///要添加的文件
        #[clap(required = true)]
        files: Vec<String>,   //参数1：文件列表

        ///将工作区中所有的文件改动提交至暂存区（包括新增、修改和删除）
        #[clap(short = 'A', long)]
        all: bool,        //参数2： -A或--all

        ///将工作区中已跟踪的文件(tracked)更新到暂存区（修改 & 删除）；不包含新增
        #[clap(short, long)]  //short自动取首字母
        update: bool,    //参数3： -u或--update
    }
    //删除文件
    Rm{
        ///要删除的文件
        #[clap(required = true)]
        files: Vec<String>,   //参数1：文件列表
        #[clap(long, action)] //flag 删除暂存区的文件
        cached: bool,        //参数2： --cached
        #[clap(short, long)]  //flag 递归删除目录
        recursive: bool,     //参数3： -r或--recursive
    }
}
