
pub trait Operation {
    fn get_left_component(&self, result: u64, right_component: u64) -> Option<u64>;
}

pub struct Addition;
impl Operation for Addition {
    fn get_left_component(&self, result: u64, right_component: u64) -> Option<u64> {
        result.checked_sub(right_component)
    }
}

pub struct Multiplication;
impl Operation for Multiplication {
    fn get_left_component(&self, result: u64, right_component: u64) -> Option<u64> {
        if result % right_component == 0 {
            Some(result / right_component)
        } else {
            None
        }
    }
}

pub struct Concatenation;
impl Operation for Concatenation {
    fn get_left_component(&self, result: u64, right_component: u64) -> Option<u64> {
        let (result_string, right_string) = (result.to_string(), right_component.to_string());
        if result_string == right_string {
            Some(0)
        } else if result_string.ends_with(&right_string) {
            let until = result_string.len() - right_string.len();
            Some(result_string.get(0..until).unwrap().parse().unwrap())
        } else {
            None
        }
    }
}