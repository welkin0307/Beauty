mod vec;
mod ray;
mod translate;
mod rotate;
mod hit;
mod sphere;
mod rect;
mod cube;
mod camera;
mod mat;
mod aabb;
mod bvh;
mod perlin;
mod texture;
mod medium;                                                                                                        

use std::{io::{stderr, Write}};
use rand::Rng;
use rayon::prelude::*;
use vec::{Vec3, Point3, Color};
use ray::Ray;
use translate::Translate;
use rotate::{Axis, Rotate};
use hit::{Hittable, HittableList};
use sphere::{Sphere, MovingSphere};
use rect::{Plane, AARect};
use cube::Cube;
use camera::Camera;
use mat::{Lambertian, Metal, Dielectric, DiffuseLight};
use bvh::BVH;
use texture::{ConstantTexture, CheckTexture, NoiseTexture, ImageTexture};
use medium::ConstantMedium;

fn ray_color(ray: &Ray, color: Color, world: &Box<dyn Hittable>, depth: u64) -> Color {
    if depth <= 0 {
        // if we've exceeded the ray bounce limit, no more light is gathered
        return Color::new(0.0, 0.0, 0.0)
    }

    // 0.001 t_min fixs shadow acne
    if let Some(rec) = world.hit(ray, 0.00001, f64::INFINITY) {
        // 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0))

        // Lambertian:
        // let target = rec.position + rec.normal + Vec3::random_in_unit_sphere().normalized();
        
        // Hemispherical scattering:
        // let target = rec.position + Vec3::random_in_hemisphere(rec.normal);

        // let r = Ray::new(rec.position, target - rec.position);
        // 0.5 * ray_color(&r, world, depth - 1)

        let emitted: Color = rec.material.emitted(rec.u, rec.v, &rec.position);

        if let Some((attenuation, scattered)) = rec.material.scatter(ray, &rec) {
            emitted + attenuation * ray_color(&scattered, color, world, depth - 1)
        } else {
            emitted
        }

    } else {
    // let unit_direction = ray.direction().normalized();
    // let t = 0.5 * (unit_direction.y() + 1.0);

    // //lerp white and blue with direction of y
    // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);
    color
    }
}

fn random_scene() -> Box<dyn Hittable> {
    let mut rng = rand::thread_rng();
    let mut world: Vec<Box<dyn Hittable>> = Vec::new();

    let ground_mat = Lambertian::new(CheckTexture::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0)), ConstantTexture::new(Color::new(0.3, 0.3, 1.0))));
    let ground_sphere = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, ground_mat);

    world.push(Box::new(ground_sphere));

    for a in -11..=11 {
        for b in -11..=11 {
            let choose_mat: f64 = rng.gen();
            let center = Point3::new((a as f64) + rng.gen_range(0.0..0.9),
                                     0.2,
                                     (b as f64) + rng.gen_range(0.0..0.9));

            if choose_mat < 0.8 {
                // Diffuse
                let albedo = Color::random(0.0..1.0) * Color::random(0.0..1.0);
                let sphere_mat = Lambertian::new(ConstantTexture::new(albedo));
                let center1 = center + Vec3::new(0.0, rng.gen_range(0.0..0.01), 0.0);
                let sphere = MovingSphere::new(center, center1, 0.0, 1.0, 0.2 ,sphere_mat);

                world.push(Box::new(sphere));
            } else if choose_mat < 0.95 {
                // Metal
                let albedo = Color::random(0.4..1.0);
                let fuzz = rng.gen_range(0.0..0.5);
                let sphere_mat = Metal::new(albedo, fuzz);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            } else {
                // Glass
                let sphere_mat = Dielectric::new(1.5);
                let sphere = Sphere::new(center, 0.2, sphere_mat);

                world.push(Box::new(sphere));
            }
        }
    }

    let mat1 = Dielectric::new(1.5);
    let mat2 = Lambertian::new(ConstantTexture::new(Color::new(0.4, 0.2, 0.1)));
    let mat3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);

    let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, mat1);
    let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, mat2);
    let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, mat3);

    world.push(Box::new(sphere1));
    world.push(Box::new(sphere2));
    world.push(Box::new(sphere3));

    Box::new(BVH::new( world, 0.0, 1.0))
}

