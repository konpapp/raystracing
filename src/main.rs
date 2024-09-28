use std::fs::File;
use std::io::{Result, Write};

use color::{write_color, Color};
use ray::Ray;
use vec3::{Point3, Vec3};

mod color;
mod ray;
mod vec3;

fn get_ray_color(r: Ray) -> Color {
    let unit_direction = Vec3::unit_vec(r.direction());
    let a = (unit_direction.y() + 1.0) * 0.5;

    let white = Color::new(1.0, 1.0, 1.0);
    let blue = Color::new(0.5, 0.7, 1.0);

    white.mul_f64(1.0 - a).add(blue.mul_f64(a))
}

fn main() -> Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let mut image_height = (image_width as f64 / aspect_ratio) as i32;
    image_height = if image_height < 1 { 1 } else { image_height };

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0); // Should use neg() here?

    let pixel_delta_u = viewport_u.div_f64(image_width as f64);
    let pixel_delta_v = viewport_v.div_f64(image_height as f64);

    let viewport_upper_left = camera_center
        .sub(Vec3::new(0.0, 0.0, focal_length))
        .sub(viewport_u.div_f64(2.0))
        .sub(viewport_v.div_f64(2.0));
    let pixel00_loc = viewport_upper_left.add((pixel_delta_u.add(pixel_delta_v)).mul_f64(0.5));

    // Render
    let mut file = File::create("image.ppm")?;
    writeln!(file, "P3\n{} {}\n255", image_width, image_height)?;

    eprintln!("Scanlines remaining:");
    for j in 0..image_height {
        eprint!("{} ", image_height - j);

        for i in 0..image_width {
            let pixel_center = pixel00_loc
                .add(pixel_delta_u.mul_f64(i as f64))
                .add(pixel_delta_v.mul_f64(j as f64));
            let ray_direction = pixel_center.sub(camera_center);
            let ray = Ray::new(camera_center, ray_direction);

            let pixel_color = get_ray_color(ray);
            write_color(&mut file, pixel_color)?;
        }
    }

    eprintln!("\nDone!");
    Ok(())
}

fn render_hello_world_image() -> Result<()> {
    let mut file = File::create("hw_image.ppm")?;
    writeln!(file, "P3\n{} {}\n255", 256, 256)?;

    eprintln!("Scanlines remaining:");
    for j in 0..256 {
        eprint!("{} ", 256 - j);

        for i in 0..256 {
            let pixel_color = Color::new(
                i as f64 / (256 - 1) as f64,
                j as f64 / (256 - 1) as f64,
                0.0,
            );

            write_color(&mut file, pixel_color)?;
        }
    }
    eprintln!("\nDone!");
    Ok(())
}
