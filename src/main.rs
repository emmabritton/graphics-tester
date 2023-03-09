use anyhow::Result;
use pixels_graphics_lib::buffer_graphics_lib::color::*;
use pixels_graphics_lib::buffer_graphics_lib::drawable::{Drawable, fill, stroke};
use pixels_graphics_lib::buffer_graphics_lib::prelude::InsertShape;
use pixels_graphics_lib::buffer_graphics_lib::shapes::collection::ShapeCollection;
use pixels_graphics_lib::buffer_graphics_lib::shapes::CreateDrawable;
use pixels_graphics_lib::buffer_graphics_lib::shapes::polyline::Polyline;
use pixels_graphics_lib::buffer_graphics_lib::text::format::{Positioning, TextFormat};
use pixels_graphics_lib::buffer_graphics_lib::text::format::Positioning::Center;
use pixels_graphics_lib::buffer_graphics_lib::text::pos::{CoordIntoTextPos, NewTextPos, TextPos};
use pixels_graphics_lib::buffer_graphics_lib::text::TextSize;
use pixels_graphics_lib::buffer_graphics_lib::text::TextSize::{Large, Normal};
use pixels_graphics_lib::buffer_graphics_lib::text::wrapping::WrappingStrategy;
use pixels_graphics_lib::graphics_shapes::coord::Coord;
use pixels_graphics_lib::graphics_shapes::triangle::{AnglePosition, FlatSide};
use pixels_graphics_lib::prelude::*;

struct Animation {
    pub value: f32,
    pub value_change: f32,
    pub next_update: f32,
    pub update_rate: f32
}

impl Animation {
    
    
    fn value_int(&self) -> isize {
        self.value as isize
    }
    
    fn update(&mut self, delta: f32) {
        self.next_update -= delta;
        if self.next_update <= 0.0 {
            self.value += self.value_change;
            self.next_update = self.update_rate;
        }
    }
    pub fn new(value: f32, value_change: f32, next_update: f32, update_rate: f32) -> Self {
        Self { value, value_change, next_update, update_rate }
    }
}

struct Example {
    current_test: usize,
    fast: Animation,
    slow: Animation,
    should_quit: bool,
    ici_static: IndexedImage,
    ici_slow: AnimatedIndexedImage,
    ici_fast: AnimatedIndexedImage
}

fn main() -> Result<()> {
    let (ici_static,_) = IndexedImage::from_file_contents(include_bytes!("../assets/test.ici")).unwrap();
    let (ici_slow,_) = AnimatedIndexedImage::from_file_contents(include_bytes!("../assets/slow.ica")).unwrap();
    let (ici_fast,_) = AnimatedIndexedImage::from_file_contents(include_bytes!("../assets/fast.ica")).unwrap();
    let system = Box::new(Example { should_quit:false, ici_static, ici_slow, current_test: 0, fast: Animation::new(0.0, 1.0, 0.0, 0.001),slow: Animation::new(0.0, 0.1, 0.0, 0.001), ici_fast });
    run(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize,  "Testing", system, Options::default())?;
    Ok(())
}

