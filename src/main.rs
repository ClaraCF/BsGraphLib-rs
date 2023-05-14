use bsgraphlib;
use std::thread;
use std::time;

fn main() {
    let mut screen = bsgraphlib::BsCmdGraph::new(50, 50, '#' as i32);
    
    loop {
        screen.cmd_clear();

        screen.draw_text(15, 15, "Hello, World!");
        screen.draw_text(15, 20, "It works!!!");

        screen.cmd_draw();
        
        /* FIXME: If you don't add this delay, the screen breaks
         * Find a way to make this work without stopping execution */
        thread::sleep(time::Duration::from_millis(5));
    }
}

// TODO: Write tests
