use std::pin::{Pin, pin};
use std::thread;
use std::time::Duration;
use trpl::StreamExt;
use trpl::{Either, Html};

fn main() {
    // demo3();
    main_demo();
}

#[allow(unused)]
#[allow(dead_code)]
fn main_demo() {
    trpl::run(async {
        demo11().await;
    });
}

#[allow(unused)]
#[allow(dead_code)]
async fn demo11() {
    let values = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let iter = values.iter().map(|n| n * 2);
    let mut stream = trpl::stream_from_iter(iter);

    let mut filtered = stream.filter(|value| value % 3 == 0 || value % 5 == 0);

    while let Some(value) = filtered.next().await {
        println!("The value was: {value}");
    }
}

#[allow(unused)]
#[allow(dead_code)]
async fn demo10() {
    let slow = async {
        trpl::sleep(Duration::from_secs(5)).await;
        "Finally finished"
    };

    match timeout(slow, Duration::from_secs(2)).await {
        Ok(message) => println!("Succeeded with '{message}'"),
        Err(duration) => {
            println!("Failed after {} seconds", duration.as_secs())
        }
    }
}

#[allow(unused)]
#[allow(dead_code)]
async fn timeout<F: Future>(future_to_try: F, max_time: Duration) -> Result<F::Output, Duration> {
    match trpl::race(future_to_try, trpl::sleep(max_time)).await {
        Either::Left(output) => Ok(output),
        Either::Right(_) => Err(max_time),
    }
}

#[allow(unused)]
#[allow(dead_code)]
async fn demo6() {
    let a = async {
        println!("'a' started.");
        slow("a", 30);
        trpl::yield_now().await;
        slow("a", 10);
        trpl::yield_now().await;
        slow("a", 20);
        trpl::yield_now().await;
        println!("'a' finished.");
    };

    let b = async {
        println!("'b' started.");
        slow("b", 75);
        trpl::yield_now().await;
        slow("b", 10);
        trpl::yield_now().await;
        slow("b", 15);
        trpl::yield_now().await;
        slow("b", 350);
        trpl::yield_now().await;
        println!("'b' finished.");
    };

    trpl::race(a, b).await;
}

fn slow(name: &str, ms: u64) {
    thread::sleep(Duration::from_millis(ms));
    println!("'{name}' ran for {ms}ms");
}

#[allow(unused)]
#[allow(dead_code)]
async fn demo5() {
    let a = async { 1u32 };
    let b = async { "Hello!" };
    let c = async { true };

    let (a_result, b_result, c_result) = trpl::join!(a, b, c);
    println!("{a_result}, {b_result}, {c_result}");
}

#[allow(unused)]
#[allow(dead_code)]
fn demo3() {
    trpl::run(async {
        let (tx, mut rx) = trpl::channel();

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("future"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                trpl::sleep(Duration::from_millis(500)).await;
            }
        });

        let rx_fut = pin!(async {
            while let Some(value) = rx.recv().await {
                println!("received '{value}'");
            }
        });

        let tx_fut = pin!(async move {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                trpl::sleep(Duration::from_millis(1500)).await;
            }
        });

        // trpl::join3(tx1_fut, tx_fut, rx_fut).await;
        // let futures =
        //     vec![Box::new(tx1_fut), Box::new(rx_fut), Box::new(tx_fut)];

        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];
        trpl::join_all(futures).await;
    });
}

#[allow(unused)]
#[allow(dead_code)]
fn demo2() {
    trpl::run(async {
        let fut1 = async {
            for i in 1..10 {
                println!("hi number {i} from the first task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        let fut2 = async {
            for i in 1..5 {
                println!("hi number {i} from the second task!");
                trpl::sleep(Duration::from_millis(500)).await;
            }
        };

        trpl::join(fut1, fut2).await;
    });
}

#[allow(unused)]
#[allow(dead_code)]
fn demo1() {
    // let args: Vec<String> = std::env::args().collect();

    trpl::run(async {
        let title_fut_1 = page_title("https://news.163.com");
        let title_fut_2 = page_title("https://news.qq.com");

        let (url, maybe_title) = match trpl::race(title_fut_1, title_fut_2).await {
            Either::Left(left) => left,
            Either::Right(right) => right,
        };

        println!("{url} returned first");
        match maybe_title {
            Some(title) => println!("Its page title is: '{title}'"),
            None => println!("Its title could not be parsed."),
        }
    })
}

async fn page_title(url: &str) -> (&str, Option<String>) {
    let text = trpl::get(url).await.text().await;

    let title = Html::parse(&text)
        .select_first("title")
        .map(|title| title.inner_html());
    (url, title)
}
