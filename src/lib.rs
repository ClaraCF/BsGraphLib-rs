use std::ops::Index;

#[cfg(target_os = "windows")]
use windows::{self, Win32::System::Console};

pub struct BsCmdGraph {
    width: i32,
    height: i32,
    texture: i32,   // The character that will be used for the pixel

    display: [[i32; 100]; 100],
    text_pointer: i32,
    strings: [[i32; 100]; 100],
    mask: [[i32; 100]; 100],
    
    #[cfg(target_os = "windows")]
    handle: windows::Win32::Foundation::HANDLE,
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

            display: [[0; 100]; 100],           // Initialize empty display
            text_pointer: 0,                    // Initialize empty text pointer
            //strings: Vec::<String>::new(),      // Initialize empty strings vector
            strings: [[0; 100]; 100],           // Initialize empty strings
            mask: [[0; 100]; 100],           // Initialize empty masks

            // TODO: Better handle this error
            #[cfg(target_os = "windows")]
            handle: unsafe {Console::GetStdHandle(Console::STD_OUTPUT_HANDLE)}.unwrap(),
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
    // The text is drawn in the specified coordinates
    // Only parts of the text that are within boundaries will be drawn
    pub fn draw_text(&mut self, x: i32, y: i32, text: &str) {
        // Generate an ID for the display to know which text to draw
        let spot = self.text_pointer + 257;
        let text = text.to_owned() + "\n";  // Append newline to text
        
        // This places the text where it needs to be drawn
        self.display[x as usize][y as usize] = spot;
        
        let mut i = 0;
        while i < text.len() {
            self.strings[i][self.text_pointer as usize] = text.as_bytes()[i].into();
            i += 1;
        }

        let mut i = 1;
        while i < text.len()-1 {
            self.display[(x+1) as usize][y as usize] = -1;
            i += 1;
        }

        self.text_pointer += 1;
    }

    // Draw the final display buffer to the console
    pub fn cmd_draw(&mut self) {
        let mut ptr;

        for y in 0..self.height-1 {
            for x in 0..self.width-1 {
                match self.display[x as usize][y as usize] {
                    0 => {
                        #[cfg(target_os = "linux")]
                        print!("\x1b[49m");
                        
                        print!(" ");
                    },
                    -1 => continue,
                    _ => {
                        if self.display[x as usize][y as usize] > 256 {
                            ptr = self.display[x as usize][y as usize] - 257;
                            
                            for i in 0..99 {
                                if self.strings[i as usize][ptr as usize] == '\n' as i32 {
                                    break;
                                }
                                // TODO: Handle this error
                                print!("{}", char::from_u32(self.strings[i as usize][ptr as usize] as u32).unwrap());
                            }
                        }

                        else {
                            #[cfg(target_os = "windows")]
                            {
                                Console::SetConsoleTextAttribute(self.handle, self.display[x as usize][y as usize]);
                                print!(tex);
                            }
                            
                            #[cfg(target_os = "linux")]
                            print!("\x1b[{}m", self.texture);
                        }
                        
                        break;
                    },
                }
            }

            println!();
        }   
    }
    
    // Clear the console screen
    pub fn cmd_clear(&mut self) {
        // Clear the screen and place the cursor at position 1
        print!("\x1B[2J\x1B[1;1H");
        //print!("\x1bc");
        //std::process::Command::new("clear").status().unwrap();
        
        // TODO: Find a better fix
        self.text_pointer = 0;
        
        for i in 0..self.height-1 {
            for j in 0..self.width-1 {
                self.put_pixel(j, i, 0);
                self.mask[j as usize][i as usize] = 0;
            }
        }
    }
}


