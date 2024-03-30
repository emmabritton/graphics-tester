use anyhow::Result;
use pixels_graphics_lib::buffer_graphics_lib::renderable_macros::DrawOffset;
use pixels_graphics_lib::buffer_graphics_lib::CustomLetter;
use pixels_graphics_lib::prelude::palette::simplify_palette;
use pixels_graphics_lib::prelude::*;
use pixels_graphics_lib::prelude::font::standard_4x5;

struct Animation {
    pub value: f32,
    pub value_change: f32,
    pub next_update: f32,
    pub update_rate: f32,
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
        Self {
            value,
            value_change,
            next_update,
            update_rate,
        }
    }
}

struct Example {
    current_test: usize,
    fast: Animation,
    slow: Animation,
    should_quit: bool,
    ici_static: IndexedImage,
    ici_slow: AnimatedIndexedImage,
    ici_fast: AnimatedIndexedImage,
    image: IndexedImage,
    mouse_xy: Coord,
}

fn main() -> Result<()> {
    let (ici_static, _) =
        IndexedImage::from_file_contents(include_bytes!("../assets/test.ici")).unwrap();
    let (ici_slow, _) =
        AnimatedIndexedImage::from_file_contents(include_bytes!("../assets/slow.ica")).unwrap();
    let (ici_fast, _) =
        AnimatedIndexedImage::from_file_contents(include_bytes!("../assets/fast.ica")).unwrap();
    let (image, _) =
        IndexedImage::from_file_contents(include_bytes!("../assets/image.ici")).unwrap();
    let system = Box::new(Example {
        image,
        should_quit: false,
        ici_static,
        ici_slow,
        current_test: 0,
        fast: Animation::new(0.0, 1.0, 0.0, 0.001),
        slow: Animation::new(0.0, 0.1, 0.0, 0.001),
        ici_fast,
        mouse_xy: Default::default(),
    });
    run(
        SCREEN_WIDTH as usize,
        SCREEN_HEIGHT as usize,
        "Testing",
        system,
        Options::default(),
    )?;
    Ok(())
}

const KEYS: [KeyCode; 4] = [
    KeyCode::ArrowLeft,
    KeyCode::ArrowRight,
    KeyCode::Space,
    KeyCode::Escape,
];

impl System for Example {
    fn keys_used(&self) -> &[KeyCode] {
        &KEYS
    }

    fn update(&mut self, timing: &Timing) {
        self.fast.update(timing.delta as f32);
        self.slow.update(timing.delta as f32);
        self.ici_slow.update(timing.fixed_time_step);
        self.ici_fast.update(timing.fixed_time_step);
    }

    fn on_mouse_move(&mut self, mouse: &MouseData) {
        self.mouse_xy = mouse.xy;
    }

    fn render(&mut self, graphics: &mut Graphics) {
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
            30 => test_30(graphics, self.fast.value_int()),
            31 => test_31(graphics, self.fast.value_int()),
            32 => test_32(graphics, self.slow.value_int()),
            33 => test_33(graphics),
            34 => test_34(graphics),
            35 => test_35(graphics),
            36 => test_36(graphics),
            37 => test_37(graphics),
            38 => test_38(graphics),
            39 => test_39(graphics, &self.ici_static),
            40 => test_40(graphics),
            41 => test_41(graphics, self.mouse_xy),
            42 => test_42(graphics),
            43 => test_43(graphics, &self.image),
            44 => test_44(graphics, &self.image),
            45 => test_alpha(graphics, PixelFont::Standard4x4, "45) Standard 4x4", 4, 4),
            46 => test_alpha(graphics, PixelFont::Standard4x5, "46) Standard 4x5", 4, 5),
            47 => test_alpha(graphics, PixelFont::Standard6x7, "47) Standard 6x7", 6, 7),
            48 => test_alpha(graphics, PixelFont::Standard8x10, "48) Standard 8x10", 8, 10),
            49 => test_alpha(graphics, PixelFont::Outline7x9, "49) Outline 7x9", 7, 9),
            50 => test_alpha(graphics, PixelFont::Script8x8, "50) Script 8x8", 8, 8),
            51 => test_font(graphics, PixelFont::Standard4x4, "51) Standard 4x4"),
            52 => test_font(graphics, PixelFont::Standard4x5, "52) Standard 4x5"),
            53 => test_font(graphics, PixelFont::Standard6x7, "53) Standard 6x7"),
            54 => test_font(graphics, PixelFont::Standard8x10, "54) Standard 8x10"),
            55 => test_font(graphics, PixelFont::Outline7x9, "55) Outline 7x9"),
            56 => test_font(graphics, PixelFont::Script8x8, "56) Script 8x8"),
            _ => graphics.draw_text(
                &format!("Unknown test: {}", self.current_test),
                CENTER.textpos(),
                TextFormat::from((RED, PixelFont::Standard6x7, Positioning::Center)),
            ),
        }
    }

    fn on_key_up(&mut self, keys: Vec<KeyCode>) {
        if keys.contains(&KeyCode::ArrowRight) {
            self.current_test += 1;
        } else if keys.contains(&KeyCode::ArrowLeft) {
            if self.current_test > 0 {
                self.current_test -= 1;
            }
        } else if keys.contains(&KeyCode::Space) {
            self.current_test = 56;
        } else if keys.contains(&KeyCode::Escape) {
            self.should_quit = true;
        }
    }

    fn should_exit(&mut self) -> bool {
        self.should_quit
    }
}

const SCREEN_WIDTH: isize = 250;
const SCREEN_HEIGHT: isize = 250;
const HALF_WIDTH: isize = SCREEN_WIDTH / 2;
const HALF_HEIGHT: isize = SCREEN_HEIGHT / 2;
const TOP_LEFT: Coord = Coord::new(0, 0);
const TOP_RIGHT: Coord = Coord::new(SCREEN_WIDTH, 0);
const BOTTOM_LEFT: Coord = Coord::new(0, SCREEN_HEIGHT);
const BOTTOM_RIGHT: Coord = Coord::new(SCREEN_WIDTH, SCREEN_HEIGHT);
const CENTER: Coord = Coord::new(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2);
const PADDING: Coord = Coord::new(8, 8);
const QUAD_TL: Coord = Coord::new(SCREEN_WIDTH / 4, SCREEN_HEIGHT / 4);
const QUAD_TR: Coord = Coord::new(SCREEN_WIDTH / 4 * 3, SCREEN_HEIGHT / 4);
const QUAD_BL: Coord = Coord::new(SCREEN_WIDTH / 4, SCREEN_HEIGHT / 4 * 3);
const QUAD_BR: Coord = Coord::new(SCREEN_WIDTH / 4 * 3, SCREEN_HEIGHT / 4 * 3);