impl System for Example {
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![VirtualKeyCode::Left, VirtualKeyCode::Right, VirtualKeyCode::Space, VirtualKeyCode::Escape]
    }

    fn update(&mut self, timing: &Timing) {
        self.fast.update(timing.delta as f32);
        self.slow.update(timing.delta as f32);
        self.ici_slow.update(timing.fixed_time_step);
        self.ici_fast.update(timing.fixed_time_step);
    }

    fn render(&self, graphics: &mut Graphics) {
        graphics.clear(BLACK);
        match self.current_test {
            0 => test_0(graphics),
            1 => test_1(graphics),
            2 => test_2(graphics),
            3 => test_3(graphics),
            4 => test_4(graphics),
            5 => test_5(graphics),
            6 => test_6(graphics),
            7 => test_7(graphics),
            8 => test_8(graphics),
            9 => test_9(graphics),
            10 => test_10(graphics),
            11 => test_11(graphics),
            12 => test_12(graphics),
            13 => test_13(graphics),
            14 => test_14(graphics, self.fast.value_int()),
            15 => test_15(graphics, self.fast.value_int()),
            16 => test_16(graphics, self.fast.value_int()),
            17 => test_17(graphics, self.fast.value_int()),
            18 => test_18(graphics),
            19 => test_19(graphics),
            20 => test_20(graphics),
            21 => test_21(graphics, self.fast.value_int()),
            22 => test_22(graphics),
            23 => test_23(graphics),
            24 => test_24(graphics, self.slow.value_int()),
            25 => test_25(graphics),
            26 => test_26(graphics, &self.ici_static, &self.ici_slow, &self.ici_fast),
            27 => test_27(graphics),
            28 => test_28(graphics),
            29 => test_29(graphics, &self.ici_static),
            _ => graphics.draw_text(&format!("Unknown test: {}", self.current_test), CENTER.textpos(), TextFormat::from((RED, TextSize::Normal, Positioning::Center)))
        }
    }

    fn on_key_pressed(&mut self, keys: Vec<VirtualKeyCode>) {
        if keys.contains(&VirtualKeyCode::Right) {
            self.current_test += 1;
        } else if keys.contains(&VirtualKeyCode::Left) {
            if self.current_test > 0 {
                self.current_test -= 1;
            }
        } else if keys.contains(&VirtualKeyCode::Space) {
            self.current_test = 29;
        } else if keys.contains(&VirtualKeyCode::Escape) {
            self.should_quit = true;
        }
    }

    fn should_exit(&self) -> bool {
        self.should_quit
    }
}

const SCREEN_WIDTH: isize = 250;
const SCREEN_HEIGHT: isize = 250;
const HALF_WIDTH: isize = SCREEN_WIDTH / 2;
const HALF_HEIGHT: isize = SCREEN_HEIGHT / 2;
const TOP_LEFT: Coord = Coord::new(0, 0);
const TOP_RIGHT: Coord = Coord::new(SCREEN_WIDTH , 0);
const BOTTOM_TOP: Coord = Coord::new(0, SCREEN_HEIGHT );
const BOTTOM_RIGHT: Coord = Coord::new(SCREEN_WIDTH , SCREEN_HEIGHT );
const CENTER: Coord = Coord::new(SCREEN_WIDTH  / 2, SCREEN_HEIGHT  / 2);
const PADDING: Coord = Coord::new(8, 8);
const QUAD_TL: Coord = Coord::new(SCREEN_WIDTH / 4, SCREEN_HEIGHT / 4);
const QUAD_TR: Coord = Coord::new(SCREEN_WIDTH / 4 * 3, SCREEN_HEIGHT / 4);
const QUAD_BL: Coord = Coord::new(SCREEN_WIDTH / 4, SCREEN_HEIGHT / 4 *3);
const QUAD_BR: Coord = Coord::new(SCREEN_WIDTH / 4 *3, SCREEN_HEIGHT / 4 *3);

fn draw_title(graphics: &mut Graphics, text: &str) {
    graphics.draw_text(text, TextPos::px((HALF_WIDTH, 2)), TextFormat::from((WHITE, TextSize::Normal, Positioning::CenterTop)));
    graphics.draw_line((0, 11), (SCREEN_WIDTH, 11), WHITE);
    graphics.draw_line((0, 12), (SCREEN_WIDTH, 12), WHITE);
}

fn draw_point<P: Into<Coord>>(graphics: &mut Graphics, pos: P) {
    let pos = pos.into();
    graphics.set_pixel(pos.x, pos.y, RED);
}

