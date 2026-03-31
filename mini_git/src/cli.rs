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
    },
    //删除文件
    Rm{
        ///要删除的文件
        #[clap(required = true)]
        files: Vec<String>,   //参数1：文件列表
        #[clap(long, action)] //flag 删除暂存区的文件
        cached: bool,        //参数2： --cached
        #[clap(short, long)]  //flag 递归删除目录
        recursive: bool,     //参数3： -r或--recursive
    },
    //提交暂存区的文件
    Commit {
        #[clap(short,long)]
        messages: String,
        #[clap(long, action)]
        allow_empty: bool,
    },
    ///查看当前状态
    Status,
    ///log 现实提交历史
    #[clap(group = ArgGroup::new("sub").required(false))]//让clap知道这个子命令有两个互斥的参数
    Log {
        #[clap(short='A), long]
        all: bool,

        #[clap(short, long)]
        number : Option<usize>,
    },
    ///branch
    Branch {
        ///分支名称
        #[clap(group="sub")]
        new_branch: Option<String>,
        ///基于某个commit创建新分支
        #[clap(requried="new_branch")]
        commit_hash: Option<String>,
        
        ///列出所有分支
        #[clap(short,long,action,group="sub",default_value_t=true)]//不传此参数时，默认为 true   
        list: bool,

        ///删除制定分支，不能删除当前所在分支
        #[clap(short='D', long, group="sub")]
        delete: Option<String>,

        ///显示当前分支
        #[clap(long, action, group="sub")]
        show_current: bool,
    },

    ///切换分支
    Switch {
        ///要切换的分支名称
        #[clap(requried_unless_present="create")]//该字段是必需的，除非存在 "create" 参数
        branch_name: Option<String>,

        ///创建并切换到新分支
        #[clap(short, long, group="sub")]
        create: Option<String>,

        ///是否允许切换到commit(分离head状态)
        #[clap(short,long,action,default_value_t=false,group="sub")]
        detach: bool,
    },
    ///restore
    Restore {
        ///要恢复的文件
        #[clap(required = true)]
        path: Vec<String>,

        ///sorce
        #[clap(long, short)]
        source: Option<String>,

        ///worktree
        #[clap(action, long, short = 'W')]
        worktree: bool,

        ///staged
        #[clap(action, long, short = 'S')]
        staged: bool,
    },
    ///Merge
    Merge {
        ///要合并的分支
        #[clap(required = true)]
        branch: String,
    },
}
pub fn handle_command() {
    let cli = Cli::parse(); //解析命令行参数，生成一个Cli结构体实例
    match cli.command {
        Command::Init => cmd::init()。expect("初始化失败"),
        Command::Add { files, all, update } => cmd::add(files, all, update),
        Command::Add { files, all, update } => cmd::add(files, all, update),
        Command::Rm { files, cached, recursive } => cmd::rm(files, cached, recursive).expect("删除失败"),
        Command::Commit { messages, allow_empty } => cmd::commit(messages, allow_empty),
        Command::Status => cmd::status(),
        Command::Log { all, number } => cmd::log(all, number),
        Command::Branch { new_branch, commit_hash, list, delete, show_current } => {
            cmd::branch(new_branch, commit_hash, list, delete, show_current)
        },
        Command::Switch { branch_name, create, detach } => {
            cmd::switch(branch_name, create, detach)
        },
        Command::Restore { path, source, mut worktree, staged } => {
            // 未指定stage和worktree时，默认操作worktree
            // 指定 --staged 将仅还原index
            if !staged {
                worktree = true;
            }
            // 未指定source 且 --staged，默认操作HEAD，否则从index中恢复（就近原则）
            /*
                If `--source` not specified, the contents are restored from `HEAD` if `--staged` is given,
                otherwise from the [index].
            */
            cmd::restore(path, source, worktree, staged);
        }
        Command::Merge { branch } => {
            cmd::merge(branch)
        },
    }
}
