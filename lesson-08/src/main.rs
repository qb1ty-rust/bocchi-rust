#![warn(clippy::all, clippy::pedantic)]

use std::error::Error;

// fn sum<T: std::marker::Copy + std::ops::Add<Output = T>>(numbers: &[T]) -> Result<T, Box<dyn Error>> {
//     Ok(numbers.iter().copied().reduce(|acc, n| acc + n).ok_or("error")?)
// }

fn sum<T>(numbers: &[T]) -> Result<T, Box<dyn Error>>
where
    T: std::marker::Copy + std::ops::Add<Output = T>,
{
    Ok(numbers
        .iter()
        .copied()
        .reduce(|acc, n| acc + n)
        .ok_or("error")?)
}

#[derive(Debug)]
struct Employee<A, T> {
    age: A,
    salary: T,
    tax: T,
}

impl<A, T: std::marker::Copy + std::ops::Sub<Output = T> + std::ops::Mul<Output = T>> Employee<A, T> {
    fn salary_with_tax(&self) -> T {
        self.salary - (self.salary * self.tax)
    }
}

trait Drive {
    fn can_drive(&self) -> bool;
}

struct Car {
    gas: u8,
}

impl Drive for Car {
    fn can_drive(&self) -> bool {
        self.gas > 0
    }
}

struct ElectroCar {
    battery_charge: i8,
}

impl Drive for ElectroCar {
    fn can_drive(&self) -> bool {
        self.battery_charge > 0
    }
}

fn car_info<T: Drive>(car: &T) {
    println!("Can drive? {}", car.can_drive());
}

fn main() -> Result<(), Box<dyn Error>> {
    let v: Vec<i32> = vec![100, 200, 140, 150];
    let c: Vec<i8> = vec![1, 5, 10, -5];

    let r = sum(&v)?;
    let f = sum(&c)?;

    println!("R: {r:?}, F: {f:?}");

    let car = Car { gas: 100 };
    let is_drive = car.can_drive();

    let electro_car = ElectroCar { battery_charge: 0 };
    let is_drive_2 = electro_car.can_drive();

    car_info(&car);
    car_info(&electro_car);

    println!("{is_drive}, {is_drive_2}");

    let employee = Employee {
        age: 19,
        salary: 1_000_000.0,
        tax: 0.12,
    };

    println!(
        "Age: {}, Salary: {}, Tax: {}",
        employee.age, employee.salary, employee.tax
    );
    println!("Employee: {employee:?}");
    println!("Final salary: {}", employee.salary_with_tax());

    Ok(())
}
