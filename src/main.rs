#[derive(Debug,Clone)]
struct Car {
    id:u32,
    name:String,
    publish_year:u32,
    hp:f32,
    average_rate:f32
}
#[derive(Debug)]

struct Car_database {
    cars:Vec<Car>
}

impl Car {
    fn new (id:u32,name:String,publish_year:u32,hp:f32,average_rate:f32) ->Self {
        Car {
            id,
            name,
            publish_year,
            hp,
            average_rate
        }

    }
    
}

impl Car_database {
    fn new() -> Self {
        Car_database { cars: vec![] }
    }

    fn add_car(&mut self, car: Car) {
        self.cars.push(car); 
    }
    fn get_car_by_id(&self,id:u32)->Option<&Car> {
        self.cars.iter().find(|car|car.id==id)

        
    }
}




fn main() {
    let mut car_database = Car_database::new();

    let  car1 = Car::new(
        1, 
        String::from("Renault Megane Sedan"), 
        2023, 
        115.00, 
        4.3
    );

    let car2 = Car::new(
        2, 
        String::from("Fiat Egea"), 
        2024, 
        95.00, 
        4.4
    );

    let car3 = Car::new(
        3, 
        String::from("Volkswagen Golf"), 
        2023, 
        150.00, 
        4.6
    );

    let car4 = Car::new(
        4, 
        String::from("Toyota Corolla"), 
        2023, 
        122.00, 
        4.5
    );

    let car5 = Car::new(
        5, 
        String::from("Ford Focus"), 
        2023, 
        150.00, 
        4.4
    );

    let car6 = Car::new(
        6, 
        String::from("Hyundai i20"), 
        2023, 
        100.00, 
        4.2
    );

    let car7 = Car::new(
        7, 
        String::from("Honda Civic"), 
        2024, 
        180.00, 
        4.7
    );

    let car8 = Car::new(
        8, 
        String::from("Peugeot 3008"), 
        2023, 
        130.00, 
        4.5
    );

    let car9 = Car::new(
        9, 
        String::from("Opel Astra"), 
        2023, 
        145.00, 
        4.4
    );

    let car10 = Car::new(
        10, 
        String::from("BMW 3 Series"), 
        2023, 
        184.00, 
        4.8
    );

    let car11 = Car::new(
        11, 
        String::from("Mercedes-Benz A-Class"), 
        2024, 
        163.00, 
        4.7
    );

    let car12 = Car::new(
        12, 
        String::from("Skoda Octavia"), 
        2023, 
        150.00, 
        4.5
    );

    let car13 = Car::new(
        13, 
        String::from("Dacia Dokker"), 
        2023, 
        95.00, 
        4.1
    );

    let car14 = Car::new(
        14, 
        String::from("Toyota Yaris"), 
        2023, 
        115.00, 
        4.6
    );

    let car15 = Car::new(
        15, 
        String::from("Mercedes-Benz C-Class"), 
        2024, 
        204.00, 
        4.8
    );

    car_database.add_car(car1);
    car_database.add_car(car2);
    car_database.add_car(car3);
    car_database.add_car(car4);
    car_database.add_car(car5);
    car_database.add_car(car6);
    car_database.add_car(car7);
    car_database.add_car(car8);
    car_database.add_car(car9);
    car_database.add_car(car10);
    car_database.add_car(car11);
    car_database.add_car(car12);
    car_database.add_car(car13);
    car_database.add_car(car14);
    car_database.add_car(car15);

    let car_id = 1 ;

    if let Some(car)=car_database.get_car_by_id(car_id){
        println!("ID: {} | Name: {} | Year: {} | HP: {} | Average Rating: {}", 
        car.id, car.name, car.publish_year, car.hp, car.average_rate);

    }
    else {
        println!("Couldn't find the car")
    }







    
}