use futures::executor::block_on;

#[derive(Debug)]
struct Song {

}

async fn learn_song() -> Song {
    println!("learn song");
    Song{}
}

async fn sing_song(song: Song) {
    println!("Sing song, {:?}",song);
}

async fn dance() {
    println!("dance");
}

async fn lear_and_sing() {
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = lear_and_sing();
    let f2 = dance();

    futures::join!(f1,f2); // await 批量的Future
}

fn main() {
    block_on(async_main())

    // let song = block_on(learn_song());
    // block_on(sing_song(song));
    // block_on(dance());
}