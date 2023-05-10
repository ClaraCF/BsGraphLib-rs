pub struct BsCmdGraph {
    width: i32,
    height: i32,
    texture: i32,   // The character that will be used for the pixel

    display: [[i32; 100]; 100],
}


// Implement private methods for Bs_cmd_graph
impl BsCmdGraph {
    /* Check that the given coordinates are
     * within the boundaries of the display
     * Returns true if on boundaries, false otherwise
    */
    fn check_boundaries(&self, x: i32, y: i32) -> bool {
        // Check that the X coordinate is within boundaries
        // Checking it doesn't go under 0 or that it
        // doesn't go over the limit
        //if x < 0 || x > 100 {
        //    return false;
        //}
        
        // Check that the Y coordinate is within boundaries
        // Checking it doesn't go under 0 or that it
        // doesn't go over the limit
        //if y < 0 || y > 100 {
        //    return false;
        //}

        //return true;
        
        // Check that both X and Y don't go below 0
        // or go above the limit of the display
        return x > 0 && x < 100 && y > 0 && y < 100;
    }
}


// Implement public methods for Bs_cmd_graph
impl BsCmdGraph {
    // Constructor taking in the window size and textur
    pub fn new(width: i32, height: i32, texture: i32) -> BsCmdGraph {
        return BsCmdGraph {
            width: width,
            height: height,
            texture: texture,
            display: [[0; 100]; 100],   // Initialize empty display
        };
    }
    
    /* Draw a single character to the screen at (x, y) coordinates
     * The character will only be drawn if the
     * coordinates are within the bounds of the window
     *
     * Returns true if the character was successfully drawn, false otherwise
    */
    pub fn put_pixel(&mut self, x: i32, y: i32, character: i32) -> bool {
        // Check that the coordinates are within the boundaries
        if !self.check_boundaries(x, y) {
            return false;
        }

        // Draw the character to the screen
        self.display[x as usize][y as usize] = character;
        return true;
    }
    
    /* Draw a line to the screen between two points.
     *
     * This is the old line tracing algorithm.
     * It's been deprecated in favor of the Bresenham algorithm.
     * This one is faster, but not as precise, so use at your own risk.
     * Using it requires your code to be marked as unsafe.
     * 
     * The line will only be drawn if the coordinates of both points
     * are within the boundaries of the window
     * Returns true if the line is successfully drawn, false otherwise
    */
    pub unsafe fn draw_line_old(&mut self) -> bool {
        todo!("Implement this deprecated function");
    } 

    /* Draw a line to the screen between two points
     * The line will only be drawn if the coordinates of both points
     * are within the boundaries of the window
     *
     * Returns true if the line is successfully drawn, false otherwise
    */
    pub fn draw_line(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: i32)  -> bool {
        let incyi;
        let incyr;
        let incxi;
        let incxr;

        // Calcultate deltas between the points
        let mut dx = x2 - x1;
        let mut dy = y2 - y1;
        
        if dy >= 0 {
            incyi = 1;
        } else {
            dy = dy * -1;
            incyi = -1;
        }
        
        if dx >= 0 {
            incxi = 1;
        } else {
            dx = dx * -1;
            incxi = -1;
        }
        
        if dx >= dy {
            incyr = 0;
            incxr = incxi;
        } else {
            incxr = 0;
            incyr = incyi;

            // Invert dx and dy
            dx ^= dy;
            dy ^= dx;
            dx ^= dy;
        }

        let mut x = x1;
        let mut y = y1;
        let avr = 2 * dy;
        let mut av = avr - dx;
        let avi = av - dx;
        
        while x != x2 || y != y2 {
            self.put_pixel(x, y, color);
            
            if av >= 0 {
                x += incxi;
                y += incyi;
                av += avi;
            } else {
                x += incxr;
                y += incyr;
                av += avr;
            }
        }
        
        return true;
    }
    
    /* Draw a polygon to the screen
     * The polygon is represented as a vertex vector
     *
     * Returns true if the polygon is fully drawn successfully, false otherwise
    */
    pub fn draw_poly(&mut self, vertex_array: Vec<[i32;2]>, color: i32) -> bool {
        let mut return_value = true;
        
        for vertex in vertex_array.windows(2) {
            return_value = return_value &&
            self.draw_line(
                vertex[0][0],
                vertex[0][1],
                vertex[1][0],
                vertex[1][1],
                color
            );
        }
        
        return return_value;
    }
    
    // Draw an image to the screen
    // Only parts of the image that are within boundaries will be drawn
    pub fn draw_img(&mut self, image: [[i32;16];16], x: i32, y: i32) {
        for (i, vertical) in image.iter().enumerate() {
            for (j, horizontal) in vertical.iter().enumerate() {
                if *horizontal != 0 {
                    if self.check_boundaries(x, y) {
                        self.display[j + x as usize][i + y as usize] = *horizontal;
                    }
                }
            }
        }
    }
    
    // Draw text to the screen
    // Only parts of the text that are within boundaries will be drawn
    pub fn draw_text(&mut self) {
        
    }
    
    // Draw the final display buffer to the console
    pub fn cmd_draw(&mut self) {
        
    }
    
    // Clear the console screen
    pub fn cmd_clear(&mut self) {

    }
}
