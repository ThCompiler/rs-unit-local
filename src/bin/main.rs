use rs_unit::rs_unit;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

rs_unit! {
    describe "Addition" {
        test "success: Add positive numbers" {
            let result = add(1,1);
            assert_eq!(result, 2);
        }

       test "success: Add negative numbers" {
            let result = add(-2, -2);
            assert_eq!(result, -4);
        }
    }
}

fn main() {}