fn two_spehre() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let top_mat = Lambertian::new(CheckTexture::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0)), ConstantTexture::new(Color::new(0.3, 0.3, 1.0))));
    let bottom_mat = Lambertian::new(CheckTexture::new(ConstantTexture::new(Color::new(1.0, 1.0, 1.0)), ConstantTexture::new(Color::new(0.3, 0.3, 1.0))));

    let top_sphere = Sphere::new(Point3::new(0.0, 10.0, 0.0), 10.0, top_mat);
    let bottom_sphere = Sphere::new(Point3::new(0.0, -10.0, 0.0), 10.0, bottom_mat);

    world.push(top_sphere);
    world.push(bottom_sphere);

    Box::new(world)
}

fn two_perlin_sphere() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let top_mat = Lambertian::new(NoiseTexture::new(2.0));
    let bottom_mat = Lambertian::new(NoiseTexture::new(2.0));

    //hash goes wrong in negative field, move object to Fitst Quadrant for now
    let top_sphere = Sphere::new(Point3::new(1000.0, 2.0, 1000.0), 2.0, top_mat);
    let bottom_sphere = Sphere::new(Point3::new(1000.0, -1000.0, 1000.0), 1000.0, bottom_mat);

    world.push(top_sphere);
    world.push(bottom_sphere);

    Box::new(world)
}

fn earth() -> Box<dyn Hittable> {
    let image = image::open("earthmap.jpg").expect("image not found").to_rgb8();
    let (width ,height) = image.dimensions();
    let data = image.into_raw();
    let texture = ImageTexture::new(data, width, height);
    let earth = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Lambertian::new(texture));
    Box::new(earth)
}

fn light_room() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let bottom_mat = Lambertian::new(ConstantTexture::new(Color::new(0.7, 0.7, 0.7)));
    let top_mat = Lambertian::new(ConstantTexture::new(Color::new(0.0, 0.1843, 0.6549)));
    let emitted = DiffuseLight::new(ConstantTexture::new(Color::new(4.0, 4.0, 4.0)));
    
    let ground = Sphere::new(Point3::new(0.0, -1000.0, 0.0), 1000.0, bottom_mat);
    let sphere = Sphere::new(Point3::new(0.0, 2.0, 0.0), 2.0, top_mat);
    let plane = AARect::new(Plane::XY, 3.0, 5.0, 1.0, 3.0, -2.0, emitted);

    world.push(ground);
    world.push(sphere);
    world.push(plane);

    Box::new(world)
}

fn cornell_box() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let red = Lambertian::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(ConstantTexture::new(Color::new(25.0, 25.0, 25.0)));

    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(AARect::new(Plane::XZ, 213.0, 343.0, 227.0, 332.0, 554.0, light));
    world.push(AARect::new(Plane::XZ, 0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(AARect::new(Plane::XZ, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    world.push(AARect::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    world.push(
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone()),-18.0), Vec3::new(130.0, 0.0, 65.0)));
    world.push(
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white),15.0), Vec3::new(265.0, 0.0, 295.0)));

    Box::new(world)
}

