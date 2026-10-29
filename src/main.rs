use std::io;
fn main() {
    let product = String::from("Apples");
    let stock = 40;
    let price = 12;

    println!("how many apples do you want to buy?");
    println!("Apples store real not phones\n Product: {product}\n Price: {price} UAH\n Stock: {stock}");


    let mut target = String::new();
    io::stdin()
    .read_line(&mut target)
    .expect("Failed to read line");
    let amount = write(&target);

    if (1..=stock).contains(&amount) {
        let total_price = price * amount;
        let new_stock = stock - amount;
        println!("Here you are");
        println!("Receipt: \n Product: {product}\n Amount {amount}\n Total price: {total_price} UAH \n Remaining {product}: {new_stock} "); }
    else{
         println!("MAN wthat did you mean you need {amount} {product} we dont have this amount of {product} "); }
}
fn write(target: &String)-> i32{
    target.trim().parse().expect("Please type a number!")
}