// using rayon
use rayon::prelude::*;
use std::time;

trait Shape {
    fn get_factor(&self) -> f64;
}

struct Sphere {}

impl Shape for Sphere {
    fn get_factor(&self) -> f64 {
        0.77
    }
}

struct Object {
    prop: f64,
    shape: Box<dyn Shape + Sync>,
}

impl Object {
    fn new(prop: f64, shape: Box<dyn Shape + Sync>) -> Self {
        Object { prop, shape }
    }
}
struct World {
    objects: Vec<Object>,
}

impl World {
    fn new(objects: Vec<Object>) -> Self {
        World { objects }
    }
}

struct Camera {
    px: f64,
    py: f64,
}

impl Camera {
    fn new(px: f64, py: f64) -> Self {
        Camera { px, py }
    }
    fn render(&self, world: &World) -> Vec<f64> {
        let mut big_buffer = Vec::new();
        for n in 0..5
        {
            let mut buffer = vec![0.0; 20];
            buffer
                .iter_mut()
                .enumerate()
                .for_each(|(i, p)| *p += pixel_at(n*20+i, self, world));
            big_buffer.append(&mut buffer);
            println!("create slice # {}",n);
        }

        big_buffer
    }
}

fn pixel_at(i: usize, camera: &Camera, world: &World) -> f64 {
    //  println!("calculating {}",i);
    let p: f64 = (i as f64) + camera.px + camera.py;
    let mut t = 0.0;
    for _ in 0..500000 {
        t += world.objects[0].prop / p.sqrt() - world.objects[1].shape.get_factor();
    }
    t
}

fn main() {
    /*
    let mut buffer = vec![0.0;100];
    let now = time::Instant::now();
    let k = 4.0;
    buffer.par_iter_mut().enumerate().for_each(
        |(i,p)|*p+=k+pixel_at(i)
    );
    println!("{:?}",buffer);
    let duration = now.elapsed();
    println!("elapsed: {} ms", duration.as_millis());
    */

    let objs = vec![
        Object::new(3.0, Box::new(Sphere {})),
        Object::new(2.0, Box::new(Sphere {})),
    ];
    let w = World::new(objs);
    let c = Camera::new(4.2, 8.4);
    let now = time::Instant::now();
    let result = c.render(&w);
    let duration = now.elapsed();
    println!("elapsed: {} ms", duration.as_millis());

    // println!("{:?}",result);
}

/*
use std::sync::mpsc;
use std::thread;
use std::time;

struct Result {
    value: f64,
    index: usize,
}

fn pixel_at(i: usize) -> f64 {
    println!("calculating {}",i);
    let p: f64 = (i as f64) + 0.5;
    let mut t = 0.0;
    for _ in 0..5000000 {
        t += 1.0 / p.sqrt();
    }
    t
}

fn main(){

    let (tx,rx) = mpsc::channel();
    let tx2 = tx.clone();
    let tx3 = tx.clone();
    let now = time::Instant::now();
    thread::spawn(move || {
        for i in 0..4 {
            let msg = Result {value:pixel_at(i), index:i};
            tx.send(msg).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 4..7 {
            let msg = Result {value:pixel_at(i), index:i};
            tx2.send(msg).unwrap();
        }
    });
    thread::spawn(move || {
        for i in 4..7 {
            let msg = Result {value:pixel_at(i), index:i};
            tx3.send(msg).unwrap();
        }
    });

    for received in rx {
        println!("{} = {}",received.index, received.value);
    }
    let duration = now.elapsed();
    println!("elapsed: {} ms", duration.as_millis());

}



------------------------------------------------------------------------------------------------------
use std::sync::{Arc, Mutex};
use std::thread;
use std::time;

fn pixel_at(i: usize) -> f64 {
    println!("calculating {}",i);
    let p: f64 = (i as f64) + 0.5;
    let mut t = 0.0;
    for _ in 0..5000000 {
        t += 1.0 / p.sqrt();
    }
    t
}

fn main() {

    let mut result = Arc::new(Mutex::new(vec![0.0; 10]));
    let now = time::Instant::now();
    let mut i: usize = 0;
    loop {
        let c1 = Arc::clone(&result);
        let t1 = thread::spawn(move || {
            let r = pixel_at(i);
            let mut tmp = c1.lock().unwrap();
            tmp[i]=r;
        });
        i += 1;
        if i == 10 {
            break;
        }
        let c2 = Arc::clone(&result);
        let t2 = thread::spawn(move || {
            let r = pixel_at(i);
            let mut tmp = c2.lock().unwrap();
            tmp[i]=r;
        });
        i += 1;
        if i == 10 {
            break;
        }
        let c3 = Arc::clone(&result);
        let t3 = thread::spawn(move || {
            let r = pixel_at(i);
            let mut tmp = c3.lock().unwrap();
            tmp[i]=r;
        });
        t1.join().unwrap();
        t2.join().unwrap();
        t3.join().unwrap();
        i += 1;
        if i == 10 {
            break;
        }
    }
    let duration = now.elapsed();
    println!("elapsed: {} ms", duration.as_millis());
    println!("result: {:?}", result);
}
*/
