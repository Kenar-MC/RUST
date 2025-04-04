fn main() {

    let name = String::from("Alice");

    println!("Hello,{name}");

    let mut player_score = 51;      //i32
    player_score += 1;              
    
    println!("Player score is {player_score}");

    let _delta_time = 1.25; // f64
    let _delta_time:f32 = 1.25; // f32
    let _delta_time = 1.25f32; // f32
    let _delta_time = 1.25_f64; // f64
    let delta_time = 1.25 as f32; // f32

    println!("Current time is {delta_time}");

    let total_points:u8 = 1+2+5; // u8
    println!("Total points are {total_points}");

    let color_in_hex = 0xFF0011; // i32
    println!("Color in hex is {color_in_hex:X}");

    let dir_permission = 0o744; // i32
    println!("Directory permission is {dir_permission:o}");

    let gate_flag:u8 = 0b0001_0100; // u8
    println!("Gate flag is {gate_flag:b} / {gate_flag:o}");

    let is_active = true;

    let first_char = 'a';

     let config = (12,123,String::from("Hello"),true);
     println!("Config is {config:?}");      // Config is (12, 123, "Hello", true)

    let first = config.0;
    let second = config.1;
    let third = config.2;

    let (w,h) = (first,second);
    println!("Width is {w} and Height is {h}"); // Width is 12 and Height is 123

    let mut score: [u8:5] = [213, 123, 23, 43, 54];
    println!("Score is {score:?}"); // Score is [213, 123, 23, 43, 54]
    scores[1]+=50;

    println!("Score is {score:?}"); // Score is [213, 173, 23, 43, 54]


    
}