fn cornell_box_with_smoke() -> Box<dyn Hittable> {
    let mut world = HittableList::default();

    let red = Lambertian::new(ConstantTexture::new(Color::new(0.65, 0.05, 0.05)));
    let white = Lambertian::new(ConstantTexture::new(Color::new(0.73, 0.73, 0.73)));
    let green = Lambertian::new(ConstantTexture::new(Color::new(0.12, 0.45, 0.15)));
    let light = DiffuseLight::new(ConstantTexture::new(Color::new(25.0, 25.0, 25.0)));

    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 555.0, green));
    world.push(AARect::new(Plane::YZ, 0.0, 555.0, 0.0, 555.0, 0.0, red));
    world.push(AARect::new(Plane::XZ, 213.0, 343.0, 227.0, 332.0, 554.0, light));
    world.push(AARect::new(Plane::XZ, 0.0, 555.0, 0.0, 555.0, 0.0, white.clone()));
    world.push(AARect::new(Plane::XZ, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));
    world.push(AARect::new(Plane::XY, 0.0, 555.0, 0.0, 555.0, 555.0, white.clone()));

    let box1 = 
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 165.0, 165.0), white.clone()),-18.0), Vec3::new(130.0, 0.0, 65.0));
    let box2 =
        Translate::new(
            Rotate::new(Axis::Y,
                        Cube::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(165.0, 330.0, 165.0), white),15.0), Vec3::new(265.0, 0.0, 295.0));

    world.push(ConstantMedium::new(box1, 0.01, ConstantTexture::new(Color::new(1.0, 1.0, 1.0))));
    world.push(ConstantMedium::new(box2, 0.01, ConstantTexture::new(Color::new(0.0, 0.0, 0.0))));

    Box::new(world)
}

enum Scene {
    Random,
    TwoSphere,
    TwoPerlinSphere,
    Earth,
    LightRoom,
    CornellBox,
    CornellSmoke
}

