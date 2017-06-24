
extern crate tcod;


use tcod::console::*;
use tcod::colors;
use tcod::input::Key;
use tcod::input::KeyCode::*;


const SCREEN_WIDTH: i32 = 80;
const SCREEN_HEIGHT: i32 = 50;
const LIMIT_FPS: i32 = 20;

pub fn run() {

    //OpenGL is needed on my mac. Otherwize it will render a white screen on startup
    let mut root = Root::initializer()
        .font("arial10x10.png", FontLayout::Tcod)
        .font_type(FontType::Greyscale)
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("Rust/libtcod tutorial")
        .renderer(Renderer::OpenGL)     
        .init();
    
    tcod::system::set_fps(LIMIT_FPS);
        
    let mut tick = 0;

    let (mut player_x, mut player_y ) = (SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);

    while !root.window_closed(){
        tick += 1;

        root.set_default_foreground(colors::WHITE);
        root.print(1, 1, tick.to_string());
        root.put_char(player_x, player_y, '@', BackgroundFlag::None);
        root.flush();
        

        //Clear previous player location in case the player moved.
        root.put_char(player_x, player_y, ' ', BackgroundFlag::None);
        let exit = handle_keys(&mut root, &mut player_x, &mut player_y);
        if exit {
            break;
        }

    }

    fn handle_keys(root: &mut Root, player_x: &mut i32, player_y: &mut i32) -> bool {
        
        //real time
        //I'm thinking if you want animations outside of a turnbased game this is what you 
        //would need. (plus emulating your own simulation ticks. (your own turned based code))
        /*
        let key = match root.check_for_keypress(tcod::input::KEY_PRESSED){
            Some(key) => key,
            None => return false,
        };
        */

        //turn based
        let key = root.wait_for_keypress(true);

        match key {
            Key{code: Up, ..} => *player_y -=1,
            Key{code: Down, ..} => *player_y +=1,
            Key{code: Left, ..} => *player_x -=1,
            Key{code: Right, ..} => *player_x +=1,
            
            Key{code: Enter, alt:true, ..} => {
                let currently_fullscreen = root.is_fullscreen();
                root.set_fullscreen(!currently_fullscreen);
            },
            Key{code: Escape, ..} => {
                return true
            }
            _ => {}, 
        }

        false
    }


}
