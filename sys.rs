//作者缩进太垃圾了没办法写了点注释 特别不规范缩进警告
use std::process::Command;
use std::env;
//use std::io::Read;
use std::str;
//ruat编译器太他妈比牛逼了 直接修好代码 rust就是牛
//此程序是个屑勉强能跑，欢迎给建议
//参考文章 https://www.cnblogs.com/a5idc/p/13752839.html
//此程序由何某在termux上编写
fn run(runc:&str , s:&str , stype:&str) -> (){
    //我等了114514年才等来这个能用的运行其他程序且打印输出结果，文心一言写的[>       // 莫名其妙的换行我不会修pwpwpwpwpwpwpwpwppwp    反正能用))))
       let output = Command::new(runc)
        .arg(s) //添加命令行参数
        .arg(stype)
        .output() // 执行命令并获取输出                                                  .expect("Failed to execute command");

    // 将命令的标准输出转换为字符串                                                  let stdout = String::from_utf8_lossy(&output.stdout);
    let stdout_str = str::from_utf8(&output.stdout).unwrap_or_else(|e| {             panic!("{e}")
    });
    // 打印标准输出

    println!("\n{}", stdout_str);


    let stderr = String::from_utf8_lossy(&output.stderr);

     println!("\n{}", stderr);
}
fn readarg() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    args

}
fn main () {
    let args = readarg();
    let long = args.len();

    //let stype = &args[1];
    //let s = &args[2];

    //println!("debug len {len}");
    let stype = if long > 1{
       &args[1]
    }else{
      panic!("啥!啥!啥! 这给的啥");
    };
    let s = if long > 2 {
       &args[2]
    }else{
      panic!("啥!啥!啥! 这给的啥");
    };
    if stype == "-h"{
       println!("这是一个假的systemctl ，主要用来给使用proot容器的人使用，前提是 可以使 用service,如果运行 开关服务时没有用可能因为没有 chkconfig ，推荐安装 sysv-rc-conf 可以在root下运行  \n apt-get install sysv-rc-conf;cp /usr/sbin/sysv-rc-conf /usr/sbin/chkconfig ，来替代chkconfig 试过debian 找不到 chkconfig ");
       std::process::exit(0)
    }
    if long <= 2{
       println!("输入不正确退出");
       std::process::exit(2)
    }
    //let stype = &args[1];
    //let s = &args[2];
    //panic!("{s} {stype}");
    if stype == "start" || stype == "stop" || stype == "restart" || stype == "status"{
       //我等了114514年才等来这个能用的运行其他程序且打印输出结果，文心一言写的，别指望 大改这个里
       run("service" , s , stype)

   } else if stype == "enable"{
            run("chkconfig" , s , "on")
   } else if stype == "disable"{
   } else {
      println!("超纲了! 毕竟作者做不到 “还是rust大佬” 的水平");
  }
      }