fn test_0(graphics: &mut Graphics) {
    draw_title(graphics, "Text Positioning");

    graphics.draw_text("Center Bottom", QUAD_TL.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::CenterBottom) ));
    graphics.draw_text("Center Top", QUAD_TL.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::CenterTop) ));
    graphics.draw_text("Left Center", QUAD_TL.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::LeftCenter) ));
    graphics.draw_text("Right Center", QUAD_TL.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::RightCenter) ));
    draw_point(graphics, QUAD_TL);

    graphics.draw_text("Left Top", QUAD_TR.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::LeftTop) ));
    graphics.draw_text("Left Bottom", QUAD_TR.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::LeftBottom) ));
    graphics.draw_text("Right Top", QUAD_TR.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::RightTop) ));
    graphics.draw_text("Right Bottom", QUAD_TR.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::RightBottom) ));
    draw_point(graphics, QUAD_TR);

    graphics.draw_text("Center", QUAD_BL.textpos(), TextFormat::from((WHITE, TextSize::Small, Positioning::Center) ));
    draw_point(graphics, QUAD_BL);
}

fn test_1(graphics: &mut Graphics) {
    draw_title(graphics, "Para Sizing and Positioning (1/2)");

    graphics.draw_text("Lorem ipsum\nsample text\nfor sizing\nand positioning", QUAD_TL.textpos(), TextFormat::from((WHITE, TextSize::Small, WrappingStrategy::AtCol(8), Positioning::Center) ));
    graphics.draw_text("Lorem ipsum\nsample text\nfor sizing\nand positioning", QUAD_TR.textpos(), TextFormat::from((WHITE, TextSize::Small, WrappingStrategy::Cutoff(10), Positioning::Center) ));
    graphics.draw_text("Lorem ipsum\nsample text\nfor sizing\nand positioning", QUAD_BL.textpos(), TextFormat::from((WHITE, TextSize::Small, WrappingStrategy::SpaceBeforeCol(12), Positioning::Center) ));
    draw_point(graphics, QUAD_TL);
    draw_point(graphics, QUAD_TR);
    draw_point(graphics, QUAD_BL);
}

fn test_2(graphics: &mut Graphics) {
    draw_title(graphics, "Para Sizing and Positioning (2/2)");

    graphics.draw_text("Lorem ipsum\nsample text\nfor sizing\nand positioning", QUAD_TL.textpos(), TextFormat::from((WHITE, TextSize::Small, WrappingStrategy::AtCol(8), Positioning::LeftTop) ));
    graphics.draw_text("Lorem ipsum\nsample text\nfor sizing\nand positioning", QUAD_TR.textpos(), TextFormat::from((WHITE, TextSize::Small, WrappingStrategy::Cutoff(10), Positioning::RightCenter) ));
    graphics.draw_text("Lorem ipsum\nsample text\nfor sizing\nand positioning", QUAD_BL.textpos(), TextFormat::from((WHITE, TextSize::Small, WrappingStrategy::SpaceBeforeCol(12), Positioning::CenterBottom) ));
    draw_point(graphics, QUAD_TL);
    draw_point(graphics, QUAD_TR);
    draw_point(graphics, QUAD_BL);
}


fn test_3(graphics: &mut Graphics) {
    draw_title(graphics, "Right Angle Triangles");

    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::TopLeft), stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::TopRight), stroke(YELLOW)));
    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::BottomLeft), stroke(GREEN)));
    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::BottomRight), stroke(MAGENTA)));
}

fn test_4(graphics: &mut Graphics) {
    draw_title(graphics, "Basic shapes");

    graphics.draw(&Drawable::from_obj(Rect::new(QUAD_TL - PADDING, QUAD_TL + PADDING), stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Circle::new(QUAD_TR, PADDING.x as usize), stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Ellipse::new(QUAD_BL, PADDING.x as usize* 2, PADDING.x as usize), stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Triangle::new(QUAD_BR - PADDING, QUAD_BR + (PADDING.x, 0), QUAD_BR + (0,PADDING.x)), stroke(BLUE)));
}

