const PI: f64 = std::f64::consts::PI;

struct Form {
    name: String,
    width: f64,
    height: f64,
}

struct Circle {
    name: String,
    radius: f64,
}

// trait =~ interface equivalent's
trait AreaCalc {
    fn area(&self) -> f64;
    fn get_name(&self) -> &String;
    fn this_is_associate_function() -> bool;
}

impl AreaCalc for Form {
    fn area(&self) -> f64 {
        // methods implements.area()
        self.width * self.height
    }

    // função associada, AreaCalc::this_is_associate_function()
    fn this_is_associate_function() -> bool {
        return true;
    }

    fn get_name(&self) -> &String {
        &self.name
    }
}

impl AreaCalc for Circle {
    fn area(&self) -> f64 {
        (self.radius * self.radius) * PI
    }

    fn this_is_associate_function() -> bool {
        return true;
    }
    fn get_name(&self) -> &String {
        &self.name
    }
}

fn show_area_strategy<T: AreaCalc>(form: T) {
    println!("{} => {}", form.get_name(), form.area());
}

fn main() {
    let form = Form {
        width: 20f64,
        height: 30f64,
        name: "Rectangle".to_string(),
    };

    let circle = Circle {
        radius: 2f64,
        name: "Circle".to_string(),
    };

    Form::this_is_associate_function();

    show_area_strategy(form);
    show_area_strategy(circle);
}
