pub use sdl2::rect::Rect;
pub use sdl2::render::Canvas;
pub use sdl2::video::Window;
pub use sdl2::pixels::Color;

pub struct Map {
    top_left: Rect,
    top_right: Rect,
    bottom_left: Rect,
    bottom_right: Rect,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let rect_width = width as f64/ 2.2;
        let rect_height = height as f64 / 2.3;

        let top_left = Rect::new(0, 0, rect_width as u32, rect_height as u32);
        let top_right = Rect::new(width as i32 - rect_width as u32 as i32, 0, rect_width as u32, rect_height as u32);
        let bottom_left = Rect::new(0, height as i32 - rect_height as i32, rect_width as u32, rect_height as u32);
        let bottom_right = Rect::new(width as i32 - rect_width as u32 as i32, height as i32 - rect_height as i32, rect_width as u32, rect_height as u32);

        Map {
            top_left,
            top_right,  
            bottom_left,
            bottom_right,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, width: i32, height: i32) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(self.top_left).expect("Failed to draw rectangle");
        canvas.fill_rect(self.top_right).expect("Failed to draw rectangle");
        canvas.fill_rect(self.bottom_left).expect("Failed to draw rectangle");
        canvas.fill_rect(self.bottom_right).expect("Failed to draw rectangle");
        
        // Calculate center points of sides
        let center_top = width / 2;
        let center_left = height / 2;

        canvas.draw_line((center_top, 0), (center_top , (height as f64 / 2.3) as i32)).expect("Failed to draw line");
        canvas.draw_line((center_top, height), (center_top , (height - (height as f64 / 2.3) as i32))).expect("Failed to draw line");

        canvas.draw_line((0, center_left), ((width as f64 / 2.2) as i32, center_left)).expect("Failed to draw line"); 
        canvas.draw_line((width, center_left), (width - (width as f64/ 2.2) as i32, center_left)).expect("Failed to draw line");        
    }
    

}