/*
void Bs_cmd_graph::Bs_cmd_clear(){ // we define here the action to clear screen for refresh
	for(int i = 0; i < y; i++){ // to avoid the thing to be duplicated while animating
		for(int j = 0; j < x; j++){ // we loop deleting the old screen buffer
			Bs_put_Pixel(j,i,0); // setting pixels to 0 they will be drawn again on a new
			maskInt[j][i] = 0; // we need to refresh buttons on screen
		}     // screen buffer making the animations possible 
	} // that can be useful to avoid static images and to avoid the thing to be
} // freezed or duplicating pixels on a glitch

// these are the classes required to create forms inside the cmd shell 
class Bs_object{ // parent class made for identifier array
	protected: 
		int id, x , y , color; // variables required for all form classes
		bool isActive = false; // we must know if it's active to act
	public:
      Bs_object(int _x,int _y,int _i,int _c);	// constructor of main library
};

Bs_object::Bs_object(int _x,int _y,int _i,int _c){ // constructor
	x = _x; y = _y; id = _i; color = _c; // define all variables required
}

class Bs_cmd_button : public Bs_object{ // button child of bs_object
	private:  
		int obj_size;
	public:
		Bs_cmd_button(int _x,int _y,int _i,int _c); // button constructor
		void onClick(void func()); // if button clicked execute a function 
		void Bs_draw_button(Bs_cmd_graph display,std::string); // draw button on display
	  	void Bs_add_id();
	  	void onClick(int,int,void func());
};

Bs_cmd_button::Bs_cmd_button(int _x,int _y,int _i,int _c) : Bs_object(_x,_y,_i,_c){} // constructor for cmd button

void Bs_cmd_button::Bs_draw_button(Bs_cmd_graph display,std::string _text){ // draw class
	int size = strlen(_text.c_str()); // test string size to make button size
	#ifdef _WIN32
	display.Bs_draw_line(x-1,y,x+size+1,y,color); // button it's text surrounded by a square ########
	#endif
	#ifdef linux
	display.Bs_draw_line(x,y,x+size+2,y,color); // button it's text surrounded by a square ########
	#endif
	display.Bs_draw_text(x+1,y+1,_text); // draw text in the middle of button                #button# <- example of button
	display.Bs_put_Pixel(x,y+1,color); // put 2 pixels just for detail                       ########
	display.Bs_put_Pixel(x+size+1,y+1,color); // put other pixel
	#ifdef _WIN32
	display.Bs_draw_line(x-1,y+2,x+size+1,y+2,color); // draw below line
	#endif
	#ifdef linux
	display.Bs_draw_line(x,y+2,x+size+2,y+2,color); // button it's text surrounded by a square ########
	#endif
	obj_size = size+2;
}

void Bs_cmd_button::Bs_add_id(){ // add the id for mask map 
	for(int yy = 0; yy < 3; yy++){ // loop for y axis on mask array
		for(int xx = 0; xx < obj_size; xx++){ // loop for x axis on array
			maskInt[x+xx][y+yy] = id; // draw id on mask map
		} // . . . this function is required
	}	 // . . . to add interaction into the map
}  //. . . . . .  if it's not defined you wont be able to use object

void Bs_cmd_button::onClick(int _x,int _y,void func()){ // every class must have onClick function
	int idd = maskInt[_x][_y];                          // NOT DEFINED ON BS_OBJ BECAUSE EACH OBJECT ACTS DIFFERENT
	switch(key){ // we use a switch to test
		case 'e': // e pressed
			if(idd == id){ // test what is the button touching
				func();  // if button touched button then execute this function defined by the user
			}
		break;
		default:break; // if click key wasnt pressed then ignore this object until next mouse touch
	}
}

class Bs_Textbox: public Bs_object{
	std::string text;
	private:
		int size;
	public:
		Bs_Textbox(int,int,int,int,std::string,int);
		void Bs_draw_textbox(Bs_cmd_graph);
		void Bs_add_id();
		void onClick(int,int);	
};

Bs_Textbox::Bs_Textbox(int _x,int _y, int _id, int _s, std::string _txt, int _c) : Bs_object(_x,_y,_id,_c){
	size = _s; text = _txt;
}

void Bs_Textbox::Bs_draw_textbox(Bs_cmd_graph display){
	display.Bs_draw_line(x-1,y,x+size+1,y,color);
	display.Bs_draw_line(x-1,y+2,x+size+1,y+2,color);
	display.Bs_put_Pixel(x,y+1,color);
	display.Bs_put_Pixel(x+size+1,y+1,color);
	if(text == ""){
		display.Bs_draw_text(x+1,y+1," ");
	}else{
		display.Bs_draw_text(x+1,y+1,text);
	}
}

void Bs_Textbox::Bs_add_id(){
	for(int yy = 0; yy < 3 ; yy++){
		for(int xx = 0; xx < x+size; xx++){
			maskInt[x+xx][y+yy] = id;
		}
	}
}

void Bs_Textbox::onClick(int _x,int _y){
	int idd = maskInt[_x][_y];
	switch(key){
		case 'e':
			if(idd == id){
			std::getline(std::cin,text);
			size = strlen(text.c_str());
			}
		break;
		default:break;
	}
}

// this is the mouse to interact with form classes inside the project
class Bs_mouse_pointer{ // define the object class
	private: // color is private since no object can interact with color
		int color; // define color variable
	public: // while the x and y variables are public
		int x,y; // because we need access on mouse position for object interaction detection 
		Bs_mouse_pointer(int,int,int); // this is the contructor asking for starting x and y and color
		void Bs_mouse_controller(); // this moves the pointer through all the window
		void Bs_mouse_draw(Bs_cmd_graph display); // this draws the pointer on the screen
}; // now that we have the constructors the actual functions:

Bs_mouse_pointer::Bs_mouse_pointer(int _x,int _y,int _c){ // constructor for muse pointer
	x = _x; // we define the parameter _x as the mouse x on screen
	y = _y; // we define the parameter _y to be the y for our mouse
	color = _c; // we define the mouse color this is for whatever you want
} // end constructor function

void Bs_mouse_pointer::Bs_mouse_controller(){ // this is the mouse controller
	switch(key){ // test which key was pressed
		case 'i':if((y-1) >= 0){y--;}break; // if W was pressed move pointer up
		case 'k':y++;break; // if S was pressed we move pointer down
		case 'j':if((x-1) >= 0){x--;}break; // if it's inside the window not touching border and A is pressed move left
		case 'l':x++;break; // if D was pressed we move to the right
		default:break; // none of these keys where pressed we ignore this function used for interactions wit E
	} // if keyboard detects E then we act on the object that we are selecting 
} // else if other key was pressed and it's not W,A,S,D,E then just ignore actions

void Bs_mouse_pointer::Bs_mouse_draw(Bs_cmd_graph display){ // we pass the display object that will draw pointer on screen
	display.Bs_put_Pixel(x,y,color); // we pass the parameters x and y to set location of our pointer and color 
} // with this the mouse should be drawn on screen and with this we should be able to select objects to interact with

// this is the test function to add on the mouse
#ifdef _WIN32
void alertMsg(){ // this is a test function that will be showing a message box
	MessageBox(NULL,"button clicked","",MB_OK); // show message that the button was clicked
} // this is just to test the interaction with buttons take it as a default function
#endif

/*
This part comes for the simple 3d engine made for this library (it might improve)
*/

