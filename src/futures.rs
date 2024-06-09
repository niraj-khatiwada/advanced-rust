use tokio::task::JoinSet;

#[tokio::main]
async fn main() {
    let (one, two) = tokio::join!(async_one(), async_two()); // -> It will run everything regardless of any one throws error. This is like Promise.allSettled()

    println!("join {:?} {:?}", one, two);

    match tokio::try_join!(async_one(), async_two()) {
        // -> It's like Promise.all(). If all of the promises are resolved, then it'll return Ok(), otherwise it'll return early Err() which was encountered.
        Ok((one, two)) => {
            println!("try-join {:?} {:?}", one, two);
        }
        Err(err) => {
            println!("Error encountered {:?}", err)
        }
    }

    let futures = vec![foo(1), foo(2)];
    println!("{:?}", futures::future::join_all(futures).await);

    tokio::spawn(async {
        let futures = vec![foo(1), foo(2)];
        println!("{:?}", futures::future::join_all(futures).await);
    })
    .await
    .unwrap();

    // Dynamic futures (It's not possible to use in a vector, so we need to use this)
    let mut futures_of_different_types_tokio = JoinSet::new();

    futures_of_different_types_tokio.spawn(async_one());
    futures_of_different_types_tokio.spawn(async_two());

    // Unfortunately, this happens sequentially
    while let Some(val) = futures_of_different_types_tokio.join_next().await {
        println!("{:?}", val);
    }
}

async fn async_one() -> Result<String, String> {
    Ok(String::from("One"))
}

async fn async_two() -> Result<String, String> {
    Err(String::from("Two"))
}

async fn foo(i: u32) -> u32 {
    i
}
