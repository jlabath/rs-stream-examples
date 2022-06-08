use futures::stream::{self, StreamExt};
use rand::Rng;
use tokio::time::{sleep, Duration, Instant};

async fn adder(i: usize) -> usize {
    sleep(Duration::from_millis(1000)).await;
    i + 1
}

async fn randomly_slow_adder(i: usize) -> usize {
    let mut rng = rand::thread_rng();
    let ms = rng.gen_range(0..1000);
    sleep(Duration::from_millis(ms)).await;
    i + 1
}

#[tokio::main]
async fn main() {
    let buffer_size = 3;
    println!("buffer_unordered concurrent execution stream examples.");
    println!(
        "tweak the buffer size to see impact, current buffer size: {}",
        buffer_size
    );

    let t1 = Instant::now();
    let res = stream::iter(0..=9)
        .map(|x| adder(x))
        .buffer_unordered(buffer_size)
        .collect::<Vec<_>>()
        .await;
    let t2 = Instant::now();
    println!(
        "Result of constant 1 second sleep with buffer size {} took {:?} and was {:?}",
        buffer_size,
        t2.duration_since(t1),
        res
    );

    let t1 = Instant::now();
    let res = stream::iter(0..=9)
        .map(|x| randomly_slow_adder(x))
        .buffer_unordered(buffer_size)
        .collect::<Vec<_>>()
        .await;
    let t2 = Instant::now();
    println!(
        "Result of random from 0 to 1 second sleep with buffer size {} took {:?} and was {:?}",
        buffer_size,
        t2.duration_since(t1),
        res,
    );
}