void draw_wall(Bs_cmd_graph display, float px, float py, float pa, float vx1, float vy1, float vx2, float vy2, int col){
float x1 = 0; float y1a = 0; 
float y1b = 0; float x2 = 0;
float y2a = 0; float y2b = 0;
float tx1 = 0; float ty1 = 0;
float tx2 = 0; float ty2 = 0;
float tz1 = 0; float tz2 = 0;
/*[MOVER EL MAPA ALREDEDOR DEL JUGADOR]*/
 	 tx1 = vx1 - px; ty1 = vy1 - py;
 	 tx2 = vx2 - px; ty2 = vy2 - py;
 	
 	 /*[TRASLADAR EL MAPA ALREDEDOR DEL JUGADOR]*/
 	 tz1 = tx1 * cos((3.14159*pa)/180) + ty1 * sin((3.14159*pa)/180);
 	 tz2 = tx2 * cos((3.14159*pa)/180) + ty2 * sin((3.14159*pa)/180);
 	 tx1 = tx1 * sin((3.14159*pa)/180) - ty1 * cos((3.14159*pa)/180);
 	 tx2 = tx2 * sin((3.14159*pa)/180) - ty2 * cos((3.14159*pa)/180);
 	 /*[__________________________________]*/
 	 //screen->Bs_draw_line(15 - tx1,15 - tz1,15 - tx2,15 - tz2,33);
 	 //screen->Bs_draw_line(35,11,35, 8, 31);
 	 //screen->Bs_put_Pixel(35,11,32);
 	//std::cout<<"linedata: "<<tx1<<" "<<tz1<<" "<<tx2<<" "<<tz2<<std::endl;
 	/*PERSPECTIVE INSIDE PROGRAM*/
 	 
 	  if(tz1 > 0 || tz2 > 0){
 	  if(tz1 > -5 && tz2 > -5){
 	  if(tz1 < 1){
 	   tz1=1;
 	  }
 	  if(tz2 < 1){
 	   tz2=1;
 	  }
 	   if(tz1 == 0){
 	    x1 = (tx1+0.0001*-1) * 16 / tz1+0.0001; y1a = -32 / tz1+0.0001; y1b = 32 / tz1+0.0001;
 	   }else{
 	    x1 = (tx1*-1) * 16 / tz1; y1a = -32 / tz1; y1b = 32 / tz1;
 	   }  
 	 
 	   if(tz2 == 0){
 	    x2 = (tx2+0.0001*-1) * 16 / tz2+0.0001; y2a = -32 / tz2+0.0001; y2b = 32 / tz2+0.0001;
 	   }else{
 	    x2 = (tx2*-1) * 16 / tz2; y2a = -32 / tz2; y2b = 32 / tz2;
 	   }
 	   if(tz1 < 24 && tz2 < 24 && tz1 > -5 && tz2 > -5){
 	    display.Bs_draw_line(roundf(35+x1),roundf(11+y1a),roundf(35+x2),roundf(11+y2a),col);
 	    display.Bs_draw_line(roundf(35+x1),roundf(11+y1b),roundf(35+x2),roundf(11+y2b),col);
 	    display.Bs_draw_line(roundf(35+x1),roundf(11+y1a),roundf(35+x1),roundf(11+y1b),col);
 	    display.Bs_draw_line(roundf(35+x2),roundf(11+y2a),roundf(35+x2),roundf(11+y2b),col);
 	   }
 	  }
 	 }
 	/*[PERSPECTIVE WALL ALGORITHM]*/
}

