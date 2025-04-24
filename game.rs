#![no_main]
#![no_std]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> ! {
    loop {}
}

// --- Constants ---
const DISPLAY_WIDTH: usize = 800;
const DISPLAY_HEIGHT: usize = 600;
const DISPLAY_BACKGROUND: Pixel = Pixel::rgba(0x1E, 0x1E, 0x2E, 0xFF);

// Player
const PLAYER_SIZE: i32 = 80;
const PLAYER_COLOR: Pixel = Pixel::rgba(0x00, 0xA2, 0xFF, 0xFF);
const PLAYER_KILL_REWARD: usize = 100;
const PLAYER_INITIAL_HEALTH: i32 = 3;

// Bullet
const BULLET_SIZE: i32 = 25;
const BULLET_SPEED: i32 = DISPLAY_HEIGHT as i32 * 2;
const BULLET_COLOR: Pixel = Pixel::rgba(0xFF, 0xC1, 0x00, 0xFF);
const BULLETS_CAPACITY: usize = 5;

// Enemy
const ENEMY_SIZE: i32 = 100;
const ENEMY_COLOR: Pixel = Pixel::rgba(0xFF, 0x4D, 0x6D, 0xFF);
const ENEMY_SPEED: i32 = DISPLAY_HEIGHT as i32 / 2;
const ENEMIES_CAPACITY: usize = 10;
const ENEMY_INITIAL_SPAWN_PERIOD: Seconds = 1.5;
const ENEMY_MIN_SPAWN_PERIOD: Seconds = 0.3;
const ENEMY_SPAWN_PERIOD_SCORE_FACTOR: f32 = 0.001;

// UI & Text
const SCORE_LABEL_COLOR: Pixel = Pixel::rgba(0xDC, 0xDC, 0xCC, 0xFF);
const HEALTH_LABEL_COLOR: Pixel = Pixel::rgba(0xDA, 0x47, 0x50, 0xFF);
const MESSAGE_COLOR: Pixel = Pixel::rgba(0xFF, 0xFF, 0xFF, 0xFF);
const SCORE_LABEL_PADDING: i32 = 17;
const SCORE_LABEL_X: i32 = SCORE_LABEL_PADDING;
const SCORE_LABEL_Y: i32 = SCORE_LABEL_PADDING;
const HEALTH_LABEL_X: i32 = SCORE_LABEL_PADDING;
const HEALTH_LABEL_Y: i32 = SCORE_LABEL_PADDING * 2 + FONT_CHAR_HEIGHT as i32 * 4;
const TEXT_SCALE: i32 = 4;
const MESSAGE_SCALE: i32 = 6;

// Shadow
const SHADOW_COLOR: Pixel = Pixel::rgba(0x2B, 0x2B, 0x2B, 0xFF);
const SHADOW_OFFSET: i32 = 4;

// Font
const FONT_IMAGE_WIDTH: usize = 128;
const FONT_IMAGE_HEIGHT: usize = 64;
const FONT_IMAGE_COLS: usize = 18;
const FONT_IMAGE_ROWS: usize = 7;
const FONT_CHAR_WIDTH: usize = FONT_IMAGE_WIDTH / FONT_IMAGE_COLS;
const FONT_CHAR_HEIGHT: usize = FONT_IMAGE_HEIGHT / FONT_IMAGE_ROWS;
const BITS_IN_BYTE: usize = 8;
const COPYRIGHT_TEXT: &[u8] = b"Made by realsanjeev";
const COPYRIGHT_SCALE: i32 = 2;
const COPYRIGHT_PADDING: usize = 10;

