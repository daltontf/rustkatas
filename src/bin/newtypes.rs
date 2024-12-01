struct Years(i64);

struct Days(i64);

impl Years {
    pub fn to_days(&self) -> Days {
        Days(self.0 * 365)
    }
}


impl Days {
    /// truncates partial years
    pub fn to_years(&self) -> Years {
        Years(self.0 / 365)
    }
}

fn is_adult(age: &Years) -> bool {
    age.0 >= 18
}

fn is_adult_i64(age_in_unknown: &i64) -> bool {
    *age_in_unknown > 18i64 
}

fn main() {
    let age = Years(25);
    let age_days = age.to_days();
    println!("Is an adult? {}", is_adult(&age));
    println!("Is an adult? {}", is_adult(&age_days.to_years()));

    println!("Is an is_adult_i64 based on age? {}", is_adult_i64(&age.0));
    println!("Is an is_adult_i64 based on age_days? {}", is_adult_i64(&age_days.0));
    // println!("Is an adult? {}", is_adult(&age_days));
}
