fn main() {
    //⭐️if - else if - else
    //simplest code
    let team_size = 7;

    if team_size < 5 {
        println!("small");
    } else if team_size < 10 {
        println!("medium");
    } else {
        println!("large");
    }

    //partially refactored code
    let team_sizes = 11;
    let team_size_in_text;

    if team_sizes < 5 {
        team_size_in_text = "small";
    } else if team_sizes < 10 {
        team_size_in_text = "medium";
    } else {
        team_size_in_text = "large";
    }
    println!("Current team size: {}", team_size_in_text);

    let _is_below_eighteen = if team_size < 18 {true} else {false};
    //Return data type should be the same on each block when using this as an expression.


    //⭐️match
    let tshirt_width = 20;
    let tshirt_size = match tshirt_width {
        16 => "S", //check 16
        17 | 18 => "M", //check 17 and 18
        19 ... 21 => "L", //check from 19 to 21 (19, 20, 21)
        22 => "XL", 
        _ => "Not Available",
    };
    println!("{}", tshirt_size); //L

    let is_allowed = false;
    let list_type = match is_allowed {
        true => "Full",
        false => "Restricted"
        // no default/ _ condition can be skipped
        // Because data type of is_allowed is boolean and all possibilities checked on conditions
    };
    println!("{}", list_type);

    let marks_paper_a: u8 = 25;
    let marks_paper_b: u8 = 30;
    let output = match (marks_paper_a, marks_paper_b) {
        (50, 50) => "Full marks for both papers",
        (50, _) => "Full marks for paper A",
        (_, 50) => "Full marks for paper B",
        (x, y) if x > 25 && y > 25 => "Good",
        (_, _) => "Work hard"
    };
    println!("{}", output); //Work hard

    //⭐️while
    let mut a = 1;
    
    while a <= 10 {
        println!("Current value : {}", a);
        a += 1;
    }
    //usage of break and continue
    let mut b = 0;
    while b < 5 {
        if b == 0 {
            println!("Skip value : {}", b);
            b += 1;
            continue;
        } else if b == 2 {
            println!("Break at : {}", b);
            break;
        }
        println!("Current value : {}", b);
        b += 1;
    }
    //outer break
    let mut c1 = 1;
    'outer_while: while c1 < 6 {//set label outer_while
        let mut c2 = 1;
        'inner_while: while c2 < 6 {
            println!("Current value : [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 { break 'outer_while; } //kill outer_while
            c2 += 1;
        }
        c1 += 1;
    }
    //⭐️loop
    loop {
        println!("Loop forever!");
    }
    //Usage of break and continue
    let mut z = 0;
    loop {
        if z == 0 {
            println!("Skip value : {}", z);
            z += 1;
            continue;
        } else if z == 2 {
            println!("Break at : {}", z);
            break;
        }
        println!("Current value : {}", z);
        z += 1;
    }
    //outer break
    let mut b1 = 1;
    'outer_loop: loop {//set label outer_loop
        let mut b2 = 1;
        'inner_loop: loop {
            println!("Current value : [{}][{}]", b1, b2);
            if b1 == b2 && b2 == 2 {
                break 'outer_loop;
            } else if b2 == 5 {
                break;
            }
            b2 += 1;
        }
        b1 += 1;
    }

    //⭐️for
    for a in 0..10 {//(a=0; a<10; a++) //0 to 10 (exclusive)
        println!("Current value : {}", a);
    }
    //usage of break and continue
    for b in 0..6 {
        if b == 0 {
            println!("Skip value : {}", b);
            continue;
        } else if b == 2 {
            println!("Break at : {}", b);
            break;
        }
        println!("Current value : {}", b);
    }
    //outer break
    'outer_for: for c1 in 1..6 { //set label outer_for
        'inner_for: for c2 in 1..6 {
            println!("Current value : [{}][{}]", c1, c2);
            if c1 == 2 && c2 == 2 {
                break 'outer_for; //kill outer_for
            }
        }
    }
    //working with arrays/vectors
    let group : [&str; 4] = ["Dennis", "Sang", "Kipkemei", "Chumba"];

    for n in 0..group.len() {//group.len() = 4 -> 0..4 👎 check group.len()on each iteration
        println!("Current person : {}", group[n]);
    }

    for person in group.iter() {
        println!("Current person : {}", person);
    }
}