const COMPRESSED_FONT: [u8; 622] = [
    0x00, 0x11, 0x20, 0xa1, 0x41, 0x0c, 0x0e, 0x08, 0x08, 0x40, 0x00, 0x05, 0x38, 0x20, 0x00, 0x01,
    0x20, 0xa1, 0x43, 0xcc, 0x92, 0x08, 0x10, 0x21, 0x50, 0x80, 0x00, 0x02, 0x02, 0x44, 0x60, 0x00,
    0x01, 0x20, 0x03, 0xe5, 0x01, 0x14, 0x00, 0x01, 0x20, 0x10, 0xe0, 0x80, 0x00, 0x02, 0x04, 0x4c,
    0xa0, 0x00, 0x01, 0x20, 0x01, 0x43, 0x82, 0x08, 0x00, 0x01, 0x20, 0x11, 0xf3, 0xe0, 0x0f, 0x80,
    0x08, 0x54, 0x20, 0x00, 0x01, 0x20, 0x03, 0xe1, 0x44, 0x15, 0x00, 0x01, 0x20, 0x10, 0xe0, 0x81,
    0x00, 0x02, 0x10, 0x64, 0x20, 0x00, 0x02, 0x01, 0x47, 0x89, 0x92, 0x00, 0x01, 0x10, 0x21, 0x50,
    0x81, 0x00, 0x02, 0x20, 0x44, 0x20, 0x00, 0x01, 0x20, 0x01, 0x41, 0x01, 0x8d, 0x00, 0x01, 0x08,
    0x40, 0x00, 0x01, 0x02, 0x00, 0x01, 0x04, 0x00, 0x01, 0x38, 0xf8, 0x00, 0x20, 0x38, 0x70, 0x63,
    0xe3, 0x8f, 0x8e, 0x1c, 0x00, 0x04, 0x07, 0x0e, 0x1c, 0x78, 0x70, 0x44, 0x88, 0xa2, 0x04, 0x00,
    0x01, 0x91, 0x22, 0x10, 0x20, 0x20, 0x02, 0x08, 0x91, 0x22, 0x44, 0x88, 0x04, 0x09, 0x22, 0x04,
    0x01, 0x11, 0x22, 0x00, 0x02, 0x43, 0xe1, 0x08, 0x97, 0x22, 0x44, 0x80, 0x08, 0x31, 0xf3, 0xc7,
    0x82, 0x0e, 0x1e, 0x00, 0x02, 0x80, 0x00, 0x01, 0x81, 0x15, 0x3e, 0x78, 0x80, 0x10, 0x08, 0x20,
    0x24, 0x44, 0x11, 0x02, 0x00, 0x01, 0x20, 0x43, 0xe1, 0x02, 0x17, 0x22, 0x44, 0x80, 0x20, 0x88,
    0x20, 0x24, 0x44, 0x11, 0x02, 0x10, 0x20, 0x20, 0x02, 0x00, 0x01, 0x10, 0x22, 0x44, 0x88, 0x7c,
    0x70, 0x23, 0xc3, 0x84, 0x0e, 0x1c, 0x00, 0x01, 0x40, 0x00, 0x02, 0x02, 0x0e, 0x22, 0x78, 0x70,
    0x00, 0x20, 0x78, 0xf9, 0xf1, 0xc4, 0x4f, 0x9f, 0x22, 0x40, 0x89, 0x11, 0xc7, 0x87, 0x1e, 0x1e,
    0x7c, 0x88, 0x44, 0x81, 0x02, 0x24, 0x42, 0x01, 0x22, 0x40, 0xd9, 0x12, 0x24, 0x48, 0x91, 0x20,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x04, 0x42, 0x01, 0x24, 0x40, 0xa9, 0x92, 0x24, 0x48, 0x91, 0x20,
    0x10, 0x88, 0x44, 0xf1, 0xe2, 0x07, 0xc2, 0x01, 0x38, 0x40, 0x89, 0x52, 0x27, 0x88, 0x9e, 0x1c,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x64, 0x42, 0x01, 0x24, 0x40, 0x89, 0x32, 0x24, 0x0a, 0x91, 0x02,
    0x10, 0x88, 0x44, 0x81, 0x02, 0x24, 0x42, 0x11, 0x22, 0x40, 0x89, 0x12, 0x24, 0x09, 0x11, 0x02,
    0x10, 0x88, 0x78, 0xf9, 0x01, 0xc4, 0x4f, 0x8e, 0x22, 0x7c, 0x89, 0x11, 0xc4, 0x06, 0x91, 0x3c,
    0x10, 0x70, 0x00, 0x20, 0x44, 0x89, 0x12, 0x27, 0xc3, 0x00, 0x01, 0x18, 0x10, 0x00, 0x01, 0x80,
    0x04, 0x00, 0x01, 0x01, 0x00, 0x01, 0x18, 0x00, 0x01, 0x44, 0x89, 0x12, 0x20, 0x42, 0x10, 0x08,
    0x28, 0x00, 0x01, 0x40, 0x04, 0x00, 0x01, 0x01, 0x00, 0x01, 0x20, 0x00, 0x01, 0x44, 0x88, 0xa1,
    0x40, 0x82, 0x08, 0x08, 0x00, 0x02, 0x01, 0xc7, 0x87, 0x0f, 0x1c, 0x7c, 0x78, 0x44, 0x88, 0x40,
    0x81, 0x02, 0x04, 0x08, 0x00, 0x03, 0x24, 0x48, 0x91, 0x22, 0x20, 0x88, 0x44, 0xa8, 0xa0, 0x82,
    0x02, 0x02, 0x08, 0x00, 0x02, 0x01, 0xe4, 0x48, 0x11, 0x3e, 0x20, 0x78, 0x28, 0xd9, 0x10, 0x84,
    0x02, 0x01, 0x08, 0x00, 0x02, 0x02, 0x24, 0x48, 0x91, 0x20, 0x20, 0x08, 0x10, 0x89, 0x10, 0x87,
    0xc3, 0x00, 0x01, 0x18, 0x00, 0x01, 0xf8, 0x01, 0xe7, 0x87, 0x0f, 0x1e, 0x20, 0x70, 0x00, 0x20,
    0x40, 0x20, 0x12, 0x04, 0x00, 0x06, 0x02, 0x00, 0x05, 0x40, 0x00, 0x01, 0x02, 0x04, 0x00, 0x06,
    0x02, 0x00, 0x05, 0x78, 0xe0, 0x72, 0x44, 0x0d, 0x1e, 0x1c, 0x78, 0x79, 0x61, 0xe7, 0x88, 0x91,
    0x22, 0x44, 0x88, 0x44, 0x20, 0x13, 0x84, 0x0a, 0x91, 0x22, 0x44, 0x89, 0x92, 0x02, 0x08, 0x91,
    0x22, 0x28, 0x88, 0x44, 0x20, 0x12, 0x44, 0x0a, 0x91, 0x22, 0x78, 0x79, 0x01, 0xc2, 0x08, 0x91,
    0x22, 0x10, 0x78, 0x44, 0x21, 0x12, 0x24, 0x08, 0x91, 0x22, 0x40, 0x09, 0x00, 0x01, 0x22, 0x48,
    0x8a, 0x2a, 0x28, 0x08, 0x44, 0xf8, 0xe2, 0x23, 0x88, 0x91, 0x1c, 0x40, 0x09, 0x03, 0xc1, 0x87,
    0x84, 0x14, 0x44, 0x70, 0x00, 0x21, 0x10, 0x41, 0x00, 0x0e, 0x20, 0x40, 0x80, 0x00, 0x0c, 0x7c,
    0x20, 0x40, 0x82, 0x40, 0x00, 0x0b, 0x08, 0x40, 0x40, 0x45, 0x80, 0x00, 0x0b, 0x10, 0x20, 0x40,
    0x80, 0x00, 0x0c, 0x20, 0x20, 0x40, 0x80, 0x00, 0x0c, 0x7c, 0x10, 0x41, 0x00, 0xbd,
];