fn main() {
    // image
    const ASPECT_RATIO: f64 = 1.0;
    const IMAGE_WIDTH: u64 = 600;
    const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;
    const SAMPLES_PER_PIXEL: u64 = 10000;
    const MAX_DEPTH: u64 = 500;

    // world
    // let mut world = World::new();
    // let mat_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let mat_center = Rc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let mat_left = Rc::new(Dielectric::new(1.5));
    // let mat_left_inner = Rc::new(Dielectric::new(1.5));
    // let mat_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.1));

    // let sphere_ground = Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0, mat_ground);
    // let sphere_center = Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5, mat_center);
    // let sphere_left = Sphere::new(Point3::new(-1.0, 0.0, -1.0), 0.5, mat_left);
    // let sphere_left_inner = Sphere::new(Point3::new(-1.0, 0.0, -1.0), -0.46, mat_left_inner);
    // let sphere_right = Sphere::new(Point3::new(1.0, 0.0, -1.0), 0.5, mat_right);

    // world.push(Box::new(sphere_ground));
    // world.push(Box::new(sphere_center));
    // world.push(Box::new(sphere_left));
    // world.push(Box::new(sphere_left_inner));
    // world.push(Box::new(sphere_right));

    // let world = random_scene();

    // camera
    // let lookfrom = Point3::new(13.0, 2.0, 3.0);
    // let lookat = Point3::new(0.0, 0.0, 0.0);
    // let vup = Vec3::new(0.0, 1.0, 0.0);
    // let dist_to_focus = 10.0;
    // let aperture = 0.1;
    // let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);
    // let viewport_height = 2.0;
    // let viewport_width = viewport_height * ASPECT_RATIO;
    // let focal_length = 1.0;

    // let origin = Point3::new(0.0, 0.0, 0.0);
    // let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    // let vertical = Vec3::new(0.0, viewport_height, 0.0);
    // let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let scene: Scene = Scene::CornellSmoke;
    let (world, background, camera) = match scene {
        Scene::Random => {
            let world = random_scene();

            let backgournd = Color::new(0.7, 0.8, 1.0);

            let lookfrom = Point3::new(13.0, 2.0, 3.0);
            let lookat = Point3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.1;
            let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

            (world, backgournd, camera)
        }
        Scene::TwoSphere =>{
            let world = two_spehre();

            let backgournd = Color::new(0.7, 0.8, 1.0);

            let lookfrom = Point3::new(13.0, 2.0, 3.0);
            let lookat = Point3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

            (world, backgournd, camera)
        }
        Scene::TwoPerlinSphere => {
            let world = two_perlin_sphere();

            let backgournd = Color::new(0.7, 0.8, 1.0);

            let lookfrom = Point3::new(1013.0, 2.0, 1003.0);
            let lookat = Point3::new(1000.0, 0.0, 1000.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

            (world, backgournd, camera)
        }
        Scene::Earth => {
            let world = earth();

            let backgournd = Color::new(0.7, 0.8, 1.0);

            let lookfrom = Point3::new(13.0, 2.0, 3.0);
            let lookat = Point3::new(0.0, 0.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.1;
            let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

            (world, backgournd, camera)
        }
        Scene::LightRoom => {
            let world = light_room();

            let backgournd = Color::new(0.0, 0.0, 0.0);

            let lookfrom = Point3::new(26.0, 3.0, 6.0);
            let lookat = Point3::new(0.0, 2.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.0;
            let camera = Camera::new(lookfrom, lookat, vup, 20.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

            (world, backgournd, camera)
        }
        Scene::CornellBox => {
            let world = cornell_box();
            
            let backgournd = Color::new(0.0, 0.0, 0.0);

            let lookfrom = Point3::new(278.0, 278.0, -800.0);
            let lookat = Point3::new(278.0, 278.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.05;
            let camera = Camera::new(lookfrom, lookat, vup, 40.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

            (world, backgournd, camera)
        },

        Scene::CornellSmoke => {
            let world = cornell_box_with_smoke();
            
            let backgournd = Color::new(0.0, 0.0, 0.0);

            let lookfrom = Point3::new(278.0, 278.0, -800.0);
            let lookat = Point3::new(278.0, 278.0, 0.0);
            let vup = Vec3::new(0.0, 1.0, 0.0);
            let dist_to_focus = 10.0;
            let aperture = 0.05;
            let camera = Camera::new(lookfrom, lookat, vup, 40.0, ASPECT_RATIO, aperture, dist_to_focus, 0.0, 1.0);

            (world, backgournd, camera)
        }
    };

    println!("P3");
    println!("{} {}",IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    //let mut rng = rand::thread_rng();
    for j in (0..IMAGE_HEIGHT).rev() {
        //adding a progress indicator
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {

            // let r = i as f64 / (IMAGE_WIDTH - 1) as f64;
            // let g = j as f64 / (IMAGE_HEIGHT - 1) as f64;
            // let b = 0.25;

            // let ir = (255.999 * r) as u64;
            // let ig = (255.999 * g) as u64;
            // let ib = (255.999 * b) as u64;

            // println!("{} {} {}", ir, ig, ib);
            
            // let pixel_color = Color::new(i as f64 / (IMAGE_WIDTH - 1) as f64,
            //                                    j as f64 / (IMAGE_HEIGHT - 1) as f64,
            //                                    0.25);
            
            // let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            // let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            // let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            // let pixel_color = ray_color(&r, &world);

            // let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            // for _ in 0..SAMPLES_PER_PIXEL {
            //     let random_u = rng.gen::<f64>();
            //     let random_v = rng.gen::<f64>();

            //     let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
            //     let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);
                
            //     let r = camera.get_ray(u, v);
            //     pixel_color += ray_color(&r, &world, MAX_DEPTH);
            // }          

            let pixel_color: Color = (0..SAMPLES_PER_PIXEL).into_par_iter().map(|_sample| {
                
                let mut rng = rand::thread_rng();
                let random_u = rng.gen::<f64>();
                let random_v = rng.gen::<f64>();

                let u = ((i as f64) + random_u) / ((IMAGE_WIDTH - 1) as f64);
                let v = ((j as f64) + random_v) / ((IMAGE_HEIGHT - 1) as f64);

                let r = camera.get_ray(u, v);

                // let unit_direction = r.direction().normalized();
                // let t = 0.5 * (unit_direction.y() + 1.0);
                // //lerp white and blue with direction of y
                // let backgournd = (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0);

                ray_color(&r, background, &world, MAX_DEPTH)
            })
            .sum();
            
            println!("{}", pixel_color.format_color(SAMPLES_PER_PIXEL));
        }
    }
    eprintln!("Done.");
}
