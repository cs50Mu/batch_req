use std::collections::HashMap;
use anyhow::Result;
use reqwest::Client;
use futures::{Stream, StreamExt};
use tokio::fs::File;
use tokio::io::BufReader;
// An extension trait which adds utility methods to AsyncBufRead types.
use tokio::io::AsyncBufReadExt;
use tokio_stream::wrappers::LinesStream;
use std::env;

// 要做的事情
/*
1. http 请求某个服务
2. 从文件读入批量参数
3. 并发
*/

/*
payload={'uid': '10023819',
'activity_alias': '0db2140170f5',
'uuid': 'xxxxxxqwer'}
*/


// 参考: https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest
// http://patshaughnessy.net/2020/1/20/downloading-100000-files-using-async-rust
// https://gendignoux.com/blog/2021/04/01/rust-async-streams-futures-part1.html  很有帮助,学到很多
#[tokio::main]
async fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    // 数据输入文件
    let file_path = &args[1];
    // 并发数
    let concurrency = args[2].parse::<usize>()?;

    let client = reqwest::Client::new();

    let input = File::open(file_path).await?;

    let bodies = gen_uid(input, concurrency).
        map(|uid| {
            // println!("{}", uid);
            // reqwest's Client is already reference-counted
            // clone 只会增加引用计数, 不会复制对象
            send_coupon(client.clone(), uid)
       }).
        // buffer_unordered(4);
        // 它的 receiver 需要是 a stream of Future
        buffered(concurrency);

    bodies.enumerate()
        // 这里也能设置并发
        // This is similar to StreamExt::for_each, but the futures produced by the closure are run concurrently
        .for_each_concurrent(10, |(idx, b)| async move {
            if idx % 10 == 0 {
                match b {
                    Ok(b) => println!("[idx: {}] Got resp: {}", idx, b),
                    Err(e) => eprintln!("[idx: {}] Got an error: {}", idx, e),
                }
            }
        })
        .await;

    Ok(())
}

// 坑: 这个函数不能声明成 async 的,否则会编译报错
fn gen_uid(file: File, concurrency: usize) -> impl Stream<Item = String> {
    LinesStream::new(BufReader::new(file).lines()).
        filter_map(|line| async { line.ok() }).
        map(|line| async move { line.trim().to_string() }).
        // 实际测试来看, 这里设置多少都没啥效果
        // 实际的并发数是由下面的那个buffered来控制的
        // 原因未知, 可能是因为文件读写相对较快?
        buffered(concurrency)
}

async fn send_coupon(client: Client, uid: String) -> Result<String> {
    let mut params = HashMap::from([
        ("uid", "10023819"),
        ("activity_alias", "ba897b89a85d"),
        ("uuid", "xxxxxxqwer")
    ]);
    params.insert("uid", &uid);
    let url = "http://coupon.int.sao.cn/coupon/send";
    let res = client.post(url).
        header("X-Shadow", "true").
        form(&params).
        send().
        await?;

    // println!("{:?}", res.text().await?);
    // println!("requesting: {} xxx", uid);
    Ok(res.text().await?)
    // Ok(())
}