// RNG (Random Number Generator)
const RNG_A: i32 = 1103515245;
const RNG_C: i32 = 12345;

struct Rng {
    seed: i32,
}

impl Rng {
    const fn from_seed(seed: i32) -> Self {
        Self { seed }
    }

    fn rand(&mut self) -> i32 {
        self.seed = RNG_A.wrapping_mul(self.seed).wrapping_add(RNG_C);
        self.seed
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct Pixel(u32);

impl Pixel {
    const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self(((a as u32) << (3*8)) |
             ((b as u32) << (2*8)) |
             ((g as u32) << (1*8)) |
             ((r as u32) << (0*8)))
    }
}

#[repr(C)]
pub struct Display {
    pixels: [Pixel; DISPLAY_WIDTH * DISPLAY_HEIGHT],
}

// Utility function
const fn max(x: i32, y: i32) -> i32 { if x > y { x } else { y } }
const fn min(x: i32, y: i32) -> i32 { if x < y { x } else { y } }
const fn clamp(x: i32, low: i32, high: i32) -> i32 { min(max(low, x), high) }


impl Display {
    fn fill(&mut self, pixel: Pixel) {
        for pixel_ref in self.pixels.iter_mut() {
            *pixel_ref = pixel;
        }
    }

    fn fill_rect(&mut self, x0: i32, y0: i32, w: i32, h: i32, pixel: Pixel) {
        let x1 = clamp(x0,         0, (DISPLAY_WIDTH - 1)  as i32) as usize;
        let x2 = clamp(x0 + w - 1, 0, (DISPLAY_WIDTH - 1)  as i32) as usize;
        let y1 = clamp(y0,         0, (DISPLAY_HEIGHT - 1) as i32) as usize;
        let y2 = clamp(y0 + h - 1, 0, (DISPLAY_HEIGHT - 1) as i32) as usize;

        for y in y1..=y2 {
            for x in x1..=x2 {
                if let Some(pixel_ref) = self.pixels.get_mut(y * DISPLAY_WIDTH + x) {
                    *pixel_ref = pixel
                }
            }
        }
    }
}

type Seconds = f32;

#[derive(Clone, Copy)]
#[repr(C)]
struct Entity {
    x: i32,
    y: i32,
    alive: bool,
}

impl Entity {
    const fn new(x: i32, y: i32) -> Self {
        Self { x, y, alive: true }
    }