fn test_5(graphics: &mut Graphics) {
    draw_title(graphics, "Text Symbols");

    graphics.draw_text("Size: NORMAL",TextPos::cr((1,2)), TextFormat::from((LIGHT_GRAY, TextSize::Normal)));
    graphics.draw_text("Letters:", TextPos::cr((1,3)), TextFormat::from((LIGHT_GRAY, TextSize::Normal)));
    graphics.draw_text("ABCDEFGHIJKLMNOPQRSTVWXYZ", TextPos::cr((1,4)), TextFormat::from((WHITE, TextSize::Normal)));
    graphics.draw_text("Numbers:", TextPos::cr((1,5)), TextFormat::from((LIGHT_GRAY, TextSize::Normal)));
    graphics.draw_text("0123456789", TextPos::cr((1,6)), TextFormat::from((WHITE, TextSize::Normal)));
    graphics.draw_text("Symbols:", TextPos::cr((1,7)), TextFormat::from((LIGHT_GRAY, TextSize::Normal)));
    graphics.draw_text("!@$%^&*(),./;'\\[]<>?:\"{}_+`~#", TextPos::cr((1,8)), TextFormat::from((WHITE, TextSize::Normal)));
    graphics.draw_text("Custom:", TextPos::cr((1,9)), TextFormat::from((LIGHT_GRAY, TextSize::Normal)));
    graphics.draw_text("…¤£¥¢", TextPos::cr((1,10)), TextFormat::from((WHITE, TextSize::Normal)));

    graphics.draw_text("Size: SMALL",TextPos::cr((1,17)), TextFormat::from((LIGHT_GRAY, TextSize::Small)));
    graphics.draw_text("Letters:", TextPos::cr((1,18)), TextFormat::from((LIGHT_GRAY, TextSize::Small)));
    graphics.draw_text("ABCDEFGHIJKLMNOPQRSTVWXYZ", TextPos::cr((1,19)), TextFormat::from((WHITE, TextSize::Small)));
    graphics.draw_text("Numbers:", TextPos::cr((1,20)), TextFormat::from((LIGHT_GRAY, TextSize::Small)));
    graphics.draw_text("0123456789", TextPos::cr((1,21)), TextFormat::from((WHITE, TextSize::Small)));
    graphics.draw_text("Symbols:", TextPos::cr((1,22)), TextFormat::from((LIGHT_GRAY, TextSize::Small)));
    graphics.draw_text("!@$%^&*(),./;'\\[]<>?:\"{}_+`~#", TextPos::cr((1,23)), TextFormat::from((WHITE, TextSize::Small)));
    graphics.draw_text("Custom:", TextPos::cr((1,24)), TextFormat::from((LIGHT_GRAY, TextSize::Small)));
    graphics.draw_text("…¤£¥¢", TextPos::cr((1,25)), TextFormat::from((WHITE, TextSize::Small)));
}

fn test_6(graphics: &mut Graphics) {
    draw_title(graphics, "Draw offset");

    let drawable = Drawable::from_obj(Rect::new((100,100),(120,120)), fill(BLUE));
    graphics.draw(&drawable);
    graphics.draw_offset((20, 20), &drawable);
    graphics.draw_offset((-20, -20), &drawable);
}

fn test_7(graphics: &mut Graphics) {
    draw_title(graphics, "Drawable mutation");

    let drawable = Drawable::from_obj(Rect::new((0,0),(20,20)).as_polygon(), fill(BLUE));
    let red = drawable.with_draw_type(stroke(RED));
    let rotated = drawable.with_rotation(45);
    let larger = drawable.with_scale(1.2);
    let smaller = drawable.with_scale(0.8);
    graphics.draw_offset((30,30),&drawable);
    graphics.draw_offset((100, 30), &red);
    graphics.draw_offset((100, 60), &rotated);
    graphics.draw_offset((100, 90), &smaller);
    graphics.draw_offset((100, 120), &larger);
    graphics.draw(&Drawable::from_obj(Rect::new((128,118),(152,142)).as_polygon(),fill(BLUE)));
}

