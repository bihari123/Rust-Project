fn main() {
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("the value of x in the inner scope is {x}")
    }

    println!("the value of x is: {x}");

    let mut number = if x < 6 { 7 } else { 6 };

    let result=loop{
        number+=1;

        if number ==10{
            break number*2;
        }
    };

    println!("The result is {result}");



    'counting_up:loop{ // labeling the loop
        let mut remaining =10;

        loop{
            if remaining ==9{
                break;
            }

            if number==2{
                break 'counting_up;
            }
 
            remaining-=1;
        }

    }

    for number in (1..=4).rev(){
        println!("{number}!")
    }

}