    const fn dead() -> Self {
        Self {
            x: 0,
            y: 0,
            alive: false,
        }
    }

    fn revive(&mut self, x: i32, y: i32) {
        self.alive = true;
        self.x = x;
        self.y = y;
    }

    fn render(&self, display: &mut Display, size: i32, color: Pixel) {
        if self.alive {
            let x = self.x - size / 2;
            let y = self.y - size / 2;
            display.fill_rect(x + SHADOW_OFFSET, y + SHADOW_OFFSET, size, size, SHADOW_COLOR);
            display.fill_rect(x, y, size, size, color);
        }
    }

    fn overlaps(&self, self_size: i32, that: &Self, that_size: i32) -> bool {
        if !self.alive || !that.alive {
            return false;
        }

        let left1 = self.x - self_size / 2;
        let right1 = self.x + self_size / 2;
        let top1 = self.y - self_size / 2;
        let bottom1 = self.y + self_size / 2;

        let left2 = that.x - that_size / 2;
        let right2 = that.x + that_size / 2;
        let top2 = that.y - that_size / 2;
        let bottom2 = that.y + that_size / 2;

        right1 >= left2 && right2 >= left1 && bottom2 >= top1 && bottom1 >= top2
    }
}

struct Font {
    pixels: [u8; FONT_IMAGE_WIDTH * FONT_IMAGE_HEIGHT],
}

impl Font {
    fn decompress_from_bytes(&mut self, bytes: &[u8]) {
        let n = bytes.len();
        let mut i = 0;
        let mut pixels_size: usize = 0;
        while i < n {
            if let Some(byte) = bytes.get(i).cloned() {
                if byte == 0x00 {
                    i += 1;
                    if let Some(next_byte) = bytes.get(i).cloned() {
                        pixels_size += next_byte as usize * 8;
                    } else {
                        break;
                    }
                    i += 1;
                } else {
                    for bit_index in 0..BITS_IN_BYTE {
                        if pixels_size < self.pixels.len() {
                            if let Some(pixel_ref) = self.pixels.get_mut(pixels_size) {
                                *pixel_ref = ((byte >> (BITS_IN_BYTE - bit_index - 1)) & 1) * 0xFF;
                            }
                            pixels_size += 1;
                        } else {
                            break;
                        }
                    }
                    i += 1;
                }
            } else {
                break;
            }
        }
    }

