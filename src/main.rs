use anyhow::Result;
use buffer_graphics_lib::color::{BLACK, BLUE, CYAN, GREEN, LIGHT_GRAY, MAGENTA, RED, WHITE, YELLOW};
use buffer_graphics_lib::drawable::{Drawable, DrawType};
use buffer_graphics_lib::Graphics;
use buffer_graphics_lib::shapes::CreateDrawable;
use buffer_graphics_lib::text::format::{Positioning, TextFormat};
use buffer_graphics_lib::text::pos::{NewTextPos, TextPos};
use buffer_graphics_lib::text::TextSize;
use graphics_shapes::coord::Coord;
use pixels_graphics_lib::{run, System, WindowScaling};
use winit::event::VirtualKeyCode;
use buffer_graphics_lib::text::pos::CoordIntoTextPos;
use buffer_graphics_lib::text::wrapping::WrappingStrategy;
use graphics_shapes::circle::Circle;
use graphics_shapes::ellipse::Ellipse;
use graphics_shapes::polygon::Polygon;
use graphics_shapes::rect::Rect;
use graphics_shapes::triangle::{AnglePosition, FlatSide, Triangle};

#[derive(Default)]
struct Example {
    current_test: usize,
}

fn main() -> Result<()> {
    let system = Box::new(Example::default());
    run(SCREEN_WIDTH as usize, SCREEN_HEIGHT as usize, WindowScaling::Auto, "Testing", system)?;
    Ok(())
}

impl System for Example {
    fn action_keys(&self) -> Vec<VirtualKeyCode> {
        vec![VirtualKeyCode::Left, VirtualKeyCode::Right]
    }

    fn update(&mut self, delta: f32) {}
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
        }
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

    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::TopLeft), DrawType::Stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::TopRight), DrawType::Stroke(YELLOW)));
    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::BottomLeft), DrawType::Stroke(GREEN)));
    graphics.draw(&Drawable::from_obj(Triangle::right_angle(CENTER, 100,100,AnglePosition::BottomRight), DrawType::Stroke(MAGENTA)));
}

