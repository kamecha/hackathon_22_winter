use super::*;

pub struct ObjectManager {
    pub objects: Vec<ObjectType>,
}

impl ObjectManager {
    pub fn new() -> ObjectManager {
        ObjectManager { objects: vec![] }
    }
    pub fn add(&mut self, object: ObjectType) {
        self.objects.push(object);
    }
    pub fn generate_object_from_csv(&mut self, path: &str, cell_size: f64) {
        let mut rdr = csv::Reader::from_path(path).unwrap();
        let mut counter_x = 0;
        let mut counter_y = 0;
        let header = rdr.headers().unwrap();
        // println!("{:?}", headers);
        for cell in header.iter() {
            match cell {
                "0" => {}
                "1" => {
                    println!("ブロックを追加したよ");
                    println!("x: {}, y: {}", counter_x, counter_y);
                    self.add(ObjectType::Square(model::Square {
                        vertices: vec![
                            model::Vertex {
                                x: counter_x as f64 * cell_size,
                                y: counter_y as f64 * cell_size,
                            },
                            model::Vertex {
                                x: counter_x as f64 * cell_size + cell_size,
                                y: counter_y as f64 * cell_size,
                            },
                            model::Vertex {
                                x: counter_x as f64 * cell_size + cell_size,
                                y: counter_y as f64 * cell_size + cell_size,
                            },
                            model::Vertex {
                                x: counter_x as f64 * cell_size,
                                y: counter_y as f64 * cell_size + cell_size,
                            },
                        ],
                        edges: vec![
                            model::Edge { start: 0, end: 1 },
                            model::Edge { start: 1, end: 2 },
                            model::Edge { start: 2, end: 3 },
                            model::Edge { start: 3, end: 0 },
                        ],
                    }));
                }
                _ => {}
            }
            counter_x += 1;
        }
        for result in rdr.records() {
            counter_x = 0;
            counter_y += 1;
            let record = result.unwrap();
            for cell in record.iter() {
                // print!("{:?}", cell);
                match cell {
                    "0" => {}
                    "1" => {
                        println!("ブロックを追加したよ");
                        println!("x: {}, y: {}", counter_x, counter_y);
                        self.add(ObjectType::Square(model::Square {
                            vertices: vec![
                                model::Vertex {
                                    x: counter_x as f64 * cell_size,
                                    y: counter_y as f64 * cell_size,
                                },
                                model::Vertex {
                                    x: counter_x as f64 * cell_size + cell_size,
                                    y: counter_y as f64 * cell_size,
                                },
                                model::Vertex {
                                    x: counter_x as f64 * cell_size + cell_size,
                                    y: counter_y as f64 * cell_size + cell_size,
                                },
                                model::Vertex {
                                    x: counter_x as f64 * cell_size,
                                    y: counter_y as f64 * cell_size + cell_size,
                                },
                            ],
                            edges: vec![
                                model::Edge { start: 0, end: 1 },
                                model::Edge { start: 1, end: 2 },
                                model::Edge { start: 2, end: 3 },
                                model::Edge { start: 3, end: 0 },
                            ],
                        }));
                    }
                    "2" => {
                        // rod
                        println!("ロッドを追加したよ");
                        println!("x: {}, y: {}", counter_x, counter_y);
                        self.add(ObjectType::Rod(model::Rod {
                            vertices: vec![
                                Vertex {
                                    x: counter_x as f64 * cell_size,
                                    y: counter_y as f64 * cell_size,
                                },
                                Vertex {
                                    x: counter_x as f64 * cell_size + cell_size,
                                    y: counter_y as f64 * cell_size + cell_size,
                                },
                            ],
                            edges: vec![Edge { start: 0, end: 1 }],
                            speed: 30.0,
                        }));
                    }
                    _ => {}
                }
                counter_x += 1;
            }
            // println!("");
        }
    }
}