fn draw_title(graphics: &mut Graphics, text: &str) {
    graphics.draw_text(
        text,
        TextPos::px((HALF_WIDTH, 2)),
        TextFormat::from((WHITE, PixelFont::Standard6x7, Positioning::CenterTop)),
    );
    graphics.draw_line((0, 11), (SCREEN_WIDTH, 11), WHITE);
    graphics.draw_line((0, 12), (SCREEN_WIDTH, 12), WHITE);
}

fn draw_point<P: Into<Coord>>(graphics: &mut Graphics, pos: P) {
    let pos = pos.into();
    graphics.set_pixel(pos.x, pos.y, RED);
}

fn test_0(graphics: &mut Graphics) {
    draw_title(graphics, "0) Text Positioning");

    graphics.draw_text(
        "Center Bottom",
        QUAD_TL.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::CenterBottom)),
    );
    graphics.draw_text(
        "Center Top",
        QUAD_TL.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::CenterTop)),
    );
    graphics.draw_text(
        "Left Center",
        QUAD_TL.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::LeftCenter)),
    );
    graphics.draw_text(
        "Right Center",
        QUAD_TL.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::RightCenter)),
    );
    draw_point(graphics, QUAD_TL);

    graphics.draw_text(
        "Left Top",
        QUAD_TR.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::LeftTop)),
    );
    graphics.draw_text(
        "Left Bottom",
        QUAD_TR.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::LeftBottom)),
    );
    graphics.draw_text(
        "Right Top",
        QUAD_TR.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::RightTop)),
    );
    graphics.draw_text(
        "Right Bottom",
        QUAD_TR.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::RightBottom)),
    );
    draw_point(graphics, QUAD_TR);

    graphics.draw_text(
        "Center",
        QUAD_BL.textpos(),
        TextFormat::from((WHITE, PixelFont::Standard4x5, Positioning::Center)),
    );
    draw_point(graphics, QUAD_BL);
}

fn test_1(graphics: &mut Graphics) {
    draw_title(graphics, "1) Para Sizing and Positioning (1/2)");

    graphics.draw_text(
        "Lorem ipsum\nsample text\nfor sizing\nand positioning",
        QUAD_TL.textpos(),
        TextFormat::from((
            WHITE,
            PixelFont::Standard4x5,
            WrappingStrategy::AtCol(8),
            Positioning::Center,
        )),
    );
    graphics.draw_text(
        "Lorem ipsum\nsample text\nfor sizing\nand positioning",
        QUAD_TR.textpos(),
        TextFormat::from((
            WHITE,
            PixelFont::Standard4x5,
            WrappingStrategy::Cutoff(10),
            Positioning::Center,
        )),
    );
    graphics.draw_text(
        "Lorem ipsum\nsample text\nfor sizing\nand positioning",
        QUAD_BL.textpos(),
        TextFormat::from((
            WHITE,
            PixelFont::Standard4x5,
            WrappingStrategy::SpaceBeforeCol(12),
            Positioning::Center,
        )),
    );
    draw_point(graphics, QUAD_TL);
    draw_point(graphics, QUAD_TR);
    draw_point(graphics, QUAD_BL);
}

fn test_2(graphics: &mut Graphics) {
    draw_title(graphics, "2) Para Sizing and Positioning (2/2)");

    graphics.draw_text(
        "Lorem ipsum\nsample text\nfor sizing\nand positioning",
        QUAD_TL.textpos(),
        TextFormat::from((
            WHITE,
            PixelFont::Standard4x5,
            WrappingStrategy::AtCol(8),
            Positioning::LeftTop,
        )),
    );
    graphics.draw_text(
        "Lorem ipsum\nsample text\nfor sizing\nand positioning",
        QUAD_TR.textpos(),
        TextFormat::from((
            WHITE,
            PixelFont::Standard4x5,
            WrappingStrategy::Cutoff(10),
            Positioning::RightCenter,
        )),
    );
    graphics.draw_text(
        "Lorem ipsum\nsample text\nfor sizing\nand positioning",
        QUAD_BL.textpos(),
        TextFormat::from((
            WHITE,
            PixelFont::Standard4x5,
            WrappingStrategy::SpaceBeforeCol(12),
            Positioning::CenterBottom,
        )),
    );
    draw_point(graphics, QUAD_TL);
    draw_point(graphics, QUAD_TR);
    draw_point(graphics, QUAD_BL);
}

fn test_3(graphics: &mut Graphics) {
    draw_title(graphics, "3) Right Angle Triangles");

    graphics.draw(&Drawable::from_obj(
        Triangle::right_angle(CENTER, 100, AnglePosition::TopLeft),
        stroke(BLUE),
    ));
    graphics.draw(&Drawable::from_obj(
        Triangle::right_angle(CENTER, 100, AnglePosition::TopRight),
        stroke(YELLOW),
    ));
    graphics.draw(&Drawable::from_obj(
        Triangle::right_angle(CENTER, 100, AnglePosition::BottomLeft),
        stroke(GREEN),
    ));
    graphics.draw(&Drawable::from_obj(
        Triangle::right_angle(CENTER, 100, AnglePosition::BottomRight),
        stroke(MAGENTA),
    ));
}

fn test_4(graphics: &mut Graphics) {
    draw_title(graphics, "4) Basic shapes");

    graphics.draw(&Drawable::from_obj(
        Rect::new(QUAD_TL - PADDING, QUAD_TL + PADDING),
        stroke(BLUE),
    ));
    graphics.draw(&Drawable::from_obj(
        Circle::new(QUAD_TR, PADDING.x as usize),
        stroke(BLUE),
    ));
    // graphics.draw(&Drawable::from_obj(Ellipse::new(QUAD_BL, PADDING.x as usize* 2, PADDING.x as usize), stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(
        Triangle::new(
            QUAD_BR - PADDING,
            QUAD_BR + (PADDING.x, 0),
            QUAD_BR + (0, PADDING.x),
        ),
        stroke(BLUE),
    ));
}