fn test_4(graphics: &mut Graphics) {
    draw_title(graphics, "Basic shapes");

    graphics.draw(&Drawable::from_obj(Rect::new(QUAD_TL - PADDING, QUAD_TL + PADDING), DrawType::Stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Circle::new(QUAD_TR, PADDING.x as usize), DrawType::Stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Ellipse::new(QUAD_BL, PADDING.x as usize* 2, PADDING.x as usize), DrawType::Stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(Triangle::new(QUAD_BR - PADDING, QUAD_BR + (PADDING.x, 0), QUAD_BR + (0,PADDING.x)), DrawType::Stroke(BLUE)));
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

    let drawable = Drawable::from_obj(Rect::new((100,100),(120,120)), DrawType::Fill(BLUE));
    graphics.draw(&drawable);
    graphics.draw_offset((20, 20), &drawable);
    graphics.draw_offset((-20, -20), &drawable);
}

fn test_7(graphics: &mut Graphics) {
    draw_title(graphics, "Drawable mutation");

    let drawable = Drawable::from_obj(Rect::new((0,0),(20,20)).as_polygon(), DrawType::Fill(BLUE));
    let red = drawable.with_draw_type(DrawType::Stroke(RED));
    let rotated = drawable.with_rotation(45);
    let larger = drawable.with_scale(1.2);
    let smaller = drawable.with_scale(0.8);
    graphics.draw_offset((30,30),&drawable);
    graphics.draw_offset((100, 30), &red);
    graphics.draw_offset((100, 60), &rotated);
    graphics.draw_offset((100, 90), &smaller);
    graphics.draw_offset((100, 120), &larger);
    graphics.draw(&Drawable::from_obj(Rect::new((128,118),(152,142)).as_polygon(),DrawType::Fill(BLUE)));
}

fn test_8(graphics: &mut Graphics) {
    draw_title(graphics, "Polygons");
    let poly1=  Drawable::from_obj(Polygon::new(vec![(30,30),(40,29),(50,50),(40,60)]), DrawType::Stroke(BLUE));
    graphics.draw(&poly1);
    graphics.draw_offset((0,60), &poly1.with_draw_type(DrawType::Fill(YELLOW)));
    graphics.draw_offset((60,60), &poly1.with_draw_type(DrawType::Fill(YELLOW)).with_rotation(45));
    graphics.draw_offset((120,60), &poly1.with_draw_type(DrawType::Fill(YELLOW)).with_rotation(80));
    graphics.draw_offset((180,60), &poly1.with_draw_type(DrawType::Fill(YELLOW)).with_rotation(160));
    graphics.draw_offset((00,120), &poly1.with_draw_type(DrawType::Fill(MAGENTA)).with_scale(1.5));
}

fn test_9(graphics: &mut Graphics) {
    draw_title(graphics, "Polygon mutation");

    let neg_drawable = Drawable::from_obj(Rect::new((0,0),(20,20)).as_polygon(), DrawType::Fill(BLUE));
    let neg_scaled = neg_drawable.with_scale(1.2);

    graphics.draw_offset(QUAD_BL, &neg_drawable);
    graphics.draw_offset(QUAD_BL + (40,0), &neg_scaled);

    let drawable = Drawable::from_obj(Rect::new((10,10),(30,30)).as_polygon(), DrawType::Fill(BLUE));
    let scaled = drawable.with_scale(1.2);

    graphics.draw_offset(QUAD_TL, &drawable);
    graphics.draw_offset(QUAD_TL + (40,0), &scaled);
}

fn test_10(graphics: &mut Graphics) {
    draw_title(graphics, "Off screen squares");

    graphics.draw(&Drawable::from_obj(Rect::new(TOP_LEFT - (10,10),TOP_LEFT + (10,10)), DrawType::Fill(BLUE)));
    graphics.draw(&Drawable::from_obj(Rect::new(BOTTOM_RIGHT - (10,10), BOTTOM_RIGHT + (10,10)), DrawType::Fill(BLUE)));

    graphics.draw_offset((50,50),&Drawable::from_obj(Rect::new(TOP_LEFT - (10,10),TOP_LEFT + (10,10)), DrawType::Fill(BLUE)));
    graphics.draw_offset((-50,-50),&Drawable::from_obj(Rect::new(BOTTOM_RIGHT - (10,10), BOTTOM_RIGHT + (10,10)), DrawType::Fill(BLUE)));
}

fn test_11(graphics: &mut Graphics) {
    draw_title(graphics, "Off screen circles");

    graphics.draw(&Drawable::from_obj(Circle::new(TOP_LEFT ,10), DrawType::Fill(BLUE)));
    graphics.draw(&Drawable::from_obj(Circle::new(BOTTOM_RIGHT, 10), DrawType::Fill(BLUE)));

    graphics.draw_offset((50,50),&Drawable::from_obj(Circle::new(TOP_LEFT ,10), DrawType::Fill(BLUE)));
    graphics.draw_offset((-50,-50),&Drawable::from_obj(Circle::new(BOTTOM_RIGHT, 10), DrawType::Fill(BLUE)));
}

fn test_12(graphics: &mut Graphics) {
    draw_title(graphics, "Off screen polygons");

    graphics.draw(&Drawable::from_obj(Polygon::new(vec![TOP_LEFT - (10,10),TOP_LEFT + (10,-10), TOP_LEFT + (10,10), TOP_LEFT + (-10,10)]), DrawType::Fill(BLUE)));
    graphics.draw(&Drawable::from_obj(Polygon::new(vec![BOTTOM_RIGHT - (10,10),BOTTOM_RIGHT + (10,-10), BOTTOM_RIGHT + (10,10), BOTTOM_RIGHT + (-10,10)]), DrawType::Fill(BLUE)));

    let left_poly = Drawable::from_obj(Polygon::new(vec![(-10,50),(20,50),(20,70),(-10,70)]), DrawType::Fill(BLUE));
    graphics.draw(&left_poly);
    graphics.draw_offset((50,0),&left_poly);

    let top_poly = Drawable::from_obj(Polygon::new(vec![(200,-10),(220,-10),(220,30),(200,30)]), DrawType::Fill(BLUE));
    graphics.draw(&top_poly);
    graphics.draw_offset((0,70),&top_poly);

    let right_poly = Drawable::from_obj(Polygon::new(vec![(230,130),(260,130),(260,150),(230,150)]), DrawType::Fill(BLUE));
    graphics.draw(&right_poly);
    graphics.draw_offset((-50,0),&right_poly);

    let bottom_poly = Drawable::from_obj(Polygon::new(vec![(100,230),(120,230),(120,260),(100,260)]), DrawType::Fill(BLUE));
    graphics.draw(&bottom_poly);
    graphics.draw_offset((0,-70),&bottom_poly);
}

fn test_13(graphics: &mut Graphics) {
    draw_title(graphics, "Triangles");

    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_TL, 20, FlatSide::Left), DrawType::Fill(MAGENTA)));
    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_TR, 20, FlatSide::Bottom), DrawType::Fill(MAGENTA)));
    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_BR, 20, FlatSide::Right), DrawType::Fill(MAGENTA)));
    graphics.draw(&Drawable::from_obj(Triangle::equilateral(QUAD_BL, 20, FlatSide::Top), DrawType::Fill(MAGENTA)));
}