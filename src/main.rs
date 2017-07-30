#![allow(dead_code)]
extern crate image;
extern crate imageproc;
use imageproc::drawing;
use image::Rgb;
use imageproc::pixelops::interpolate;
use std::path::Path;
use std::string::String;
use std::collections::HashMap;

mod turtle;
mod lindenmayer;
use turtle::Turtle;
use turtle::Vec2d;

pub struct ImagePainter
{
    canvas : image::RgbImage,
    color : Rgb<u8>,
}

impl ImagePainter
{
    fn new(width: u32, height: u32, init_color : Rgb<u8>) -> ImagePainter
    {
        ImagePainter{canvas : image::RgbImage::new(width,height), color : init_color}
    }

}

impl turtle::PaintDevice for ImagePainter
{
    fn draw_line(&mut self, start : &Vec2d, end : &Vec2d)
    {
        let start_tuple = (start.x as i32, start.y as i32);
        let end_tuple   = (end.x as i32, end.y as i32);
        drawing::draw_antialiased_line_segment_mut(&mut self.canvas, start_tuple,end_tuple,self.color,interpolate);
    }

    fn set_color(&mut self, r:u8, g:u8, b:u8)
    {
        self.color = Rgb([r,g,b]);
    }

    fn save(&self, filename : String)
    {
        let ref mut fout = Path::new(&filename);
        let _ = self.canvas.save(fout).unwrap();
    }

    fn show(&self, width:u32, height:u32)
    {
        /*let mut window : PistonWindow = WindowSettings::new("piston: image", [width,height])
            .exit_on_esc(true)
            .opengl(OpenGL::V3_2)
            .build()
            .unwrap();
        //window.set_lazy(true);
        while let Some(e) = window.next()
        {
            window.draw_2d(&e, |c,g| {
                clear([0f32;4],g);
                
            });
        }*/
    }

}

fn draw<F>(start_symbol : &'static str, production_rules : HashMap<&'static str, Vec<(f32,&'static str)>>, angle_degrees : f32, scaleFn : F)
    where F : Fn(u32) -> u32
{
    let system = lindenmayer::LSystem::new("S",production_rules);

    for (n,item) in system.take(10).enumerate()
    {
        let mut painter = ImagePainter::new(1200u32,1200u32,Rgb([255u8,255u8,255u8]));
        let mut t = Turtle::new(Vec2d{x:600f32, y: 1200f32}, Vec2d{x:0f32,y:-1f32}, &mut painter);
        t.pen_down();
        let mut states = Vec::new();
        for action in item.chars()
        {
            match action
            {
                'l' => t.turn(angle_degrees).no_return(),
                'r' => t.turn(-angle_degrees).no_return(),
                'F' => t.forward(scaleFn(n as u32) as i32).no_return(),
                'X' => t.no_return(),
                'B' => t.forward(-(scaleFn(n as u32) as i32)).no_return(),
                '[' => states.push( (t.pos(), t.fd_vector()) ),
                ']' => {let (pos,fd_vector) = states.pop().unwrap(); t.set_pos(pos); t.set_fd_vector(fd_vector);},
                x => panic!("No action for value {} specified", x),
            }
        }

        t.save(format!("{}{}.png","test", n));

    }
}

fn main() 
{
    //X → F[−X][X]F[−X]+FX), (F → FF
    let mut descr = HashMap::new();
    descr.insert("S", vec![(1f32,"X")]);
    descr.insert("F", vec![(1f32,"FF")]);
    descr.insert("X", vec![(0.6f32,"F[rX][X]F[rX]lFX"), (0.4f32, "FrX")]);
    
    draw("S", descr,25f32, |n| f32::round((700f32 / f32::powf(2f32,n as f32))) as u32);

}