void display_direction(Bs_cmd_graph display,int x, int y,int pa){
 std::string orient = "East";
 if(pa > 45 && pa < 135){
   orient = "North";
 }
 if(pa > 135 && pa < 225){
   orient = "West";
 }
 if(pa > 225 && pa < 315){
   orient = "South";
 }
 display.Bs_draw_text(x,y,"Facing: "+orient);
}

/*
 Engine for vertices inside my other engine thisone DESIGNED FOR REAL 3D OBJECTS IN ROOM
*/

struct vertex3d_type{
	int lx,ly,lz,lt;
	int wx,wy,wz,wt;
	int sx,sy;
};

struct line_type{int start, end;};

struct shapes3d_type{
	int color;
	int vertN;
	int lineN;
	vertex3d_type *points;
	line_type *line;
};

float matrix[4][4];

void Bs_set_projection(shapes3d_type *shape, int dist){
 for(int v = 0; v < (*shape).vertN; v++){
  vertex3d_type *vptr = &(*shape).points[v];
  vptr->sx = dist*vptr->wx/vptr->wz;
  vptr->sy = dist*vptr->wy/vptr->wz;
 }
}

void Bs_initialize_mtrx(){
 //initialize master transformation
 matrix[0][0] = 1; matrix[0][1] = 0; matrix[0][2] = 0; matrix[0][3] = 0;
 matrix[1][0] = 0; matrix[1][1] = 1; matrix[1][2] = 0; matrix[1][3] = 0;
 matrix[2][0] = 0; matrix[2][1] = 0; matrix[2][2] = 1; matrix[2][3] = 0;
 matrix[3][0] = 0; matrix[3][1] = 0; matrix[3][2] = 0; matrix[3][3] = 1;
}

void Bs_multiply_matrices(float resultado[4][4], float matrix1[4][4], float matrix2[4][4]){
	for(int y = 0; y < 4; y++){
	   for(int x = 0; x < 4; x++){
	      resultado[y][x] = 0;
	      for(int i = 0; i < 4; i++){
	      resultado[y][x] += matrix1[y][i] * matrix2[i][x]; 
	      }
	   }
	}
}

void Bs_copy_into_matrix(float matrix1[4][4],float matrix2[4][4]){
	for(int y = 0; y < 4; y++){
		for(int x = 0; x < 4; x++){
			matrix2[y][x] = matrix1[y][x];
		}	
	}
}

