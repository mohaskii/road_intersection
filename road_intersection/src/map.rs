use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::pixels::Color;

pub struct Map {
    top_left: Rect,
    top_right: Rect,
    bottom_left: Rect,
    bottom_right: Rect,
}

impl Map {
    pub fn new(width: u32, height: u32) -> Self {
        let rect_width = width / 3;
        let rect_height = height / 3;

        let top_left = Rect::new(0, 0, rect_width, rect_height);
        let top_right = Rect::new(width as i32 - rect_width as i32, 0, rect_width, rect_height);
        let bottom_left = Rect::new(0, height as i32 - rect_height as i32, rect_width, rect_height);
        let bottom_right = Rect::new(width as i32 - rect_width as i32, height as i32 - rect_height as i32, rect_width, rect_height);

        Map {
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        }
    }

    pub fn draw(&self, canvas: &mut Canvas<Window>, width: u32, height: u32) {
        canvas.set_draw_color(Color::RGB(255, 255, 255));
        canvas.fill_rect(self.top_left).expect("Failed to draw rectangle");
        canvas.fill_rect(self.top_right).expect("Failed to draw rectangle");
        canvas.fill_rect(self.bottom_left).expect("Failed to draw rectangle");
        canvas.fill_rect(self.bottom_right).expect("Failed to draw rectangle");
        
        // Calculate center points of sides
        let center_top = width / 2;
        let center_left = height / 2;

        canvas.draw_line((center_top as i32, 0), (center_top  as i32, height as i32 / 3)).expect("Failed to draw line");
        canvas.draw_line((center_top as i32, height as i32), (center_top  as i32, height as i32 / 3)).expect("Failed to draw line");

        canvas.draw_line((0, center_left as i32), (width as i32 / 3, center_left as i32)).expect("Failed to draw line");        
    }
    

}
