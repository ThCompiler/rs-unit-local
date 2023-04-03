use run_tests::suite::Suite;
use run_tests_macro::run_tests;

pub struct TestSuite {
    input: String
}

impl Default for TestSuite {
    fn default() -> Self {
        Self {
            input: String::default()
        }
    }
}

impl TestSuite {
    pub fn base_test(&self) {
        assert_eq!(self.input, "1".to_string());
    }
}

impl Suite for TestSuite {
    fn before_test(&mut self) {
        self.input = "1".to_string();
    }

    fn after_test(&mut self) {
        self.input.clear();
    }
}

run_tests! {
    suite TestSuite {
        base_test
    }
}

fn main() {}