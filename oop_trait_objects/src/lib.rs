pub struct AveragedCollection {
    list: Vec<i32>,
    avg: f64,
}

impl AveragedCollection {
    pub fn add(&mut self, val: i32) {
        self.list.push(val);
        self.update_avg();
    }

    pub fn remove(&mut self) -> Option<i32> {
        let res = self.list.pop();
        match res {
            Some(val) => {
                self.update_avg();
                Some(val)
            }
            None => None,
        }
    }

    pub fn average(&self) -> f64 {
        self.avg
    }

    fn update_avg(&mut self) {
        let total: i32 = self.list.iter().sum();
        self.avg = total as f64 / self.list.len() as f64;
    }
}

pub trait Draw {
    fn draw(&self);
}

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {}
}
