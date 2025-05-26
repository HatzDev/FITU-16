use mlua::{FromLua, Lua, Result, Table, Value};

use crate::scene::{Scene, HEIGHT, WIDTH};

pub struct Area{
    pub x: u8,
    pub y: u8,
    pub width: u8,
    pub height: u8
}

pub struct Point {
    pub x: u8,
    pub y: u8,
}

impl<'lua> FromLua for Area {
    fn from_lua(lua_value: Value, lua: &Lua) -> Result<Self> {
        let table = Table::from_lua(lua_value, lua)?;
        Ok(Area {
            x: table.get("x")?,
            y: table.get("y")?,
            width: table.get("width")?,
            height: table.get("height")?,
        })
    }
}

impl<'lua> FromLua for Point{
    fn from_lua(value: Value, lua: &Lua) -> Result<Self> {
        let table = Table::from_lua(value, lua)?;
        Ok(Point {
            x: table.get("x")?,
            y: table.get("y")?
        })
    }
}

pub struct Draw{
    scene: Scene
}

impl Draw{
    pub fn new() -> Self {
        let scene = Scene::new();
        
        Draw { scene }
    }

    pub fn cls(&mut self, color_id: usize) {
        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                self.scene.set_pixel(x, y, color_id);
            }
        }
    }

    pub fn rect_fill(&mut self, area: Area, color_id: usize) {
        for y in area.y..area.y.saturating_add(area.height) {
            for x in area.x..area.x.saturating_add(area.width) {
                self.scene.set_pixel(x, y, color_id);
            }
        }
    }

    pub fn line(&mut self, p1: Point, p2: Point, color_id: usize) {
        let mut x0 = p1.x as i16;
        let mut y0 = p1.y as i16;
        let x1 = p2.x as i16;
        let y1 = p2.y as i16;

        let dx = (x1 - x0).abs();
        let dy = (y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx - dy;

        loop {
            if x0 >= 0 && y0 >= 0 && x0 < WIDTH.into() && y0 < HEIGHT.into(){
                self.scene.set_pixel(x0 as u8, y0 as u8, color_id);
            }

            if x0 == x1 && y0 == y1 {
                break;
            }

            let e2 = 2 * err;
            if e2 > -dy {
                err -= dy;
                x0 += sx;
            }
            if e2 < dx {
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn update(&mut self){
        self.scene.update();
    }
}