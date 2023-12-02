use std::fs::read_to_string;

fn main() {
    let mut doc: String = read_to_string("first-input").unwrap();
    // println!("{:?}", doc);

    let mut line_number : u32 = 0;
    let mut total: u64 = 0;

    let words : [&str; 9] = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine"
    ];

    let replace_words : [&str; 9] = [
        "o1e",
        "t2o",
        "t3e",
        "4",
        "5e",
        "6",
        "7n",
        "e8t",
        "n9e"
    ];

    let mut i = 1;
    for word in words {
        doc = doc.replace(word, replace_words[i-1]);
        println!("{} replaced by {}", word, i);
        i += 1;
    }

    for line in doc.lines() {
        let mut nums = String::new();

        for char in line.chars() {
            if let Some(_) = char.to_digit(10) {
                nums.push(char);
            }
        }
       
        let line_num : String = nums.get(0..1).unwrap().to_owned() + nums.get(nums.len()-1..nums.len()).unwrap();


        total += line_num.parse::<u64>().unwrap();

        println!("line numbeer {}, text {}, nums {:?}, line_num {}, total {}", line_number, line, nums, line_num, total);

        line_number += 1;
    }
}
