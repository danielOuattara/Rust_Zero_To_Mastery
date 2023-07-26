//-------------------------------------------------------
/* Basic
--------- */

// struct Temperature {
//     degrees_f: f64,
// }

// fn show_temperature(temperature: Temperature) {
//     println!("{:?} degrees F ", temperature.degrees_f);
// }

// fn main() {
//     let hot = Temperature { degrees_f: 110.0 };
//     show_temperature(hot);
// }
//
//
/*--------------------------------------------------------
Starting Implementation : make show_temperature() part of
Struct Temperature
---------------------------- */

// struct Temperature {
//     degrees_f: f64,
// }

// impl Temperature {
//     fn show_temperature(temperature: Temperature) {
//         println!("{:?} degrees F ", temperature.degrees_f);
//     }
// }

// fn main() {
//     let hot = Temperature { degrees_f: 110.0 };
//     Temperature::show_temperature(hot);
// }

//--------------------------------------------------------
/* Completing Implementation version 1 : taking a reference to self
-----------------------------------------------------------------*/

// struct Temperature {
//     degrees_f: f64,
// }

// impl Temperature {
//     fn show_temperature(&self) {
//         println!("{:?} degrees F ", self.degrees_f);
//     }
// }

// fn main() {
//     let hot = Temperature { degrees_f: 110.0 };
//     hot.show_temperature(); // New !
//     Temperature::show_temperature(&hot); // still OK
// }

//---------------------------------------------------------
/* Completing Implementation version 2
--------------------------------------- */

struct Temperature {
    degrees_f: f32,
}

impl Temperature {
    fn freezing() -> Self {
        Self { degrees_f: 32.0 }
    }

    fn boiling() -> Self {
        return Self { degrees_f: 212.3 };
    }

    fn show_temperature(&self) {
        println!("{:?} degrees F ", self.degrees_f);
    }
}

fn main() {
    let hot = Temperature { degrees_f: 110.0 };
    hot.show_temperature();

    let cold = Temperature::freezing();
    cold.show_temperature();
    Temperature::show_temperature(&cold); // Still OK !

    let boiling = Temperature::boiling();
    boiling.show_temperature();
    Temperature::show_temperature(&boiling); // Still OK !
}