fn test_5(graphics: &mut Graphics) {
    draw_title(graphics, "5) Text Symbols");

    graphics.draw_text(
        "Size: NORMAL",
        TextPos::cr((1, 2)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "Letters:",
        TextPos::cr((1, 3)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "ABCDEFGHIJKLMNOPQRSTVWXYZ",
        TextPos::cr((1, 4)),
        TextFormat::from((WHITE, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "Numbers:",
        TextPos::cr((1, 5)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "0123456789",
        TextPos::cr((1, 6)),
        TextFormat::from((WHITE, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "Symbols:",
        TextPos::cr((1, 7)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "!@$%^&*(),./;'\\[]<>?:\"{}_+`~#",
        TextPos::cr((1, 8)),
        TextFormat::from((WHITE, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "Custom:",
        TextPos::cr((1, 9)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "…¤£¥¢✓",
        TextPos::cr((1, 10)),
        TextFormat::from((WHITE, PixelFont::Standard6x7)),
    );

    graphics.draw_text(
        "Size: SMALL",
        TextPos::cr((1, 17)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "Letters:",
        TextPos::cr((1, 18)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "ABCDEFGHIJKLMNOPQRSTVWXYZ",
        TextPos::cr((1, 19)),
        TextFormat::from((WHITE, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "Numbers:",
        TextPos::cr((1, 20)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "0123456789",
        TextPos::cr((1, 21)),
        TextFormat::from((WHITE, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "Symbols:",
        TextPos::cr((1, 22)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "!@$%^&*(),./;'\\[]<>?:\"{}_+`~#",
        TextPos::cr((1, 23)),
        TextFormat::from((WHITE, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "Custom:",
        TextPos::cr((1, 24)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard4x5)),
    );
    graphics.draw_text(
        "…¤£¥¢✓",
        TextPos::cr((1, 25)),
        TextFormat::from((WHITE, PixelFont::Standard4x5)),
    );

    graphics.draw_text(
        "0\n1\n2\n3\n4\n5\n6\n7\n8\n9",
        TextPos::cr((30, 14)),
        TextFormat::from((WHITE, PixelFont::Standard6x7)),
    );
    graphics.draw_text(
        "0\n1\n2\n3\n4\n5\n6\n7\n8\n9",
        TextPos::cr((40, 21)),
        TextFormat::from((WHITE, PixelFont::Standard4x5)),
    );
}

fn test_6(graphics: &mut Graphics) {
    draw_title(graphics, "6) Draw offset");

    let drawable = Drawable::from_obj(Rect::new((100, 100), (120, 120)), fill(BLUE));
    graphics.draw(&drawable);
    graphics.draw_offset((20, 20), &drawable);
    graphics.draw_offset((-20, -20), &drawable);
}

fn test_7(graphics: &mut Graphics) {
    draw_title(graphics, "7) Drawable mutation");

    let drawable = Drawable::from_obj(Rect::new((0, 0), (20, 20)).as_polygon(), fill(BLUE));
    let red = drawable.with_draw_type(stroke(RED));
    let rotated = drawable.with_rotation(45);
    let larger = drawable.with_scale(1.2);
    let smaller = drawable.with_scale(0.8);
    graphics.draw_offset((30, 30), &drawable);
    graphics.draw_offset((100, 30), &red);
    graphics.draw_offset((100, 60), &rotated);
    graphics.draw_offset((100, 90), &smaller);
    graphics.draw_offset((100, 120), &larger);
    graphics.draw(&Drawable::from_obj(
        Rect::new((128, 118), (152, 142)).as_polygon(),
        fill(BLUE),
    ));
}

fn test_8(graphics: &mut Graphics) {
    draw_title(graphics, "8) Polygons");
    let poly1 = Drawable::from_obj(
        Polygon::new(&[(30, 30), (40, 29), (50, 50), (40, 60)]),
        stroke(BLUE),
    );
    graphics.draw(&poly1);
    graphics.draw_offset((0, 60), &poly1.with_draw_type(fill(YELLOW)));
    graphics.draw_offset(
        (60, 60),
        &poly1.with_draw_type(fill(YELLOW)).with_rotation(45),
    );
    graphics.draw_offset(
        (120, 60),
        &poly1.with_draw_type(fill(YELLOW)).with_rotation(80),
    );
    graphics.draw_offset(
        (180, 60),
        &poly1.with_draw_type(fill(YELLOW)).with_rotation(160),
    );
    graphics.draw_offset(
        (00, 120),
        &poly1.with_draw_type(fill(MAGENTA)).with_scale(1.5),
    );
}

fn test_9(graphics: &mut Graphics) {
    draw_title(graphics, "9) Polygon mutation");

    let neg_drawable = Drawable::from_obj(Rect::new((0, 0), (20, 20)).as_polygon(), fill(BLUE));
    let neg_scaled = neg_drawable.with_scale(1.2);

    graphics.draw_offset(QUAD_BL, &neg_drawable);
    graphics.draw_offset(QUAD_BL + (40, 0), &neg_scaled);

    let drawable = Drawable::from_obj(Rect::new((10, 10), (30, 30)).as_polygon(), fill(BLUE));
    let scaled = drawable.with_scale(1.2);

    graphics.draw_offset(QUAD_TL, &drawable);
    graphics.draw_offset(QUAD_TL + (40, 0), &scaled);
}

fn test_10(graphics: &mut Graphics) {
    draw_title(graphics, "10) Off screen squares");

    graphics.draw(&Drawable::from_obj(
        Rect::new(TOP_LEFT - (10, 10), TOP_LEFT + (10, 10)),
        fill(BLUE),
    ));
    graphics.draw(&Drawable::from_obj(
        Rect::new(BOTTOM_RIGHT - (10, 10), BOTTOM_RIGHT + (10, 10)),
        fill(BLUE),
    ));

    graphics.draw_offset(
        (50, 50),
        &Drawable::from_obj(
            Rect::new(TOP_LEFT - (10, 10), TOP_LEFT + (10, 10)),
            fill(BLUE),
        ),
    );
    graphics.draw_offset(
        (-50, -50),
        &Drawable::from_obj(
            Rect::new(BOTTOM_RIGHT - (10, 10), BOTTOM_RIGHT + (10, 10)),
            fill(BLUE),
        ),
    );
}

fn test_11(graphics: &mut Graphics) {
    draw_title(graphics, "11) Off screen circles");

    graphics.draw(&Drawable::from_obj(Circle::new(TOP_LEFT, 10), fill(BLUE)));
    graphics.draw(&Drawable::from_obj(
        Circle::new(BOTTOM_RIGHT, 10),
        fill(BLUE),
    ));

    graphics.draw_offset(
        (50, 50),
        &Drawable::from_obj(Circle::new(TOP_LEFT, 10), fill(BLUE)),
    );
    graphics.draw_offset(
        (-50, -50),
        &Drawable::from_obj(Circle::new(BOTTOM_RIGHT, 10), fill(BLUE)),
    );
}

fn test_12(graphics: &mut Graphics) {
    draw_title(graphics, "12) Off screen polygons");

    graphics.draw(&Drawable::from_obj(
        Polygon::new(&[
            TOP_LEFT - (10, 10),
            TOP_LEFT + (10, -10),
            TOP_LEFT + (10, 10),
            TOP_LEFT + (-10, 10),
        ]),
        fill(BLUE),
    ));
    graphics.draw(&Drawable::from_obj(
        Polygon::new(&[
            BOTTOM_RIGHT - (10, 10),
            BOTTOM_RIGHT + (10, -10),
            BOTTOM_RIGHT + (10, 10),
            BOTTOM_RIGHT + (-10, 10),
        ]),
        fill(BLUE),
    ));

    let left_poly = Drawable::from_obj(
        Polygon::new(&[(-10, 50), (20, 50), (20, 70), (-10, 70)]),
        fill(BLUE),
    );
    graphics.draw(&left_poly);
    graphics.draw_offset((50, 0), &left_poly);

    let top_poly = Drawable::from_obj(
        Polygon::new(&[(200, -10), (220, -10), (220, 30), (200, 30)]),
        fill(BLUE),
    );
    graphics.draw(&top_poly);
    graphics.draw_offset((0, 70), &top_poly);

    let right_poly = Drawable::from_obj(
        Polygon::new(&[(230, 130), (260, 130), (260, 150), (230, 150)]),
        fill(BLUE),
    );
    graphics.draw(&right_poly);
    graphics.draw_offset((-50, 0), &right_poly);

    let bottom_poly = Drawable::from_obj(
        Polygon::new(&[(100, 230), (120, 230), (120, 260), (100, 260)]),
        fill(BLUE),
    );
    graphics.draw(&bottom_poly);
    graphics.draw_offset((0, -70), &bottom_poly);
}

fn test_13(graphics: &mut Graphics) {
    draw_title(graphics, "13) Triangles");

    graphics.draw(&Drawable::from_obj(
        Triangle::equilateral(QUAD_TL, 20, FlatSide::Left),
        fill(MAGENTA),
    ));
    graphics.draw(&Drawable::from_obj(
        Triangle::equilateral(QUAD_TR, 20, FlatSide::Bottom),
        fill(MAGENTA),
    ));
    graphics.draw(&Drawable::from_obj(
        Triangle::equilateral(QUAD_BR, 20, FlatSide::Right),
        fill(MAGENTA),
    ));
    graphics.draw(&Drawable::from_obj(
        Triangle::equilateral(QUAD_BL, 20, FlatSide::Top),
        fill(MAGENTA),
    ));
}

fn test_14(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "14) Poly Rotation - Stroke");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let rect = Rect::new(CENTER - (20, 20), CENTER + (20, 20)).as_polygon();
    let drawable = Drawable::from_obj(rect, stroke(MAGENTA));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_15(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "15) Poly Rotation - Filled");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let rect = Rect::new(CENTER - (20, 20), CENTER + (20, 20)).as_polygon();
    let drawable = Drawable::from_obj(rect, fill(RED));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_16(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "16) Triangle Rotation - Stroke");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let triangle = Triangle::equilateral(CENTER, 40, FlatSide::Bottom);
    let drawable = Drawable::from_obj(triangle, stroke(MAGENTA));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_17(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "17) Triangle Rotation - Filled");

    graphics.draw_circle(Circle::new(CENTER, 27), stroke(BLUE));
    graphics.draw_circle(Circle::new(CENTER, 20), stroke(BLUE));

    let triangle = Triangle::equilateral(CENTER, 40, FlatSide::Bottom);
    let drawable = Drawable::from_obj(triangle, fill(RED));
    graphics.draw(&drawable.with_rotation(degrees));
}

fn test_18(graphics: &mut Graphics) {
    draw_title(graphics, "18) Line Rotation");

    graphics.draw_line((60, 50), (60, 150), YELLOW);
    graphics.draw_line((160, 50), (160, 150), YELLOW);

    let line1 = Line::new((60, 50), (60, 150)).rotate(47);
    let line2 = Line::new((160, 50), (160, 150)).rotate_around(47, coord!(160, 150));

    graphics.draw(&Drawable::from_obj(line1, stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(line2, stroke(BLUE)));
}

fn test_19(graphics: &mut Graphics) {
    draw_title(graphics, "19) Moving shapes");

    let triangle = Triangle::equilateral((40, 40), 10, FlatSide::Left);
    let moved = triangle.move_to(coord!(60, 40));
    let translated = triangle.translate_by(coord!(0, 20));

    graphics.draw_triangle(triangle, fill(BLUE));
    graphics.draw_triangle(moved, fill(YELLOW));
    graphics.draw_triangle(translated, fill(RED));

    let rect = Rect::new((140, 30), (170, 50));
    let moved = rect.move_to(coord!(180, 30));
    let translated = rect.translate_by(coord!(0, 30));

    graphics.draw_rect(rect, fill(BLUE));
    graphics.draw_rect(moved, fill(YELLOW));
    graphics.draw_rect(translated, fill(RED));

    let polygon = Polygon::new(&[(40, 120), (60, 120), (55, 130), (30, 150)]);
    let moved = polygon.move_to(coord!(100, 120));
    let translated = polygon.translate_by(coord!(0, 30));

    graphics.draw_polygon(polygon, fill(BLUE));
    graphics.draw_polygon(moved, fill(YELLOW));
    graphics.draw_polygon(translated, fill(RED));
}

fn test_20(graphics: &mut Graphics) {
    draw_title(graphics, "20) Basic polyline");

    graphics.draw(&Polyline::rounded_rect(150, 40, 220, 120, 20, BLUE).unwrap());
    graphics.draw(&Polyline::rounded_rect(180, 70, 200, 90, 4, YELLOW).unwrap());
}

fn test_21(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "21) Arcs");

    graphics.draw_arc(QUAD_TL, 0, 90, 20, false, RED);

    graphics.draw_arc(QUAD_TR, degrees, degrees + 30, 40, false, BLUE);
    graphics.draw_arc(QUAD_TR, degrees + 10, degrees + 40, 39, false, GREEN);
    graphics.draw_arc(QUAD_TR, degrees + 20, degrees + 50, 38, false, RED);

    graphics.draw_arc(QUAD_BL, 0, 300, 4, false, YELLOW);
    graphics.draw_arc(QUAD_BL, 0, 300, 4, false, ORANGE);

    graphics.draw_arc(QUAD_BR, 0, 90, 20, true, MAGENTA);
}

fn test_22(graphics: &mut Graphics) {
    draw_title(graphics, "22) Colors");

    let colors = &[
        WHITE,
        LIGHT_GRAY,
        RED,
        DARK_GRAY,
        GREEN,
        BLUE,
        YELLOW,
        MAGENTA,
        PURPLE,
        ORANGE,
        CYAN,
        BROWN,
        DARKER_GRAY,
        MID_GRAY,
        LIGHTER_GRAY,
        GB_0,
        GB_1,
        GB_2,
        GB_3,
        OFF_BLACK,
        OFF_WHITE,
    ];
    let names = &[
        "WHITE",
        "LIGHT GRAY",
        "RED",
        "DARK GRAY",
        "GREEN",
        "BLUE",
        "YELLOW",
        "MAGENTA",
        "PURPLE",
        "ORANGE",
        "CYAN",
        "BROWN",
        "DARKER GRAY",
        "MID GRAY",
        "LIGHTER GRAY",
        "GB 0",
        "GB 1",
        "GB 2",
        "GB 3",
        "OFF BLACK",
        "OFF WHITE",
    ];

    let start = Coord::new(70, 30);
    let mut row = 0;
    let mut col = 0;
    let row_space = 120;
    let col_space = 20;
    for (i, color) in colors.iter().enumerate() {
        let coord = Coord::from((row * row_space, col * col_space));
        graphics.draw_text(
            &format!("{}", names[i]),
            TextPos::px(coord + start),
            (*color, PixelFont::Standard8x10, Positioning::Center),
        );
        row += 1;
        if row > 1 {
            row = 0;
            col += 1;
        }
    }
}

fn test_23(graphics: &mut Graphics) {
    draw_title(graphics, "23) Collections");

    let mut collection = ShapeCollection::default();
    InsertShape::insert_above(
        &mut collection,
        Rect::new((150, 150), (170, 190)).as_polygon(),
        stroke(BLUE),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((190, 150), (210, 190)).as_polygon(),
        fill(BLUE),
    );

    graphics.draw(&collection);

    graphics.draw(&collection.with_move((20, 20)).with_draw_type(fill(YELLOW)));

    graphics.draw(
        &collection
            .with_translation((-80, 00))
            .with_draw_type(fill(PURPLE)),
    );

    graphics.draw(
        &collection
            .with_move((190, 20))
            .with_draw_type(fill(MAGENTA))
            .with_scale(0.6),
    );
}

fn test_24(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "24) Rotating collections");

    let mut collection = ShapeCollection::default();
    InsertShape::insert_above(
        &mut collection,
        Rect::new((100, 0), (120, 30)).as_polygon(),
        stroke(BLUE),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((130, 0), (150, 30)).as_polygon(),
        fill(BLUE),
    );

    graphics.draw(&collection.with_rotation_around(degrees, (0, 0)));

    let mut collection = ShapeCollection::default();
    InsertShape::insert_above(
        &mut collection,
        Rect::new((30, 30), (50, 60)).as_polygon(),
        stroke(YELLOW),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((60, 60), (80, 80)).as_polygon(),
        fill(YELLOW),
    );

    graphics.draw(&collection.with_rotation_around(degrees, (0, 0)));

    let mut collection = ShapeCollection::default();
    InsertShape::insert_above(
        &mut collection,
        Rect::new((150, 150), (170, 170)).as_polygon(),
        stroke(MAGENTA),
    );
    InsertShape::insert_above(
        &mut collection,
        Rect::new((170, 170), (190, 190)).as_polygon(),
        fill(MAGENTA),
    );

    graphics.draw(&collection.with_rotation(degrees));
}

fn test_25(graphics: &mut Graphics) {
    draw_title(graphics, "25) Text bounds");

    let short = "one line";
    let long = "multiple lines of text";

    let bounds_short_normal = PixelFont::Standard6x7.measure(short);
    let bounds_short_large = PixelFont::Standard8x10.measure(short);
    let bounds_multi_normal = PixelFont::Standard6x7.measure(&WrappingStrategy::AtCol(6).wrap(long).join("\n"));
    let bounds_multi_large = PixelFont::Standard8x10.measure(&WrappingStrategy::AtCol(6).wrap(long).join("\n"));

    graphics.draw_rect(
        Rect::new((0, 0), bounds_short_normal).move_center_to(QUAD_TL),
        stroke(BLUE),
    );
    graphics.draw_rect(
        Rect::new((0, 0), bounds_short_large).move_center_to(QUAD_TR),
        stroke(BLUE),
    );
    graphics.draw_rect(
        Rect::new((0, 0), bounds_multi_normal).move_center_to(QUAD_BL),
        stroke(BLUE),
    );
    graphics.draw_rect(
        Rect::new((0, 0), bounds_multi_large).move_center_to(QUAD_BR),
        stroke(BLUE),
    );

    graphics.draw_text(
        short,
        TextPos::px(QUAD_TL),
        (
            WHITE,
            PixelFont::Standard6x7,
            WrappingStrategy::None,
            Positioning::Center,
        ),
    );
    graphics.draw_text(
        short,
        TextPos::px(QUAD_TR),
        (
            WHITE,
            PixelFont::Standard8x10,
            WrappingStrategy::None,
            Positioning::Center,
        ),
    );
    graphics.draw_text(
        long,
        TextPos::px(QUAD_BL),
        (
            WHITE,
            PixelFont::Standard6x7,
            WrappingStrategy::AtCol(6),
            Positioning::Center,
        ),
    );
    graphics.draw_text(
        long,
        TextPos::px(QUAD_BR),
        (
            WHITE,
            PixelFont::Standard8x10,
            WrappingStrategy::AtCol(6),
            Positioning::Center,
        ),
    );
}

fn test_26(
    graphics: &mut Graphics,
    image: &IndexedImage,
    slow: &AnimatedIndexedImage,
    fast: &AnimatedIndexedImage,
) {
    draw_title(graphics, "26) Indexed Images");

    graphics.draw_indexed_image((30, 30), image);
    graphics.draw_animated_image((130, 30), slow);
    graphics.draw_animated_image((130, 50), fast);
}

fn test_27(graphics: &mut Graphics) {
    draw_title(graphics, "27) Color brightness");

    let color = Color::new(124, 67, 43, 255);

    let brighter = color.lighten();
    let brighter2 = brighter.lighten();
    let brighter3 = brighter2.lighten();
    let darker = color.darken();
    let darker2 = darker.darken();
    let darker3 = darker2.darken();

    let rect = Drawable::from_obj(Rect::new((0, 0), (30, 30)), DrawType::Fill(WHITE));
    rect.with_move((10, 100))
        .with_draw_type(fill(darker3))
        .render(graphics);
    rect.with_move((40, 100))
        .with_draw_type(fill(darker2))
        .render(graphics);
    rect.with_move((70, 100))
        .with_draw_type(fill(darker))
        .render(graphics);
    rect.with_move((100, 100))
        .with_draw_type(fill(color))
        .render(graphics);
    rect.with_move((130, 100))
        .with_draw_type(fill(brighter))
        .render(graphics);
    rect.with_move((160, 100))
        .with_draw_type(fill(brighter2))
        .render(graphics);
    rect.with_move((190, 100))
        .with_draw_type(fill(brighter3))
        .render(graphics);
}

fn test_28(graphics: &mut Graphics) {
    draw_title(graphics, "28) Color saturation");

    let color = Color::new(124, 197, 93, 255);

    let brighter = color.saturate();
    let brighter2 = brighter.saturate();
    let brighter3 = brighter2.saturate();
    let darker = color.desaturate();
    let darker2 = darker.desaturate();
    let darker3 = darker2.desaturate();

    let rect = Drawable::from_obj(Rect::new((0, 0), (30, 30)), DrawType::Fill(WHITE));
    rect.with_move((10, 100))
        .with_draw_type(fill(darker3))
        .render(graphics);
    rect.with_move((40, 100))
        .with_draw_type(fill(darker2))
        .render(graphics);
    rect.with_move((70, 100))
        .with_draw_type(fill(darker))
        .render(graphics);
    rect.with_move((100, 100))
        .with_draw_type(fill(color))
        .render(graphics);
    rect.with_move((130, 100))
        .with_draw_type(fill(brighter))
        .render(graphics);
    rect.with_move((160, 100))
        .with_draw_type(fill(brighter2))
        .render(graphics);
    rect.with_move((190, 100))
        .with_draw_type(fill(brighter3))
        .render(graphics);
}

fn test_29(graphics: &mut Graphics, image: &IndexedImage) {
    draw_title(graphics, "29) Changing images");

    let mut orig = image.clone();
    let mut palette = orig.get_palette().to_vec();
    palette.push(Color::new(125, 16, 150, 255));
    orig.set_palette(&palette).unwrap();
    orig.set_pixel(13, (palette.len() - 1) as u8).unwrap();
    let darker = orig.with_brightness(0.6);
    let sated = orig.with_saturate(-0.2);

    graphics.draw_indexed_image((100, 100), &orig);
    graphics.draw_indexed_image((50, 100), &darker);
    graphics.draw_indexed_image((150, 100), &sated);
}

fn test_30(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "30) Shape Rotation (stroke)");

    let draw_type = stroke(WHITE);

    let line = Drawable::from_obj(Line::new((20, 40), (40, 40)), draw_type);
    let rect = Drawable::from_obj(Rect::new((60, 20), (90, 60)), draw_type);
    let triangle = Drawable::from_obj(Triangle::new((120, 20), (170, 20), (145, 70)), draw_type);
    let circle = Drawable::from_obj(Circle::new((40, 100), 20), draw_type);
    // let ellipse = Drawable::from_obj(Ellipse::new((100, 100), 20, 30), draw_type);
    let polygon = Drawable::from_obj(
        Polygon::new(&[
            (150, 100),
            (170, 100),
            (155, 120),
            (180, 102),
            (150, 180),
            (120, 110),
        ]),
        draw_type,
    );

    graphics.draw(&line.with_rotation(degrees));
    graphics.draw(&rect.with_rotation(degrees));
    graphics.draw(&triangle.with_rotation(degrees));
    graphics.draw(&circle.with_rotation(degrees));
    // graphics.draw(&ellipse.with_rotation(degrees));
    graphics.draw(&polygon.with_rotation(degrees));
}

fn test_31(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "31) Shape Rotation (fill)");

    let draw_type = fill(WHITE);

    let line = Drawable::from_obj(Line::new((20, 40), (40, 40)), draw_type);
    let rect = Drawable::from_obj(Rect::new((60, 20), (90, 60)), draw_type);
    let triangle = Drawable::from_obj(Triangle::new((120, 20), (170, 20), (145, 70)), draw_type);
    let circle = Drawable::from_obj(Circle::new((40, 100), 20), draw_type);
    // let ellipse = Drawable::from_obj(Ellipse::new((100, 100), 20, 30), draw_type);
    let polygon = Drawable::from_obj(
        Polygon::new(&[
            (150, 100),
            (170, 100),
            (155, 120),
            (180, 102),
            (150, 180),
            (120, 110),
        ]),
        draw_type,
    );

    graphics.draw(&line.with_rotation(degrees));
    graphics.draw(&rect.with_rotation(degrees));
    graphics.draw(&triangle.with_rotation(degrees));
    graphics.draw(&circle.with_rotation(degrees));
    // graphics.draw(&ellipse.with_rotation(degrees));
    graphics.draw(&polygon.with_rotation(degrees));
}

fn test_32(graphics: &mut Graphics, degrees: isize) {
    draw_title(graphics, "32) Shape Rotation (contains)");

    let draw_type = stroke(WHITE);
    let contains = stroke(GREEN);

    let line = Line::new((20, 40), (40, 40)).rotate(degrees);
    let rect = Rect::new((60, 20), (90, 60)).rotate(degrees);
    let triangle = Triangle::new((120, 20), (170, 20), (145, 70)).rotate(degrees);
    let circle = Circle::new((40, 100), 20).rotate(degrees);
    // let ellipse =Ellipse::new((100, 100), 20, 30).rotate(degrees);
    let polygon = Polygon::new(&[
        (150, 100),
        (170, 100),
        (155, 120),
        (180, 102),
        (150, 180),
        (120, 110),
    ])
        .rotate(degrees);

    let line_point = Coord::new(30, 30);
    let rect_point = Coord::new(71, 21);
    let triangle_point = Coord::new(146, 19);
    let circle_point = Coord::new(42, 102);
    let ellipse_point = Coord::new(70, 70);
    let polygon_point = Coord::new(170, 130);

    let line_draw_type = if line.contains(line_point) {
        contains
    } else {
        draw_type
    };
    let rect_draw_type = if rect.contains(rect_point) {
        contains
    } else {
        draw_type
    };
    let triangle_draw_type = if triangle.contains(triangle_point) {
        contains
    } else {
        draw_type
    };
    let circle_draw_type = if circle.contains(circle_point) {
        contains
    } else {
        draw_type
    };
    // let ellipse_draw_type = if ellipse.contains(ellipse_point) { contains} else {draw_type};
    let polygon_draw_type = if polygon.contains(polygon_point) {
        contains
    } else {
        draw_type
    };

    graphics.draw(&Drawable::from_obj(line, line_draw_type));
    graphics.draw(&Drawable::from_obj(rect.clone(), rect_draw_type));
    graphics.draw(&Drawable::from_obj(triangle, triangle_draw_type));
    graphics.draw(&Drawable::from_obj(circle, circle_draw_type));
    // graphics.draw(&Drawable::from_obj(ellipse, ellipse_draw_type));
    graphics.draw(&Drawable::from_obj(polygon, polygon_draw_type));

    graphics.set_pixel(line_point.x, line_point.y, RED);
    graphics.set_pixel(rect_point.x, rect_point.y, RED);
    graphics.set_pixel(triangle_point.x, triangle_point.y, RED);
    graphics.set_pixel(circle_point.x, circle_point.y, RED);
    graphics.set_pixel(ellipse_point.x, ellipse_point.y, RED);
    graphics.set_pixel(polygon_point.x, polygon_point.y, RED);
}

fn test_33(graphics: &mut Graphics) {
    graphics.clear(RED);

    graphics
        .clip_mut()
        .add_rect(Rect::new(TOP_LEFT, BOTTOM_RIGHT));
    graphics
        .clip_mut()
        .remove_rect(Rect::new((70, 70), (120, 120)));
    graphics
        .clip_mut()
        .remove_circle(Circle::new((200, 100), 20));

    graphics.clear_aware(DARK_GRAY);

    let mut image = Image::new_blank(20, 20);
    image.set_pixel(0, 0, CYAN);
    image.set_pixel(19, 0, MAGENTA);
    image.set_pixel(0, 19, GREEN);
    image.set_pixel(19, 19, PURPLE);

    graphics.draw_image((60, 60), &image);

    graphics.draw_image_unchecked((110, 110), &image);

    graphics.clip_mut().set_all_valid();

    graphics.draw_rect(Rect::new((0, 0), (SCREEN_WIDTH, 12)), fill(BLACK));

    draw_title(graphics, "33) Clipping (complex)");
}

fn test_34(graphics: &mut Graphics) {
    draw_title(graphics, "34) Polygons");

    let ellipse1 = Ellipse::new(QUAD_TL, 100, 50);
    let ellipse2 = Ellipse::new(QUAD_TR, 50, 100);
    let ellipse3 = Ellipse::new(QUAD_BL, 50, 100).rotate(45);
    let ellipse4 = Ellipse::new(QUAD_BR, 50, 100).rotate(-45);

    let poly1 = ellipse1.as_polygon();
    let poly2 = ellipse2.as_polygon();
    let poly3 = ellipse3.as_polygon();
    let poly4 = ellipse4.as_polygon();

    graphics.draw(&Drawable::from_obj(ellipse1, stroke(WHITE)));
    graphics.draw(&Drawable::from_obj(ellipse2, stroke(WHITE)));
    graphics.draw(&Drawable::from_obj(ellipse3, stroke(WHITE)));
    graphics.draw(&Drawable::from_obj(ellipse4, stroke(WHITE)));

    graphics.draw(&Drawable::from_obj(poly1, stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(poly2, stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(poly3, stroke(BLUE)));
    graphics.draw(&Drawable::from_obj(poly4, stroke(BLUE)));
}

fn test_35(graphics: &mut Graphics) {
    draw_title(graphics, "35) Image Rotation/Flip");

    let mut image = Image::new_blank(12, 24);
    image.set_pixel(0, 0, BLUE);
    image.set_pixel(1, 1, BLUE);
    image.set_pixel(2, 2, BLUE);
    image.set_pixel(3, 3, BLUE);
    image.set_pixel(11, 22, RED);
    image.set_pixel(11, 23, RED);
    image.set_pixel(10, 23, RED);

    graphics.draw_image_unchecked((100, 50), &image);
    graphics.draw_image_unchecked((130, 50), &image.rotate_cw());
    graphics.draw_image_unchecked((170, 50), &image.rotate_cw().rotate_cw());
    graphics.draw_image_unchecked((70, 50), &image.rotate_ccw());
    graphics.draw_image_unchecked((40, 50), &image.rotate_ccw().rotate_ccw());

    let mut flipped_v = image.clone();
    flipped_v.flip_vertical();
    graphics.draw_image_unchecked((40, 100), &flipped_v);

    let mut flipped_h = image.clone();
    flipped_h.flip_horizontal();
    graphics.draw_image_unchecked((70, 100), &flipped_h);

    let mut image = Image::new_blank(24, 24);
    image.set_pixel(0, 0, BLUE);
    image.set_pixel(1, 1, BLUE);
    image.set_pixel(2, 2, BLUE);
    image.set_pixel(3, 3, BLUE);
    image.set_pixel(23, 23, RED);
    image.set_pixel(22, 23, RED);
    image.set_pixel(23, 22, RED);

    graphics.draw_image_unchecked((100, 180), &image);
    graphics.draw_image_unchecked((130, 180), &image.rotate_cw());
    graphics.draw_image_unchecked((170, 180), &image.rotate_cw().rotate_cw());
    graphics.draw_image_unchecked((70, 180), &image.rotate_ccw());
    graphics.draw_image_unchecked((40, 180), &image.rotate_ccw().rotate_ccw());

    let mut flipped_v = image.clone();
    flipped_v.flip_vertical();
    graphics.draw_image_unchecked((40, 220), &flipped_v);

    let mut flipped_h = image.clone();
    flipped_h.flip_horizontal();
    graphics.draw_image_unchecked((70, 220), &flipped_h);
}

fn test_36(graphics: &mut Graphics) {
    draw_title(graphics, "36) Large Text");

    graphics.draw_text(
        "Size: LARGE",
        TextPos::cr((1, 2)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "Letters:",
        TextPos::cr((1, 3)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "ABCDEFGHIJKL",
        TextPos::cr((1, 4)),
        TextFormat::from((WHITE, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "MNOPQRSTVWXYZ",
        TextPos::cr((1, 5)),
        TextFormat::from((WHITE, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "Numbers:",
        TextPos::cr((1, 6)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "0123456789",
        TextPos::cr((1, 7)),
        TextFormat::from((WHITE, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "Symbols:",
        TextPos::cr((1, 8)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "!@$%^&*(),./;'\\",
        TextPos::cr((1, 9)),
        TextFormat::from((WHITE, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "[]<>?:\"{}_+`~#",
        TextPos::cr((1, 10)),
        TextFormat::from((WHITE, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "Custom:",
        TextPos::cr((1, 11)),
        TextFormat::from((LIGHT_GRAY, PixelFont::Standard8x10)),
    );
    graphics.draw_text(
        "…¤£¥¢✓",
        TextPos::cr((1, 12)),
        TextFormat::from((WHITE, PixelFont::Standard8x10)),
    );

    graphics.draw_text(
        "0\n1\n2\n3\n4\n5\n6\n7\n8\n9",
        TextPos::cr((20, 7)),
        TextFormat::from((WHITE, PixelFont::Standard8x10)),
    );
}

fn test_37(graphics: &mut Graphics) {
    draw_title(graphics, "37) Custom Font");

    graphics.custom_font.insert(
        chr_to_code('b'),
        CustomLetter {
            _4x5: [true; standard_4x5::LETTER_PX_COUNT],
            ..CustomLetter::default()
        },
    );

    graphics.draw_letter((20, 20), 'b', PixelFont::Standard4x5, WHITE);

    graphics.custom_font.clear();
}

fn test_38(graphics: &mut Graphics) {
    draw_title(graphics, "38) Transparency");

    graphics.draw_rect(Rect::new((60, 60), (200, 200)), fill(WHITE));
    graphics.draw_rect(
        Rect::new((30, 30), (80, 80)),
        fill(Color::new(255, 51, 77, 127)),
    );
    graphics.draw_rect(
        Rect::new((100, 30), (160, 120)),
        fill(Color::new(255, 51, 77, 127)),
    );
    graphics.draw_rect(
        Rect::new((100, 50), (160, 140)),
        fill(Color::new(51, 127, 152, 127)),
    );
}

fn test_39(graphics: &mut Graphics, indexed_image: &IndexedImage) {
    draw_title(graphics, "39) IndexedImage -> Image");
    let image = Image::from_indexed(indexed_image).to_renderable((130, 100), DrawOffset::TopLeft);

    graphics.draw_indexed_image((100, 100), &indexed_image);
    image.render(graphics);
}

fn test_40(graphics: &mut Graphics) {
    draw_title(graphics, "40) More Triangles");

    let top = Triangle::right_angle(
        coord!(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2) - (0, 50),
        50,
        AnglePosition::Top,
    );
    let bottom = Triangle::right_angle(
        coord!(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2) + (0, 50),
        50,
        AnglePosition::Bottom,
    );
    let left = Triangle::right_angle(
        coord!(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2) - (50, 0),
        50,
        AnglePosition::Left,
    );
    let right = Triangle::right_angle(
        coord!(SCREEN_WIDTH / 2, SCREEN_HEIGHT / 2) + (50, 0),
        50,
        AnglePosition::Right,
    );

    graphics.draw_triangle(top, stroke(GB_0));
    graphics.draw_triangle(bottom, stroke(GB_1));
    graphics.draw_triangle(left, stroke(GB_2));
    graphics.draw_triangle(right, stroke(GB_3));
}

fn test_41(graphics: &mut Graphics, mouse_xy: Coord) {
    draw_title(graphics, "41) Nearest pixel");

    let line1 = Line::new((30, 20), (60, 100));
    let line_nearest1 = line1.nearest_point(mouse_xy);

    let line2 = Line::new((120, 40), (60, 120));
    let line_nearest2 = line2.nearest_point(mouse_xy);

    graphics.draw_line(line1.start(), line1.end(), MID_GRAY);
    graphics.set_pixel(line_nearest1.x, line_nearest1.y, RED);

    graphics.draw_line(line2.start(), line2.end(), MID_GRAY);
    graphics.set_pixel(line_nearest2.x, line_nearest2.y, RED);
}

fn test_42(graphics: &mut Graphics) {
    draw_title(graphics, "42) Palette simplification");

    let palette = vec![
        Color::new(255, 0, 0, 255),
        Color::new(235, 0, 0, 255),
        Color::new(215, 0, 0, 255),
        Color::new(0, 0, 240, 255),
        Color::new(0, 0, 200, 255),
    ];
    let simp = [
        simplify_palette(&palette, 5),
        simplify_palette(&palette, 20),
        simplify_palette(&palette, 40),
        simplify_palette(&palette, 60),
        simplify_palette(&palette, 100),
        simplify_palette(&palette, 800),
    ];

    let rect = Rect::new((0, 0), (20, 20));

    for (y, row) in simp.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            graphics.draw_rect(rect.move_to(coord!(x * 20, y * 22) + 50), fill(*c));
        }
    }
}

fn test_43(graphics: &mut Graphics, image: &IndexedImage) {
    draw_title(graphics, "43) Indexed flip/rot");

    let vert = image.flip_vertical().unwrap();
    let horz = image.flip_horizontal().unwrap();
    let both = image.flip_horizontal().unwrap().flip_vertical().unwrap();

    graphics.draw_indexed_image(coord!(30, 30), image);
    graphics.draw_indexed_image(coord!(70, 30), &vert);
    graphics.draw_indexed_image(coord!(110, 30), &horz);
    graphics.draw_indexed_image(coord!(150, 30), &both);

    unsafe {
        let vertu = image.flip_vertical_unchecked();
        let horzu = image.flip_horizontal_unchecked();
        let bothu = image.flip_horizontal_unchecked().flip_vertical_unchecked();

        graphics.draw_indexed_image(coord!(70, 65), &vertu);
        graphics.draw_indexed_image(coord!(110, 65), &horzu);
        graphics.draw_indexed_image(coord!(150, 65), &bothu);
    }
}

fn test_44(graphics: &mut Graphics, image: &IndexedImage) {
    draw_title(graphics, "44) Indexed scale");

    let nn = image.scale(Scaling::nn_double()).unwrap();
    let e2 = image.scale(Scaling::Epx2x).unwrap();
    let e4 = image.scale(Scaling::Epx4x).unwrap();

    graphics.draw_indexed_image(coord!(20, 20), image);
    graphics.draw_indexed_image(coord!(55, 20), &nn);
    graphics.draw_indexed_image(coord!(130, 20), &e2);

    unsafe {
        let nnu = image.scale_unchecked(Scaling::nn_double());
        let e2u = image.scale_unchecked(Scaling::Epx2x);
        let e4u = image.scale_unchecked(Scaling::Epx4x);

        graphics.draw_indexed_image(coord!(20, 100), &nnu);
        graphics.draw_indexed_image(coord!(100, 100), &e2u);
    }
}

const CHARS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789!@$%^&*(),./;'\\[]<>?:\"{}_+-=`~#°…¤£¥¢✓€|";

fn test_font(graphics: &mut Graphics, font: PixelFont, name: &str) {
    draw_title(graphics, name);

    graphics.draw_text("The black quartz sphinx offered to buy the toy from the brown fox {european} for £30 (GBP), but fox replied I only accept dollars (USD) and it'll be $54. The sphinx though \"hmm, what about ¥126 or ¤789\". The fox yelled back WHAT! ARE YOU TRYING TO RIP ME OFF BLACK QUARTZ SPHINX; I ONLY ASK FOR A FAIR AMOUNT: ¢99… Sorry, got a bit loud there, let me see `(1 + 2 - 1) * 1 = 2` ~~ ok and if I offer 50% off, & 2^2 is 4 and some slashes /\\ and can't forget the #°✓.\nI think that's everything except my email fox@animals.com [1]", TextPos::Px(8, 14), (WHITE, font, WrappingStrategy::SpaceBeforeCol(font.px_to_cols(SCREEN_WIDTH as usize - 16))));
}

fn test_alpha(graphics: &mut Graphics, font: PixelFont, name: &str, expected_w: usize, expected_h: usize) {
    draw_title(graphics, name);

    if font.size().0 != expected_w || font.size().1 != expected_h {
        graphics.draw_text(&format!("Invalid font {expected_w},{expected_h} != {:?}", font.size()), TextPos::Px(16, 14), (RED, PixelFont::Standard4x4));
    }

    let chars_per_line = (SCREEN_WIDTH as usize - (SCREEN_WIDTH as f32 * 0.04).floor() as usize) / (16 + 1);

    let offset = coord!(SCREEN_WIDTH as f32 *0.02, 20.);

    for (idx, line) in CHARS.chars().collect::<Vec<char>>().chunks(chars_per_line).enumerate() {
        let line_offset = offset + (0, idx * 28);
        for (x, chr) in line.iter().enumerate() {
            let px = line_offset.x + (x * 17) as isize;
            graphics.draw_letter((px, line_offset.y), *chr, PixelFont::Standard8x10, MID_GRAY);
            graphics.draw_letter((px, line_offset.y + 12_isize), *chr, font, WHITE);
        }
    }
}

fn test_sentence(graphics: &mut Graphics, idx: usize, fonts: &[(PixelFont, &str)]) {
    draw_title(graphics, &format!("{idx}) Test Sentences"));

    let offset = coord!(4, 20);

    for (i, (font, name)) in fonts.iter().enumerate() {
        let pos = offset + (0, i * 40);
        graphics.draw_text(&format!("This is the font '{name}'."), TextPos::px(pos), (WHITE, *font));
        graphics.draw_text("The 7 quick brown foxes jump over the 12 lazy dogs!", TextPos::px(pos + (0, 20)), (WHITE, *font, WrappingStrategy::SpaceBeforeCol(font.px_to_cols(SCREEN_WIDTH as usize - 8))));
    }
}