    fn get(&self, x: i32, y: i32) -> Option<&u8> {
        if x >= 0 && x < FONT_IMAGE_WIDTH as i32 && y >= 0 && y < FONT_IMAGE_HEIGHT as i32 {
            self.pixels.get(y as usize * FONT_IMAGE_WIDTH + x as usize)
        } else {
            None
        }
    }

    fn render_ascii(&self,
                    display: &mut Display,
                    code: u8,
                    start_x: i32, start_y: i32,
                    scale: i32,
                    color: Pixel) {
        if 32 <= code && code <= 126 {
            let char_x = (code - 32) as usize % FONT_IMAGE_COLS;
            let char_y = (code - 32) as usize / FONT_IMAGE_COLS;

            for y in 0..FONT_CHAR_HEIGHT as i32 {
                for x in 0..FONT_CHAR_WIDTH as i32 {
                    let font_x = char_x as i32 * FONT_CHAR_WIDTH as i32 + x;
                    let font_y = char_y as i32 * FONT_CHAR_HEIGHT as i32 + y;

                    if let Some(alpha) = self.get(font_x, font_y) {
                        if *alpha == 0xFF {
                            let display_start_x = start_x + x * scale;
                            let display_start_y = start_y + y * scale;
                            display.fill_rect(display_start_x, display_start_y, scale, scale, color);
                        }
                    }
                }
            }
        } else {
            // Render '?' for unknown characters
            let char_x = ('?' as u8 - 32) as usize % FONT_IMAGE_COLS;
            let char_y = ('?' as u8 - 32) as usize / FONT_IMAGE_COLS;
            for y in 0..FONT_CHAR_HEIGHT as i32 {
                for x in 0..FONT_CHAR_WIDTH as i32 {
                    let font_x = char_x as i32 * FONT_CHAR_WIDTH as i32 + x;
                    let font_y = char_y as i32 * FONT_CHAR_HEIGHT as i32 + y;
                    if let Some(alpha) = self.get(font_x, font_y) {
                        if *alpha == 0xFF {
                            let display_start_x = start_x + x * scale;
                            let display_start_y = start_y + y * scale;
                            display.fill_rect(display_start_x, display_start_y, scale, scale, color);
                        }
                    }
                }
            }
        }
    }

    fn render_bytes(&self,
                    display: &mut Display,
                    bytes: &[u8],
                    x: i32, y: i32,
                    scale: i32,
                    color: Pixel) {
        for (i, byte) in bytes.iter().enumerate() {
            self.render_ascii(
                display,
                *byte,
                x + i as i32 * FONT_CHAR_WIDTH as i32 * scale, y,
                scale,
                color);
        }
    }

    fn render_bytes_shadowed(&self, display: &mut Display, bytes: &[u8], x: i32, y: i32, scale: i32, color: Pixel, shadow_color: Pixel, shadow_offset: i32 ) {
        // Draw shadow first
        self.render_bytes(display, bytes, x + shadow_offset, y + shadow_offset, scale, shadow_color);
        // Draw text on top
        self.render_bytes(display, bytes, x, y, scale, color);
    }

    fn text_width(&self, text: &[u8], scale: i32) -> i32 {
        text.len() as i32 * FONT_CHAR_WIDTH as i32 * scale
    }

