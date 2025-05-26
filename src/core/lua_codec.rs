use mlua::{Function, Lua};
use std::{cell::RefCell, rc::Rc, usize};
use crate::draw::{Area, Draw, Point};

pub struct LuaCodec {
    lua: Lua,
    draw: Rc<RefCell<Draw>>,
    editor_init: Option<Function>,
    editor_draw: Option<Function>,
}

impl LuaCodec {
    pub fn new() -> Self {
        let lua = Lua::new();
        let draw = Rc::new(RefCell::new(Draw::new()));

        let mut codec = LuaCodec {
            lua,
            draw,
            editor_init: None,
            editor_draw: None,
        };

        codec.load_editor_main();
        codec.add_lua_functions();
        codec.read_lua_functions();

        codec.init();

        codec
    }

    pub fn init(&mut self){
        if let Some(editor_init) = &self.editor_init {
            if let Err(e) = editor_init.call::<()>(()) {
                eprintln!("Error executing _init: {}", e);
                return;
            }
        } else {
            eprintln!("editor_init is not set");
        }
    }

    pub fn draw(&mut self){
        if let Some(editor_draw) = &self.editor_draw {
            if let Err(e) = editor_draw.call::<()>(()) {
                eprintln!("Error executing _draw: {}", e);
                return;
            }
        } else {
            eprintln!("editor_draw is not set");
        }

        self.draw.borrow_mut().update();
    }

    fn load_editor_main(&mut self){
        let script = match std::fs::read_to_string("src/FITU-16/editor_main.lua") {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading Lua file: {}", e);
                return;
            }
        };

        if let Err(e) = self.lua.load(&script).exec() {
            eprintln!("Error executing Lua script: {}", e);
            return;
        }
    }

    fn add_lua_functions(&mut self) { 
        let printdb = match self.lua.create_function(move |_, content: String| {
            println!("{}", content);
            Ok(())
        }) {
            Ok(func) => func,
            Err(e) => {
                eprintln!("Failed to create printdb function: {}", e);
                return;
            }
        };

        let draw_ref = self.draw.clone();
        let rectfill = match self.lua.create_function(move |_, (area, color_id): (Area, usize)| {
            draw_ref.borrow_mut().rect_fill(area, color_id);
            Ok(())
        }) {
            Ok(func) => func,
            Err(e) => {
                eprintln!("Failed to create rectfill function: {}", e);
                return;
            }
        };

        let draw_ref = self.draw.clone();
        let line = match self.lua.create_function(move |_, (p1, p2, color_id): (Point, Point, usize)| {
            draw_ref.borrow_mut().line(p1, p2, color_id);
            Ok(())
        }) {
            Ok(func) => func,
            Err(e) => {
                eprintln!("Failed to create line function: {}", e);
                return;
            }
        };

        let draw_ref = self.draw.clone();
        let cls = match self.lua.create_function(move |_, color_id: usize| {
            draw_ref.borrow_mut().cls(color_id);
            Ok(())
        }) {
            Ok(func) => func,
            Err(e) => {
                eprintln!("Failed to create cls function: {}", e);
                return;
            }
        };

        if let Err(e) = self.lua.globals().set("printdb", printdb) {
            eprintln!("Error registering printdb function: {}", e);
            return;
        }

        if let Err(e) = self.lua.globals().set("rectfill", rectfill) {
            eprintln!("Error registering rectfill function: {}", e);
            return;
        }

        if let Err(e) = self.lua.globals().set("line", line) {
            eprintln!("Error registering line function: {}", e);
            return;
        }

        if let Err(e) = self.lua.globals().set("cls", cls) {
            eprintln!("Error registering cls function: {}", e);
            return;
        }
    }

    fn read_lua_functions(&mut self) {
        match self.lua.globals().get::<Function>("_init") {
            Ok(func) => self.editor_init = Some(func),
            Err(e) => eprintln!("Error getting _init function: {}", e),
        };
    
        match self.lua.globals().get::<Function>("_draw") {
            Ok(func) => self.editor_draw = Some(func),
            Err(e) => eprintln!("Error getting _draw function: {}", e),
        };
    }
}