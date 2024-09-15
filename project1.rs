mod math {
    pub struct Answer {
        pub add_sum: i32,
        pub sub_sum: i32,
        pub mult_sum: i32
    }
    pub fn sum_up(num1: i32, num2: i32) -> Answer{
        let result_add: i32 = add(num1, num2);
        let result_sub: i32 = sub(num1, num2);
        let result_mult: i32 = mult(num1, num2);
        
        let sum: Answer = Answer {
        add_sum: result_add,
        sub_sum: result_sub, 
        mult_sum: result_mult
            
        };
        
        return sum;
    }
    fn add(num1: i32, num2: i32) -> i32 {
        num1 + num2
    }
    
    fn sub(num1: i32, num2: i32) -> i32 {
        num1 - num2
    }
    fn mult(num1: i32, num2: i32) -> i32 {
        num1 * num2
    }
}

fn main(){
    let result = math::sum_up(2,2);
    
    println!("Sum_add: {}\nSum_sub: {}\nSum_mult: {}", result.add_sum, result.sub_sum, result.mult_sum);
}