void Bs_scalef_shp(float scale){
 float mat[4][4];float smat[4][4];
 smat[0][0] = scale; smat[0][1] = 0; smat[0][2] = 0; smat[0][3] = 0;
 smat[1][0] = 0; smat[1][1] = scale; smat[1][2] = 0; smat[1][3] = 0;
 smat[2][0] = 0; smat[2][1] = 0; smat[2][2] = scale; smat[2][3] = 0;
 smat[3][0] = 0; smat[3][1] = 0; smat[3][2] = 0; smat[3][3] = 1;
 Bs_multiply_matrices(mat,smat,matrix);
 Bs_copy_into_matrix(mat,matrix);
}
void Bs_translatef_shp(int xt,int yt,int zt){
 float mat[4][4];float tmat[4][4];
 tmat[0][0] = 1; tmat[0][1] = 0; tmat[0][2] = 0; tmat[0][3] = 0;
 tmat[1][0] = 0; tmat[1][1] = 1; tmat[1][2] = 0; tmat[1][3] = 0;
 tmat[2][0] = 0; tmat[2][1] = 0; tmat[2][2] = 1; tmat[2][3] = 0;
 tmat[3][0] = xt;tmat[3][1] = yt;tmat[3][2] = zt;tmat[3][3] = 1;
 Bs_multiply_matrices(mat,matrix,tmat);
 Bs_copy_into_matrix(mat,matrix);
}
void Bs_rotatef_shp(float ax,float ay,float az){
 float xmat[4][4]; float ymat[4][4]; float zmat[4][4];
 float mat1[4][4]; float mat2[4][4];
 // x rotation
 xmat[0][0] = 1; xmat[0][1] = 0; xmat[0][2] = 0; xmat[0][3] = 0;
 xmat[1][0] = 0; xmat[1][1] = cos(ax);xmat[1][2] = sin(ax);xmat[1][3] = 0;
 xmat[2][0] = 0; xmat[2][1] = -sin(ax);xmat[2][2] = cos(ax);xmat[2][3] = 0;
 xmat[3][0] = 0; xmat[3][1] = 0; xmat[3][2] = 0; xmat[3][3] = 1;
 Bs_multiply_matrices(mat1,xmat,matrix);
 // y rotation
 ymat[0][0] = cos(ay); ymat[0][1] = 0; ymat[0][2] = -sin(ay); ymat[0][3] = 0;
 ymat[1][0] =       0; ymat[1][1] = 1; ymat[1][2] =        0; ymat[1][3] = 0;
 ymat[2][0] = sin(ay); ymat[2][1] = 0; ymat[2][2] =  cos(ay); ymat[2][3] = 0;
 ymat[3][0] =       0; ymat[3][1] = 0; ymat[3][2] =        0; ymat[3][3] = 1;
 Bs_multiply_matrices(mat2,ymat,mat1);
 // z rotation
 zmat[0][0] = cos(az); zmat[0][1] = sin(az); zmat[0][2] = 0; zmat[0][3] = 0;
 zmat[1][0] =-sin(az); zmat[1][1] = cos(az); zmat[1][2] = 0; zmat[1][3] = 0;
 zmat[2][0] = 0; zmat[2][1] = 0; zmat[2][2] = 1; zmat[2][3] = 0;
 zmat[3][0] = 0; zmat[3][1] = 0; zmat[3][2] = 0; zmat[3][3] = 1;
 Bs_multiply_matrices(matrix,zmat,mat2);
}

void Bs_transform_shp(shapes3d_type *shape){
 for(int v = 0; v < (*shape).vertN ; v++){
  vertex3d_type *vptr = &(*shape).points[v];
  vptr->wx = vptr->lx*matrix[0][0]+vptr->ly*matrix[1][0]+vptr->lz*matrix[2][0]+matrix[3][0];
  vptr->wy = vptr->lx*matrix[0][1]+vptr->ly*matrix[1][1]+vptr->lz*matrix[2][1]+matrix[3][1];
  vptr->wz = vptr->lx*matrix[0][2]+vptr->ly*matrix[1][2]+vptr->lz*matrix[2][2]+matrix[3][2];
 }
}

void Bs_draw_3dshp(shapes3d_type shape,Bs_cmd_graph display,int offsetX, int offsetY){
 for(int i = 0; i < shape.lineN; i++){
  display.Bs_draw_line(shape.points[shape.line[i].start].sx+offsetX,
	   shape.points[shape.line[i].start].sy+offsetY,
	   shape.points[shape.line[i].end].sx+offsetX,
	   shape.points[shape.line[i].end].sy+offsetY,
	   shape.color);
 }
}


void Bs_draw_3dpoly(shapes3d_type fig,Bs_cmd_graph display,int offsetX, int offsetY){
 int p2 = 0;
 for(int i = 0 ; i < fig.vertN ; i++){
  p2=i+1;
  if(p2>=fig.vertN){p2=0;}
  display.Bs_draw_line(fig.points[i].sx+offsetX,
	   fig.points[i].sy+offsetY,
	   fig.points[p2].sx+offsetX,
	   fig.points[p2].sy+offsetY,
	   fig.color);
 }
}

 */