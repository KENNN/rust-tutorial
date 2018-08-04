fn main() {
    let number = 7;

    if number < 5 {
    	println!("condition was true");
    } else {
    	println!("condition was false");	
    }

    if number != 0 {
    	println!("number is something other than zero");
    }

    let number = 6;
    if number % 4 == 0 {
    	println!("number is divisible by 4");
    } else if number % 3 == 0 {
		println!("number is divisible by 3");
    } else if number % 2 == 0 {
    	println!("number is divisible by 2");
    } else {
    	println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition {
    	5
    } else {
    	6
    };
    println!("The value of number is {}", number);

	/* never ending loop
    loop {
    	println!("again");  
    }
    */

    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
    	println!("The value is: {}", a[index]);
    	index = index + 1;
    }

    for element in a.iter() {
    	println!("The value is: {}", element);
    }

    for number in (1..4).rev() {
    	println!("{}!", number);
    }
    println!("LIFT OFF!!!");
}