fn test_8(graphics: &mut Graphics) {
    draw_title(graphics, "Polygons");
    let poly1=  Drawable::from_obj(Polygon::new(&[(30,30),(40,29),(50,50),(40,60)]), stroke(BLUE));
    graphics.draw(&poly1);
    graphics.draw_offset((0,60), &poly1.with_draw_type(fill(YELLOW)));
    graphics.draw_offset((60,60), &poly1.with_draw_type(fill(YELLOW)).with_rotation(45));
    graphics.draw_offset((120,60), &poly1.with_draw_type(fill(YELLOW)).with_rotation(80));
    graphics.draw_offset((180,60), &poly1.with_draw_type(fill(YELLOW)).with_rotation(160));
    graphics.draw_offset((00,120), &poly1.with_draw_type(fill(MAGENTA)).with_scale(1.5));
}

fn test_9(graphics: &mut Graphics) {
    draw_title(graphics, "Polygon mutation");

    let neg_drawable = Drawable::from_obj(Rect::new((0,0),(20,20)).as_polygon(), fill(BLUE));
    let neg_scaled = neg_drawable.with_scale(1.2);

    graphics.draw_offset(QUAD_BL, &neg_drawable);
    graphics.draw_offset(QUAD_BL + (40,0), &neg_scaled);

    let drawable = Drawable::from_obj(Rect::new((10,10),(30,30)).as_polygon(), fill(BLUE));
    let scaled = drawable.with_scale(1.2);

    graphics.draw_offset(QUAD_TL, &drawable);
    graphics.draw_offset(QUAD_TL + (40,0), &scaled);
}

fn test_10(graphics: &mut Graphics) {
    draw_title(graphics, "Off screen squares");

    graphics.draw(&Drawable::from_obj(Rect::new(TOP_LEFT - (10,10),TOP_LEFT + (10,10)), fill(BLUE)));
    graphics.draw(&Drawable::from_obj(Rect::new(BOTTOM_RIGHT - (10,10), BOTTOM_RIGHT + (10,10)), fill(BLUE)));

    graphics.draw_offset((50,50),&Drawable::from_obj(Rect::new(TOP_LEFT - (10,10),TOP_LEFT + (10,10)), fill(BLUE)));
    graphics.draw_offset((-50,-50),&Drawable::from_obj(Rect::new(BOTTOM_RIGHT - (10,10), BOTTOM_RIGHT + (10,10)), fill(BLUE)));
}

fn test_11(graphics: &mut Graphics) {
    draw_title(graphics, "Off screen circles");

    graphics.draw(&Drawable::from_obj(Circle::new(TOP_LEFT ,10), fill(BLUE)));
    graphics.draw(&Drawable::from_obj(Circle::new(BOTTOM_RIGHT, 10), fill(BLUE)));

    graphics.draw_offset((50,50),&Drawable::from_obj(Circle::new(TOP_LEFT ,10), fill(BLUE)));
    graphics.draw_offset((-50,-50),&Drawable::from_obj(Circle::new(BOTTOM_RIGHT, 10), fill(BLUE)));
}

fn test_12(graphics: &mut Graphics) {
    draw_title(graphics, "Off screen polygons");

    graphics.draw(&Drawable::from_obj(Polygon::new(&[TOP_LEFT - (10,10),TOP_LEFT + (10,-10), TOP_LEFT + (10,10), TOP_LEFT + (-10,10)]), fill(BLUE)));
    graphics.draw(&Drawable::from_obj(Polygon::new(&[BOTTOM_RIGHT - (10,10),BOTTOM_RIGHT + (10,-10), BOTTOM_RIGHT + (10,10), BOTTOM_RIGHT + (-10,10)]), fill(BLUE)));

    let left_poly = Drawable::from_obj(Polygon::new(&[(-10,50),(20,50),(20,70),(-10,70)]), fill(BLUE));
    graphics.draw(&left_poly);
    graphics.draw_offset((50,0),&left_poly);

    let top_poly = Drawable::from_obj(Polygon::new(&[(200,-10),(220,-10),(220,30),(200,30)]), fill(BLUE));
    graphics.draw(&top_poly);
    graphics.draw_offset((0,70),&top_poly);

    let right_poly = Drawable::from_obj(Polygon::new(&[(230,130),(260,130),(260,150),(230,150)]), fill(BLUE));
    graphics.draw(&right_poly);
    graphics.draw_offset((-50,0),&right_poly);

    let bottom_poly = Drawable::from_obj(Polygon::new(&[(100,230),(120,230),(120,260),(100,260)]), fill(BLUE));
    graphics.draw(&bottom_poly);
    graphics.draw_offset((0,-70),&bottom_poly);
}

