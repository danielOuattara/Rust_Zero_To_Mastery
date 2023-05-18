// String demo

struct LineItem {
    name: String,
    count: i32,
}

fn show_name(data: &str) {
    println!("{:?}", data)
}

fn main() {
    let receipt = vec![
        LineItem {
            name: "cereal".to_owned(),
            count: 2,
        },
        LineItem {
            name: String::from("strawberry"),
            count: 3,
        },
    ];

    for item in &receipt {
        println!("{:?}: quantity {:?}", item.name, item.count);
    }

    for item in &receipt {
        show_name(&item.name);
    }
}
