mod lib;
use std::time::Duration;
use std::thread;

fn main() {
    let mut screen = lib::BsCmdGraph::new(50, 50, '#' as i32);
    
    loop {
        screen.cmd_clear();

        screen.draw_text(15, 15, "Hello, World!");
        screen.draw_text(15, 20, "It works!!!");

        screen.cmd_draw();
        
        // TODO: Find a better way to make this thing work that doesn't involve stopping execution
        thread::sleep(Duration::from_millis(5));
    }
}
