fn main() {


    let mut scores: Vec<u16>  = vec![1, 2, 3, 4, 5]; // Vec<i32> is a vector of integers
    let mut sum = 0;
    
    println!("The initial scores are: {:#?}", scores);

    scores.push(6); // Add a new score to the vector
    println!("The scores are: {:#?}", scores);

    for score in &scores {
        sum += score;
    }
    println!("The sum of the scores is: {:#?}", sum);

    let last_score = scores.pop().unwrap(); // Remove the last score from the vector
        
    println!("The last score is: {:#?}", last_score);
    println!("The scores without the last score: {:#?}", scores);



    /////////////////////////////////////

    let mut colors = Vec::new();
    colors.push(String::from("red"));
    colors.push(String::from("green"));
    colors.push(String::from("blue"));
    println!("The colors are: {:#?}", colors);

    colors.reverse(); // Reverse the order of the colors
    println!("The colors in reverse order are: {:#?}", colors);

    let codes:Vec<u8> = (50..=100).collect(); // Create a vector of numbers from 50 to 100
    println!("The codes are: {:?}", codes);

    let numbers = (1..=9).collect::<Vec<u8>>(); // Create a vector of numbers from 1 to 9
    let first_two = numbers[0..2].to_vec(); // Get the first two numbers
    println!("The first two numbers are: {:#?}", first_two);
    
    


}