    fn text_height(&self, scale: i32) -> i32 {
        FONT_CHAR_HEIGHT as i32 * scale
    }
}

const LABEL_CAPACITY: usize = 64;
struct Label {
    chars: [u8; LABEL_CAPACITY],
    count: usize,
}

impl Label {
    const fn empty() -> Self {
        Self {
            chars: [0; LABEL_CAPACITY],
            count: 0,
        }
    }

    fn render(&self,
              display: &mut Display,
              font: &Font,
              x: i32, y: i32,
              scale: i32,
              color: Pixel) {
        if let Some(bytes) = self.chars.get(0..self.count) {
            font.render_bytes_shadowed(display, bytes, x, y, scale, color, SHADOW_COLOR, SHADOW_OFFSET);
        }
    }

    fn clear(&mut self) {
        self.count = 0;
    }

    fn push_byte(&mut self, b: u8) {
        if self.count < LABEL_CAPACITY {
            if let Some(char_ref) = self.chars.get_mut(self.count) {
                *char_ref = b;
                self.count += 1;
            }
        }
    }

    fn push_bytes(&mut self, bs: &[u8]) {
        for b in bs {
            self.push_byte(*b);
        }
    }

    fn push_int(&mut self, mut x: i32) {
        let saved_count = self.count;

        if x == 0 {
            self.push_byte(b'0');
        } else {
            while x > 0 && self.count < LABEL_CAPACITY {
                self.push_byte((x % 10) as u8 + b'0');
                x /= 10;
            }

            if x > 0 {
                // x does not fit into the Label rolling back and quitting
                self.count = saved_count;
                return;
            }
        }

        if let Some(chars) = self.chars.get_mut(saved_count .. self.count) {
            chars.reverse();
        }
    }
}

#[repr(C)]
pub struct State {
    player: Entity,
    player_health: i32,
    bullets: [Entity; BULLETS_CAPACITY],
    enemies: [Entity; ENEMIES_CAPACITY],
    enemy_spawn_cooldown: Seconds,
    pause: bool,
    game_over: bool,
    score: usize,
    score_label: Label,
    health_label: Label,
    rng: Rng,
}

impl State {
    const fn default() -> Self {
        Self {
            player: Entity::new(DISPLAY_WIDTH as i32 / 2, DISPLAY_HEIGHT as i32 - PLAYER_SIZE),
            player_health: PLAYER_INITIAL_HEALTH,
            bullets: [Entity::dead(); BULLETS_CAPACITY],
            enemies: [Entity::dead(); ENEMIES_CAPACITY],
            enemy_spawn_cooldown: ENEMY_INITIAL_SPAWN_PERIOD,
            pause: false,
            game_over: false,
            score: 0,
            score_label: Label::empty(),
            health_label: Label::empty(),
            rng: Rng::from_seed(123456789),
        }
    }

    fn update(&mut self, dt: Seconds) {
        if self.pause || self.game_over {
            return;
        }
        // Update bullets
        for bullet in self.bullets.iter_mut() {
            if bullet.alive {
                bullet.y -= (BULLET_SPEED as f32 * dt) as i32;
                if bullet.y + BULLET_SIZE / 2 < 0 {
                    bullet.alive = false;
                }
            }
        }

        for enemy in self.enemies.iter_mut() {
            if enemy.alive {
                {
                    enemy.y += (ENEMY_SPEED as f32 * dt) as i32;
                    if enemy.y - ENEMY_SIZE / 2 > DISPLAY_HEIGHT as i32 {
                        enemy.alive = false;
                        continue;
                    }
                }

                // Check bullet-enemy collision
                for bullet in self.bullets.iter_mut() {
                    if bullet.alive {
                        if enemy.overlaps(ENEMY_SIZE, bullet, BULLET_SIZE) {
                            bullet.alive = false;
                            enemy.alive = false;
                            self.score += PLAYER_KILL_REWARD;
                            break;
                        }
                    }
                }
            }
            // Check player-enemy collision (only if enemy is still alive)
            if enemy.alive && enemy.overlaps(ENEMY_SIZE, &self.player, PLAYER_SIZE) {
                enemy.alive = false;
                self.player_health -= 1;
                if self.player_health <= 0 {
                    self.player.alive = false;
                    self.game_over = true;
                }
            }
        }

        self.enemy_spawn_cooldown -= dt;
        if self.enemy_spawn_cooldown <= 0.0 {
            let enemy_x = self.rng.rand().abs() % (DISPLAY_WIDTH as i32 - ENEMY_SIZE) + ENEMY_SIZE / 2;
            self.spawn_enemy(enemy_x, -ENEMY_SIZE / 2);
            let score_factor = (self.score as f32 * ENEMY_SPAWN_PERIOD_SCORE_FACTOR).min(1.0);
            let new_cooldown = ENEMY_INITIAL_SPAWN_PERIOD * (1.0 - score_factor * 0.8);
            self.enemy_spawn_cooldown = new_cooldown.max(ENEMY_MIN_SPAWN_PERIOD);
        }

        self.score_label.clear();
        self.score_label.push_bytes(b"Score: ");
        self.score_label.push_int(self.score as i32);

        self.health_label.clear();
        self.health_label.push_bytes(b"Health: ");
        self.health_label.push_int(self.player_health.max(0));
    }