fn test_13(graphics: &mut Graphics) {
    draw_title(graphics, "Triangles");

    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_TL, 20, FlatSide::Left), fill(MAGENTA)));
    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_TR, 20, FlatSide::Bottom), fill(MAGENTA)));
    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_BR, 20, FlatSide::Right), fill(MAGENTA)));
    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_BL, 20, FlatSide::Top), fill(MAGENTA)));
}

fn test_14(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "Poly Rotation - Stroke");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let rect = Rect::new(CENTER - (20,20), CENTER + (20,20)).as_polygon();
    let drawable = Drawable::from_obj(rect, stroke(MAGENTA));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_15(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "Poly Rotation - Filled");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let rect = Rect::new(CENTER - (20,20), CENTER + (20,20)).as_polygon();
    let drawable = Drawable::from_obj(rect, fill(RED));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_16(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "Triangle Rotation - Stroke");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let triangle = Triangle::equilateral(CENTER, 40, FlatSide::Bottom);
    let drawable = Drawable::from_obj(triangle, stroke(MAGENTA));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_17(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "Triangle Rotation - Filled");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let triangle = Triangle::equilateral(CENTER, 40, FlatSide::Bottom);
    let drawable = Drawable::from_obj(triangle, fill(RED));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_18(graphics: &mut Graphics) {
    draw_title(graphics, "Line Rotation");

    graphics.draw_line((60,50),(60,150), YELLOW);
    graphics.draw_line((160,50),(160,150), YELLOW);

    let line1 = Line::new((60,50),(60,150)).rotate(47);
    let line2 = Line::new((160,50),(160,150)).rotate_around(47,(160,150));

    graphics.draw(&Drawable::from_obj(line1, stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(line2, stroke(BLUE)));
}

fn test_19(graphics: &mut Graphics) {
    draw_title(graphics, "Moving shapes");

    let triangle = Triangle::equilateral((40,40),10, FlatSide::Left);
    let moved = triangle.move_to((60,40));
    let translated = triangle.translate_by((0,20));

    graphics.draw_triangle(triangle, fill(BLUE));
    graphics.draw_triangle(moved, fill(YELLOW));
    graphics.draw_triangle(translated, fill(RED));


    let rect = Rect::new((140,30),(170,50));
    let moved = rect.move_to((180,30));
    let translated = rect.translate_by((0,30));

    graphics.draw_rect(rect, fill(BLUE));
    graphics.draw_rect(moved, fill(YELLOW));
    graphics.draw_rect(translated, fill(RED));


    let polygon = Polygon::new(&[(40, 120),(60,120),(55,130),(30,150)]);
    let moved = polygon.move_to((100,120));
    let translated = polygon.translate_by((0,30));

    graphics.draw_polygon(polygon, fill(BLUE));
    graphics.draw_polygon(moved, fill(YELLOW));
    graphics.draw_polygon(translated, fill(RED));
}

fn test_20(graphics: &mut Graphics) {
    draw_title(graphics, "Basic polyline");

    graphics.draw(&Polyline::rounded_rect(150, 40, 220, 120, 20, BLUE).unwrap());
    graphics.draw(&Polyline::rounded_rect(180, 70, 200, 90, 4, YELLOW).unwrap());
}

fn test_21(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "Arcs");

    graphics.draw_arc(QUAD_TL, 0, 90, 20, false, RED);

    graphics.draw_arc(QUAD_TR, degrees,degrees+30, 40, false, BLUE);
    graphics.draw_arc(QUAD_TR, degrees+10,degrees+40, 39, false, GREEN);
    graphics.draw_arc(QUAD_TR, degrees+20,degrees+50, 38, false, RED);

    graphics.draw_arc(QUAD_BL, 0, 300, 4, false, YELLOW);
    graphics.draw_arc(QUAD_BL, 0, 300, 4, false, ORANGE);

    graphics.draw_arc(QUAD_BR, 0, 90, 20, true, MAGENTA);
}

