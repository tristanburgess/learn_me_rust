use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    // could take in more generic type parameters
    T: Fn(u32) -> u32,
{
    calculation: T,
    // could be a hashmap of domain values mapped to memoized function values
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, rand: u32) {
    let mut expensive_res = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_res.value(intensity));
        println!("Next, do {} situps!", expensive_res.value(intensity));
    } else {
        if rand == 3 {
            println!("Take a break today! Remmebr to stay hydrated!");
        } else {
            println!("Today, run for {} minutes!", expensive_res.value(intensity));
        }
    }
}

fn main() {
    let sim_usr_spec_value = 10;
    let sim_rand = 7;

    generate_workout(sim_usr_spec_value, sim_rand);

    let sim_usr_spec_value = 27;
    let sim_rand = 3;

    generate_workout(sim_usr_spec_value, sim_rand);

    let sim_usr_spec_value = 27;
    let sim_rand = 2;

    generate_workout(sim_usr_spec_value, sim_rand);

    // We can capture the larger environment in closures:
    let x = 4;
    let eq_to_x = |z| z == x;
    let y = 4;
    assert!(eq_to_x(y));
}
