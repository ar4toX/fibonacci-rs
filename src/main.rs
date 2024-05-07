fn main() {
    //Working Variables
    let mut x: i128=1;
    let mut y: i128=0;
    let mut counter: i32=1;

    loop {
        //Generate and print fibonacci 
        //numbers while counting them
        println!("{}th: {}", counter, y);
        counter+=1;
        x+=y;
        println!("{}th: {}", counter, x);
        counter+=1;
        y+=x;
    }
}