fn test_22(graphics: &mut Graphics) {
    draw_title(graphics, "Colors");

    let colors = &[WHITE, LIGHT_GRAY,  RED, DARK_GRAY,GREEN, BLUE, YELLOW, MAGENTA, PURPLE, ORANGE, CYAN, BROWN];
    let names= &["WHITE", "LIGHT GRAY", "RED", "DARK GRAY", "GREEN", "BLUE", "YELLOW", "MAGENTA", "PURPLE", "ORANGE", "CYAN", "BROWN"];

    let start = Coord::new(50,40);
    let mut row = 0;
    let mut col = 0;
    let row_space = 120;
    let col_space = 20;
    for (i,color) in colors.iter().enumerate() {
        let coord = Coord::from((row * row_space, col * col_space));
        graphics.draw_text(&format!("{}", names[i]), TextPos::px(coord + start), (*color, Large, Center));
        row += 1;
        if row > 1 {
            row = 0;
            col += 1;
        }
    }
}

fn test_23(graphics: &mut Graphics) {
    draw_title(graphics, "Collections");

    let mut collection = ShapeCollection::new();
    InsertShape::insert_above(&mut collection, Rect::new((150,150),(170,190)).as_polygon(), stroke(BLUE));
    InsertShape::insert_above(&mut collection, Rect::new((190,150),(210,190)).as_polygon(), fill(BLUE));

    graphics.draw(&collection);

    graphics.draw(&collection.with_move((20,20)).with_draw_type(fill(YELLOW)));

    graphics.draw(&collection.with_translation((-80,00)).with_draw_type(fill(PURPLE)));

    graphics.draw(&collection.with_move((190,20)).with_draw_type(fill(MAGENTA)).with_scale(0.6));
}

fn test_24(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "Rotating collections");

    let mut collection = ShapeCollection::new();
    InsertShape::insert_above(&mut collection, Rect::new((100,0),(120,30)).as_polygon(), stroke(BLUE));
    InsertShape::insert_above(&mut collection, Rect::new((130,0),(150,30)).as_polygon(), fill(BLUE));

    graphics.draw(&collection.with_rotation_around(degrees, (0,0)));

    let mut collection = ShapeCollection::new();
    InsertShape::insert_above(&mut collection, Rect::new((30,30),(50,60)).as_polygon(), stroke(YELLOW));
    InsertShape::insert_above(&mut collection, Rect::new((60,60),(80,80)).as_polygon(), fill(YELLOW));

    graphics.draw(&collection.with_rotation_around(degrees, (0,0)));

    let mut collection = ShapeCollection::new();
    InsertShape::insert_above(&mut collection, Rect::new((150,150),(170,170)).as_polygon(), stroke(MAGENTA));
    InsertShape::insert_above(&mut collection, Rect::new((170,170),(190,190)).as_polygon(), fill(MAGENTA));

    graphics.draw(&collection.with_rotation(degrees));

}

fn test_25(graphics: &mut Graphics) {
    draw_title(graphics, "Text bounds");

    let short = "one line";
    let long = "multiple lines of text";

    let bounds_short_normal = Normal.measure(short, WrappingStrategy::None);
    let bounds_short_large = Large.measure(short, WrappingStrategy::None);
    let bounds_multi_normal = Normal.measure(long, WrappingStrategy::AtCol(6));
    let bounds_multi_large = Large.measure(long, WrappingStrategy::AtCol(6));

    graphics.draw_rect(Rect::new((0,0), bounds_short_normal).move_center_to(QUAD_TL), stroke(BLUE));
    graphics.draw_rect(Rect::new((0,0), bounds_short_large).move_center_to(QUAD_TR), stroke(BLUE));
    graphics.draw_rect(Rect::new((0,0), bounds_multi_normal).move_center_to(QUAD_BL), stroke(BLUE));
    graphics.draw_rect(Rect::new((0,0), bounds_multi_large).move_center_to(QUAD_BR), stroke(BLUE));

    graphics.draw_text(short, TextPos::px(QUAD_TL), (WHITE, Normal, WrappingStrategy::None, Center));
    graphics.draw_text(short, TextPos::px(QUAD_TR), (WHITE, Large, WrappingStrategy::None, Center));
    graphics.draw_text(long, TextPos::px(QUAD_BL), (WHITE, Normal, WrappingStrategy::AtCol(6), Center));
    graphics.draw_text(long, TextPos::px(QUAD_BR), (WHITE, Large, WrappingStrategy::AtCol(6), Center));

}

