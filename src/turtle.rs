
#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Copy)]
#[derive(Clone)]
pub struct Vec2d 
{
    pub x : f32,
    pub y : f32, 
}

impl Vec2d
{
    pub fn add_assign(&mut self, other : &Vec2d) -> &mut Self
    {
        self.x += other.x;
        self.y += other.y;
        self
    }

    pub fn sub_assign(&mut self, other : &Vec2d) -> &mut Self
    {
        self.x -= other.x;
        self.y -= other.y;
        self
    }

    pub fn normalize(&mut self) -> &mut Self
    {
        let norm = self.length();
        self.x /= norm;
        self.y /= norm;
        self
    }

    pub fn length(&self) -> f32
    {
        f32::sqrt(self.x * self.x + self.y * self.y)
    }

    pub fn add(first : &Vec2d, second : &Vec2d) -> Vec2d
    {
        Vec2d{x : first.x + second.x, y : first.y + second.y}
    }

    pub fn sub(first : &Vec2d, second : &Vec2d) -> Vec2d
    {
        Vec2d{x : first.x - second.x, y : first.y - second.y}
    }

    pub fn scalar_mult(first : &Vec2d, scalar : f32) -> Vec2d
    {
        Vec2d{x : first.x * scalar, y : first.y * scalar}
    }

    pub fn scalar_div(first : &Vec2d, scalar : f32) -> Vec2d
    {
        Vec2d{x : first.x / scalar, y : first.y / scalar}
    }

    pub fn scalar_mult_assign(&mut self, scalar : f32) -> &mut Self
    {
        self.x *= scalar;
        self.y *= scalar;
        self
    }

    pub fn scalar_div_assign(&mut self,  scalar : f32) -> &mut Self
    {
        self.x /= scalar;
        self.y /= scalar;
        self
    }

}

pub trait PaintDevice
{
    fn draw_line(&mut self, start : &Vec2d, end : &Vec2d);
    fn set_color(&mut self, r : u8, g : u8, b : u8);
    fn save(&self, filename : String);
    fn show(&self, width: u32, height : u32);
}

pub struct Turtle<'a, T: PaintDevice + 'a>
{
    pos : Vec2d,
    forward_vector : Vec2d,
    painter : &'a mut T,
    can_draw : bool,
}

impl<'a, T: PaintDevice + 'a> Turtle<'a, T>
{

    pub fn save(&mut self, path : String) -> &mut Self
    {
        self.painter.save(path);
        self
    }

    pub fn new(start_pos: Vec2d, start_forward_vector : Vec2d , paint_device : &'a mut T) -> Turtle<'a,T>
    {
        Turtle{ pos : start_pos, forward_vector : start_forward_vector, painter : paint_device, can_draw : true}
    }

    pub fn no_return(&self)
    {

    }

    pub fn fd_vector(&self) -> Vec2d
    {
        self.forward_vector
    }

    pub fn set_fd_vector(&mut self, fd_vector : Vec2d)
    {
        self.forward_vector = fd_vector;
    }

    pub fn set_pos(&mut self, pos : Vec2d)
    {
        self.pos = pos;
    }

    pub fn pos(&self) -> Vec2d
    {
        self.pos
    }

    pub fn forward(&mut self, pixels : i32) -> &mut Self
    {
        let before = self.pos;
        let dir = Vec2d::scalar_mult(&self.forward_vector, pixels as f32);
        self.pos.add_assign( &dir );
        if self.can_draw {self.painter.draw_line(&before, &self.pos);}  
        self
    }

    pub fn turn(&mut self, degrees : f32) -> &mut Self
    {
        let radians = f32::to_radians(degrees);
        let cosa = f32::cos(radians);
        let sina = f32::sin(radians);
        let mut new_forward_vector = self.forward_vector; 
        new_forward_vector.x = cosa * self.forward_vector.x + sina * self.forward_vector.y;
        new_forward_vector.y = -sina * self.forward_vector.x + cosa * self.forward_vector.y;
        self.forward_vector = new_forward_vector;
        self.forward_vector.normalize();
        self
    }

    pub fn pen_up(&mut self) -> &mut Self
    {
        self.can_draw = false;
        self
    }

    pub fn pen_down(&mut self) -> &mut Self
    {
        self.can_draw = true;
        self
    }
}