    fn render(&self, display: &mut Display, font: &Font) {
        // Always clear the background
        display.fill(DISPLAY_BACKGROUND);
        if !self.pause {
            self.player.render(display, PLAYER_SIZE, PLAYER_COLOR);
            for bullet in self.bullets.iter() {
                bullet.render(display, BULLET_SIZE, BULLET_COLOR)
            }
            for enemy in self.enemies.iter() {
                enemy.render(display, ENEMY_SIZE, ENEMY_COLOR)
            }
            self.score_label.render(display, font,
                                    SCORE_LABEL_X, SCORE_LABEL_Y,
                                    TEXT_SCALE, SCORE_LABEL_COLOR);
            // Render health only if player is alive or game just ended
            if self.player.alive || self.game_over {
                self.health_label.render(display, font,
                                        HEALTH_LABEL_X, HEALTH_LABEL_Y,
                                        TEXT_SCALE, HEALTH_LABEL_COLOR);
            }
            {
                let text = COPYRIGHT_TEXT;
                let scale = COPYRIGHT_SCALE;
                let text_w = font.text_width(text, scale);
                let text_h = font.text_height(scale);
                let x = DISPLAY_WIDTH as i32 - text_w - COPYRIGHT_PADDING as i32;
                let y = DISPLAY_HEIGHT as i32 - text_h - COPYRIGHT_PADDING as i32;
                font.render_bytes_shadowed(display, text, x, y, scale, SCORE_LABEL_COLOR, SHADOW_COLOR, SHADOW_OFFSET / 2);
            }
        }

        if self.pause {
            let text = b"PAUSED";
            let scale = MESSAGE_SCALE;
            let text_w = font.text_width(text, scale);
            let text_h = font.text_height(scale);
            let x = (DISPLAY_WIDTH as i32 - text_w) / 2;
            let y = (DISPLAY_HEIGHT as i32 - text_h) / 2;
            font.render_bytes_shadowed(display, text, x, y, scale, MESSAGE_COLOR, SHADOW_COLOR, SHADOW_OFFSET);
        }

        if self.game_over {
            let game_over_text = b"GAME_OVER";
            let scale = MESSAGE_SCALE;
            let text_w = font.text_width(game_over_text, scale);
            let text_h = font.text_height(scale);
            let game_over_x = (DISPLAY_WIDTH as i32 - text_w) / 2;
            let game_over_y = (DISPLAY_HEIGHT as i32 - text_h) / 2 - text_h / 2;
            font.render_bytes_shadowed(display, game_over_text, game_over_x, game_over_y, scale, MESSAGE_COLOR, SHADOW_COLOR, SHADOW_OFFSET);

            let score_text = self.score_label.chars.get(0..self.score_label.count).unwrap_or(b"");
            let score_scale = TEXT_SCALE;
            let score_text_w = font.text_width(score_text, score_scale);
            let score_x = (DISPLAY_WIDTH as i32 - score_text_w) / 2;
            let score_y = game_over_y + text_h + SCORE_LABEL_PADDING;
            self.score_label.render(display, font, score_x, score_y, score_scale, SCORE_LABEL_COLOR);


            let restart_text = b"Press Space to Restart";
            let restart_scale = TEXT_SCALE;
            let restart_text_w = font.text_width(restart_text, restart_scale);
            let _restart_text_h = font.text_height(restart_scale);
            let restart_x = (DISPLAY_WIDTH as i32 - restart_text_w) / 2;
            let restart_y = score_y + font.text_height(score_scale) + SCORE_LABEL_PADDING;
            font.render_bytes_shadowed(display, restart_text, restart_x, restart_y, restart_scale, MESSAGE_COLOR, SHADOW_COLOR, SHADOW_OFFSET);
        }
    }

