// C-type standard struct
struct Song{
    title: String,
    album: String,
    artist: String,
    time_sec: i32
}

// Tuple Struct
struct Point(i32, i32, i32);

fn main(){
    let song1 = Song{
        title: String::from("30/90"),
        artist: String::from("Tick..tick..boom!"),
        album: String::from("Andrew Garfield"),
        time_sec: 3600
    };

    println!("
             {} was made by {} for the album {} and it is {} seonds long.
             ",
             song1.title,
             song1.album,
             song1.artist,
             song1.time_sec
             );


    let origin = Point(0,0,0);
    println!("Point contains {:?} and {:?}", origin.0, origin.1);

    let Point(x, y, z) = origin;
    println!("Point is {:?}, {:?}", x, y);


}