fn test_26(graphics: &mut Graphics, image: &IndexedImage, slow: &AnimatedIndexedImage, fast: &AnimatedIndexedImage) {
    draw_title(graphics, "Indexed Images");

    graphics.draw_indexed_image((30,30),image);
    graphics.draw_animated_image((130,30),slow);
    graphics.draw_animated_image((130,50),fast);
}

fn test_27(graphics: &mut Graphics) {
    draw_title(graphics, "Color brightness");

    let color = Color::rgb(124, 67, 43);

    let brighter = color.lighten();
    let brighter2 = brighter.lighten();
    let brighter3 = brighter2.lighten();
    let darker = color.darken();
    let darker2 = darker.darken();
    let darker3 = darker2.darken();

    let rect = Drawable::from_obj(Rect::new((0,0), (30,30)), DrawType::Fill(WHITE));
    rect.with_move((10,100)).with_draw_type(fill(darker3)).render(graphics);
    rect.with_move((40,100)).with_draw_type(fill(darker2)).render(graphics);
    rect.with_move((70,100)).with_draw_type(fill(darker)).render(graphics);
    rect.with_move((100,100)).with_draw_type(fill(color)).render(graphics);
    rect.with_move((130,100)).with_draw_type(fill(brighter)).render(graphics);
    rect.with_move((160,100)).with_draw_type(fill(brighter2)).render(graphics);
    rect.with_move((190,100)).with_draw_type(fill(brighter3)).render(graphics);

}

fn test_28(graphics: &mut Graphics) {
    draw_title(graphics, "Color saturation");

    let color = Color::rgb(124, 197, 93);

    let brighter = color.saturate();
    let brighter2 = brighter.saturate();
    let brighter3 = brighter2.saturate();
    let darker = color.desaturate();
    let darker2 = darker.desaturate();
    let darker3 = darker2.desaturate();

    let rect = Drawable::from_obj(Rect::new((0,0), (30,30)), DrawType::Fill(WHITE));
    rect.with_move((10,100)).with_draw_type(fill(darker3)).render(graphics);
    rect.with_move((40,100)).with_draw_type(fill(darker2)).render(graphics);
    rect.with_move((70,100)).with_draw_type(fill(darker)).render(graphics);
    rect.with_move((100,100)).with_draw_type(fill(color)).render(graphics);
    rect.with_move((130,100)).with_draw_type(fill(brighter)).render(graphics);
    rect.with_move((160,100)).with_draw_type(fill(brighter2)).render(graphics);
    rect.with_move((190,100)).with_draw_type(fill(brighter3)).render(graphics);
}


fn test_29(graphics: &mut Graphics, image: &IndexedImage) {
    draw_title(graphics, "Changing images");

    let mut orig = image.clone();
    let mut palette = orig.get_palette().to_vec();
    palette.push(IciColor::new(125,16,150, 255));
    orig.set_palette(&palette).unwrap();
    orig.set_pixel(13, (palette.len() - 1) as u8).unwrap();
    let darker = orig.with_brightness(0.6);
    let sated = orig.with_saturate(-0.2);

    graphics.draw_indexed_image((100,100),&orig);
    graphics.draw_indexed_image((50,100),&darker);
    graphics.draw_indexed_image((150,100),&sated);
}