    fn spawn_enemy(&mut self, x: i32, y: i32) {
        for enemy in self.enemies.iter_mut() {
            if !enemy.alive {
                enemy.revive(x, y);
                break;
            }
        }
    }

    fn spawn_bullet(&mut self, x: i32, y: i32) {
        for bullet in self.bullets.iter_mut() {
            if !bullet.alive {
                bullet.revive(x, y);
                break;
            }
        }
    }

    fn mouse_move(&mut self, x: i32, _y: i32) {
        if self.player.alive {
            self.player.x = clamp(x, PLAYER_SIZE / 2, DISPLAY_WIDTH as i32 - PLAYER_SIZE / 2);
        }
    }

    fn mouse_click(&mut self) {
        if self.player.alive && !self.pause && !self.game_over {
            self.spawn_bullet(
                self.player.x,
                self.player.y - PLAYER_SIZE / 2 - BULLET_SIZE / 2,
            );
        }
    }

    fn toggle_pause_or_reset(&mut self) {
        if !self.game_over {
            self.pause = !self.pause;
        } else {
            self.reset();
        }
    }

    fn reset(&mut self) {
        *self = Self::default();
    }
}

static mut FONT: Font = Font {
    pixels: [0; FONT_IMAGE_WIDTH * FONT_IMAGE_HEIGHT],
};

static mut STATE: State = State::default();
static mut DISPLAY: Display = Display {
    pixels: [Pixel(0); DISPLAY_WIDTH * DISPLAY_HEIGHT],
};

#[no_mangle]
pub unsafe extern "C" fn init() {
    FONT.decompress_from_bytes(&COMPRESSED_FONT);
    STATE = State::default();
}

#[no_mangle]
pub extern "C" fn get_display_width() -> usize {
    DISPLAY_WIDTH
}

#[no_mangle]
pub extern "C" fn get_display_height() -> usize {
    DISPLAY_HEIGHT
}

#[no_mangle]
pub unsafe extern "C" fn get_display() -> *mut Display {
    // Return a raw pointer instead of a mutable reference
    // This is safer regarding Rust's aliasing rules for static mut
    &mut DISPLAY as *mut Display
}

#[no_mangle]
pub unsafe extern "C" fn next_frame(dt: Seconds) {
    // Accessing static mut is unsafe, but allowed within an unsafe fn
    STATE.update(dt);
    STATE.render(&mut DISPLAY, &FONT);
}

#[no_mangle]
pub unsafe extern "C" fn mouse_move(x: i32, y: i32) {
    STATE.mouse_move(x, y);
}

#[no_mangle]
pub unsafe extern "C" fn mouse_click() {
    STATE.mouse_click();
}

#[no_mangle]
pub unsafe extern "C" fn toggle_pause_or_reset() {
    STATE.toggle_pause_or_reset();
}

#[allow(dead_code)]
extern "C" {
    fn js_sin(x: f32) -> f32;
    fn js_cos(x: f32) -> f32;
}
