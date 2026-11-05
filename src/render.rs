use crate::behavior::Behavior;
use crate::coat::KoiBody;
use crate::koi::{KoiDevelopment, calculate_growth_fraction};
use crate::markings::calculate_streak_marks;
use crate::motion::{Course, SwimPose, calculate_swim_pose, frame_count};
use std::f64::consts::TAU;

pub const FRAME_WIDTH_PIXELS: u32 = 256;
pub const FRAME_HEIGHT_PIXELS: u32 = 256;

pub const RAVEN_WIDTH: usize = 32;
pub const RAVEN_HEIGHT: usize = 32;

struct SpriteFrame {
    y_offset: usize,
    rows: &'static [&'static str],
}

const FRAME_0: &[&str] = &[
    "..B.......................D.................",
    "..BB......................DD................",
    "..BB.....................DDD................",
    "..BB.....................DDDB...............",
    ".BBBK.......BB...........DDDB...............",
    ".BBBK......BBK...........DDDB...............",
    ".BBKK......BBK...........DDDB...............",
    ".BBKK.....BBKK..........DDDDB.......D.......",
    ".BBKK....BBKK...........DDDDBB.....DDD......",
    ".BBKK....BBKK...........DDDDBB....DDDB......",
    ".BBKKK..BBKKK...........DDDBBB...DDDDB......",
    ".BBKKK..BKKKK....BB.....DDDBB...DDDDB.......",
    ".BBKKK..BKKKK...BBB.....DDDBB..DDDDBB.......",
    ".BBKKK.BBKKK...BBBK....DDDDBB.DDDDDB........",
    "..BKKKKBKKKK...BBKK....DDDDBBDDDDDBB...DDD..",
    "..BKKKKBKKKK..BBKKK....DDDDBBDDDDBBB.DDDDD..",
    "..BKKKKBKKKK.BBBKKK....DDDBBDDDDBBB.DDDDB...",
    "..BKKKKKKKKK.BBKKK.....DDDBBDDDBBBBDDDBB....",
    "..BBKKKKKKKKBBBKKK....DDDBBBBBBBBBDDDBB.....",
    "..BBKKKKKKKBBBKKK.....DDDBBBBBBKBDDDDBB.....",
    "..BBKKKKKKKBBKKKK...BBDDBBBBBKKBDDDDBB......",
    "..BBKKKKKKKBKKKK...BBDDBBBBBKKBDDBBBB.......",
    "..BBKKKKKKKKKKKK.BBBKDDBDBKKKBBBBBBK..DDDD..",
    "...BBKKKBKKKKKK.BBBKKDDDBBBBKKBBKKK.DDDBB...",
    "...BBKKBBKKKKKKKBBKKDDDBBBBKKKKKKKDDDBBB....",
    "...BBKKBBKKKKKKBBKK.DDDBBBKKKKKKKDDBBBB.....",
    "....BKBBKKKKKKKKKKK.DDBBKKBBKKKKBBBBK.......",
    "....BBBBKKKKKKKKKK.DDDDBBBBKKKKKKKKK........",
    "....BBBBKKKKKKKKKKBDDDDDBBKKKKKKKKKK........",
    ".....BBBKKKKKKKKKKBDDDDBBKKKKKKKKKK..DDD....",
    ".....BBKKKKKKKKKKKBDDDBBKKBKKKKKKKDDDDD.....",
    ".....BBKKKKKBBKKKKBDDDBBBBKKKKKKKBBBBB......",
    ".....BBKKKBBBKKKKKDDDBBBBKKKKKKKBBKKK.......",
    "......BKBBBBKKKKKBDDBBBKKKKKKKKKKKKK........",
    "......BBBBBBKKKKKBDDBBKKKKKKKKKKKK..........",
    "......BBBBBKKKKKKBDBBBBBBBKKKKKKKKBBBB......",
    "......BBBBKKKKKKBBBBBBBBBKKKKKKKBBBB........",
    ".......BBBBKKKKKBBBBBBBKKKKKKKKBB...........",
    ".......BBBBBKKKKBBBBBBKKKKKBKKKK............",
    ".......BBBKKKKKBBBBBBBBBBBBKKKKBB...........",
    ".....CCCCKKKKKKBBBBBBBBBBBKKKKBB............",
    "....CCCKKKKKKKKBBKKKKKBBBKKKKKK.............",
    "...CCCKKWKKKKKKKKKKKKKKKBBBBBBKK............",
    "..HHHSCKKKCKKKKKKKKKKKKKKKBBBKK.............",
    ".HHSSSSSKCCKKKKKKKKKKKKKKKKKKKKKKKKK........",
    ".HSSSSKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK.....",
    "SSSSKKKCKKKKBBKKKKKKKKKKKKKKKKKKKKKKKKKKKK..",
    "SSSKKCCKKKBBBKKKKKKKKKKKKKKKKKKKKKKKKKKBBBBB",
    "SSKC.......KKKKKKKKKKKKKBKKKKKKKKKKKBBKKK...",
    ".............KKKKKKKKKKKBBBKKK..BBBKKKBBBB..",
    "................KKKKKKKKK.BBKKKK..BBBKK.....",
    "....................BKKKKK...KKKK...BBBB....",
    "......................BBKKKK...BB...........",
    "..........................BK...B............",
    ".........................BBB...B............",
    "........................BB....BK............",
    ".......................BK.....KKK...........",
    "......................BBK.....KKKB..........",
    "......................BKKK....K.K...........",
    "......................KK.KK...KK.K..........",
    "......................KKK......K............",
    ".......................K....................",
];

const FRAME_1: &[&str] = &[
    ".B..........................D...............",
    "BB..........................DD..............",
    "BBK........................DDD..............",
    "BBK........BB..............DDDB.............",
    "BBK.......BBK..............DDDB.............",
    "BBKK......BBK..............DDDB.............",
    "BBKK.....BBKK.............DDDDB.............",
    "BBKK.....BBKK.............DDDBB......DD.....",
    "BBKK....BBKK..............DDDBB.....DDD.....",
    "BBKK....BKKK..............DDDBB....DDDB.....",
    "BBKKK..BBKKK....BB.......DDDDBB...DDDDB.....",
    "BBKKK..BKKKK...BBB.......DDDDB...DDDDB......",
    "BBKKK..BKKK...BBBK.......DDDDB..DDDDBB......",
    "BBKKK.BBKKK...BBKK.......DDDDB.DDDDDB.......",
    ".BKKKKBKKKK..BBKKK......DDDDBBDDDDDBB...DDD.",
    ".BKKKKBKKKK.BBBKKK......DDDDBBDDDDBBB.DDDDD.",
    ".BKKKKBKKKK.BBKKK.......DDDBBDDDDBBB.DDDDB..",
    ".BKKKKKKKKKBBBKKK......DDDDBBDDDBBBBDDDBB...",
    ".BBKKKKKKKBBBKKK.......DDDBBBBBBBBBDDDBB....",
    ".BBKKKKKKKBBKKKK...BB.DDDBBBBBBBKBDDDDBB....",
    ".BBKKKKKKKBKKKKK..BBK.DDBBBBBBKKBDDDDBB.....",
    "..BKKKKKKKKKKKK..BBK..DDBDBBBKKBDDBBBB......",
    "..BBKKKKKKKKKKK.BBKK.DDDDBBKKKBBBBBBK..DDDD.",
    "..BBBKKKBKKKKKKKBBKK.DDDBBBBBKKBBKKK.DDDBB..",
    "...BBKKBBKKKKKKBBKK.DDDBBBBBKKKKKKKDDDBBB...",
    "...BBKKBBKKKKKKKKK..DDBBBKKKKKKKKKDBBBBB....",
    "....BKBBKKKKKKKKKK.BDDDDBBBBBKKKKBBBKK......",
    "....BBBBKKKKKKKKKKBBDDDBBBBKKKKKKKKKK.......",
    "....BBBBKKKKKKKKKKBBDDBBBBKKKKKKKKKK........",
    ".....BBBKKKKKKKKKKBBDDBBKKKKKKKKKKK..DDD....",
    ".....BBKKKKKKKKKKKBDDDBBBBBKKKKKKKDDDDD.....",
    ".....BBKKKKKBKKKKKBDDBBBBKKKKKKKKBBBBB......",
    ".....BBKKKBBBKKKKBBDBBKKKKKKKKKKBBKKK.......",
    "......BKBBBBKKKKKBDDBBBBKKKKKKKKKKKK........",
    "......BBBBBBKKKKKBDBBBBBBBKKKKKKKK..........",
    "......BBBBBKKKKKBBBBBBBBBKKKKKKKKBBBBB......",
    "......BBBBBKKKKKBBBBBBBKKKKKKKKKBBBB........",
    ".......BBBBBKKKKBBBBBBKKKKBBKKKBBB..........",
    ".......BBBBBBKKBBBBBBBBBBBBKKKKK............",
    "........BBBKKKKBBBBBBBBBBBKKKKBBB...........",
    ".....CCCCKKKKKKBBKKKKKBBBKKKKKK.............",
    "....CCCKKKKKKKKKKKKKKKKKBBBBBBKK............",
    "...CCCKKWKKKKKKKKKKKKKKKKKBBBKK.............",
    "..HHHSCKKKCKKKKKKKKKKKKKKKKKKKKKKKK.........",
    ".HHSSSSSKCCKKKKKKKKKKKKKKKKKKKKKKKKKKK......",
    ".HSSSSKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKKK...",
    "SSSSKKKCKKKKBBKKKKKKKKKKKKKKKKKKKKKKKKBBBBBB",
    "SSSKKCCKKKBBBKKKKKKKKKKKKKKKKKKKKKKKBBKKK...",
    "SSKC.........KKKKKKKKKKKBKKKK..BBBBKKBBBBB..",
    "...............KKKKKKKKKBBBKKK...BBBBKK.....",
    "..................KKKKKKK.BBKKKK....BBBB....",
    "....................BKKKKK...KKKK...........",
    "......................BBKKKK...BB...........",
    "..........................BK...B............",
    ".........................BBB...B............",
    "........................BB....BK............",
    ".......................BK.....KKK...........",
    "......................BBK.....KKKB..........",
    "......................BKKK....K.K...........",
    "......................KK.KK...KK.K..........",
    "......................KKK......K............",
    ".......................K....................",
];

const FRAME_2: &[&str] = &[
    ".............KKKKKK.........................",
    ".....CCCCKKKKKKKKKKKKKKKK...................",
    "....CCCKKKKKKKKKBBBBBBBKKKKK................",
    "...CCCKKWKKKKKKBBBBBBBBBBBBBBKKKK...........",
    "..HHHSCKKKCKKKKBBBBBBBBBBBBBBBKKKKK.........",
    ".HHSSSSSKCCKKKKBBBBBBBBBBKKKKKKKKKKKK.......",
    ".HSSSSKKKKKKKKKBBBBBBBBBBBKKKKKKKKKKKKK.....",
    "SSSSKKKCKKKKBBKBBBBBBBBBBBBKKKKKKKKKKKKBKK..",
    "SSSKKCCKKKBBBKKKBBBBBBKKKKKBKKKKKKKBBKKBBBBB",
    "SSKC.....BBBKKKKBBBBBBBKKKKKKKKKBBKKBBKKK...",
    ".........BBKKKKKBBBBBBBBBKKKKKKKKBBBKKBBBB..",
    "........BBKKKKKKKBDBBBBBBBKKKKKKKKBBBBK.....",
    ".......BBKKKKKKKKBDDBBKKKKKKKKKKKKK..BBB....",
    ".......BBKKKKKKKKBDDBBBKKKKKKKKKKKKK........",
    ".......BBKKKKKKKKKDDDBBBBKKKKKKKBBKK........",
    "......BBKKKKKKKKKKBDDDBBBBKKKKKKKBBBB.......",
    "......BBKKKKKKKKKKBDDDBBKKBKKKKKKKDDDB......",
    "......BBKKKKKKKKKKKDDDDBBKKKKKKKKKK.DD......",
    "......BBBKKKKKKKKKKDDDDDBBKKKKKKKKKK.D......",
    ".....BBBBKKKKKKKKKKDDDDBBBBKKKKKKKKK........",
    ".....BBBBKKKKKKKKKKDDDBBKKBBKKKKBBBBK.......",
    ".....BKBBKKKKKKKKKKKDDDBBBKKKKKKKDDBBBB.....",
    "....BBKKBBKKKKKKKKKKDDDBBBBKKKKKKKDDDBBB....",
    "....BBKKBBKKKKKKKBBKKDDDBBBBKKBBKKK.DDDBB...",
    "....BBKKKBKKKKKKKKBBKDDBDBKKKBBBBBBK..DDB...",
    "....BKKKKKKKKKKKK.BBKDDBBBBBKKBDDBBBB...B...",
    "....BKKKKKKKKKKKK..BBKDDBBBBBKKBDDDDBB......",
    "...BBKKKKKKKBKKKKK..BBDDDBBBBBBKBDDDDBB.....",
    "...BBKKKKKKKBBBKKK...BDDDBBBBBBBBBDDDBB.....",
    "...BKKKKKKKKKBBBKKK....DDDBBDDDBBBBDDDBB....",
    "...BKKKKBKKKK.BBKKK....DDDBBDDDDBBB.DDDDB...",
    "...BKKKKBKKKK.BBBKKK...DDDDBBDDDDBBB.DDDDD..",
    "..BBKKKKBKKKK..BBKKK...DDDDBBDDDDDBB...DDD..",
    "..BBKKK.BBKKK...BBKK...DDDDBB.DDDDDB........",
    "..BBKKK..BKKK...BBBK....DDDBB..DDDDBB.......",
    "..BBKK...BKKKK...BBB....DDDBB...DDDDB.......",
    "..BBKK...BBKKK....BB....DDDBBB...DDDDB......",
    "..BBKK....BKKK..........DDDDBB....DDDB......",
    "..BBKK....BBKK..........DDDDBB.....DDD......",
    "..BBKK....BBKK..........DDDDB.......D.......",
    "..BBK......BKK...........DDDB...............",
    "..BBK......BBK...........DDDB...............",
    "..BBK......BBK...........DDDB...............",
    "..BB........BB...........DDDB...............",
    "..B......................DDD................",
    "..........................DD................",
    "..........................D.................",
];

const FRAME_3: &[&str] = &[
    ".............KKKKKK.........................",
    "..........KKKKKKKKKKKKKKK...................",
    ".....CCCCKKKKKKKBBBBBBBKKKKK................",
    "....CCCKKKKKKKKBBBBBBBBBBBKKKKK.............",
    "...CCCKKWKKKKKKBBBBBKKKKKKKKKKKKK...........",
    "..HHHSCKKKCKKKKBBBBBBBBBBBBBBKKKKKK.........",
    ".HHSSSSSKCCKKKKBBBBBBBBBBBBBBBKKKKKKK.......",
    ".HSSSSKKKKKKKKKBBBBBBBBBBKKKKKKKKKKKKKK.....",
    "SSSSKKKCKKKKBBKBBBBBBBBBBBKKKKBBKKKKKKKBKK..",
    "SSSKKCCKKKBBBKKBBBBBBBBBBBBKKKKKBKKBBKKBBBBB",
    "SSKC....BBBBKKKKBBBBBBKKKKBBKKKKKBKKBBKKK...",
    ".......BBBKKKKKKBBDBBBBKKKKKKKKKKKBBKKBBBB..",
    ".......BBBKKKKKKBBDBBBBBBKKKKKKKKBBBBBK.....",
    ".......BBKKKKKKKKBDDBBBBBBKKKKKKKKK..BBB....",
    "......BBKKKKKKKKKBDDBBBBKKKKKKKKKKKK........",
    "......BBKKKKKKKKKBBDDBKKKKKKKKKKBBBKK.......",
    ".....BBBKKKKKKKKKKBDDBBBBKKKKKKKKBBBBB......",
    ".....BBBKKKKKKKKKKBDDDBBBBBKKKKKKKDDDBB.....",
    ".....BBBBKKKKKKKKKBBDDBBKKKKKKKKKKK.DDB.....",
    "....BBBBBKKKKKKKKKKBDDDBBBKKKKKKKKKK..D.....",
    "....BBBBKKKKKKKKKKKBDDDBBBBKKKKKKKKKK.......",
    "....BKBBKKKKKKKKKKKBDDDBBBBBBKKKKBBBBK......",
    "...BBKKBBKKKKKKKKKKKDDDDBKKKKKKKKKDDBBBB....",
    "...BBKKBBKKKKKKKBBKKDDDDBBBBKKBBKKKDDDDBB...",
    "...BBKKKBKKKKKK.BBBKKDDDBBKKKBBBBBBK..DDB...",
    "...BKKKKKKKKKKKK.BBBKDDDBBBBKKBDDBBBB..DD...",
    "..BBKKKKKKKBBKKKK..BBDDDDBBBBKKBDDDDBB......",
    "..BBKKKKKKKBBBKKK...BBDDDBBBBBBKBDDDDBB.....",
    "..BBKKKKKKKKBBBKKK....DDDBBBBBBBBBDDDBB.....",
    "..BKKKKKKKKK.BBKKK....DDDDBBDDDBBBBDDDBB....",
    "..BKKKKBKKKK.BBBKKK....DDDBBDDDDBBB.DDDBB...",
    "..BKKKKBKKKK..BBKKK....DDDBBBDDDDBBB.DDDB...",
    "..BKKKKBKKKK...BBKK....DDDBBBDDDDDBB..DDD...",
    ".BBKKK.BBKKK...BBBK.....DDDBB.DDDDDB...DD...",
    ".BBKKK..BKKKK...BBB.....DDDBBB.DDDDBB.......",
    ".BBKKK..BKKKK....BB.....DDDBBB..DDDDB.......",
    ".BBKK...BBKKK...........DDDBBB...DDDB.......",
    ".BBKK....BBKK...........DDDBBB....DDD.......",
    ".BBKK....BBKK...........DDDDBB....DDD.......",
    ".BBKK.....BBKK..........DDDDB......DD.......",
    ".BBBK......BBK..........DDDDB...............",
    ".BBBK......BBK...........DDDB...............",
    "..BB........BB...........DDDB...............",
    "..BB.....................DDDB...............",
    "..BB.....................DDD................",
    "..B.......................DD................",
    "..........................D.................",
];

const FRAME_4: &[&str] = &[
    ".....CCCCKKKKKKKKKK.........................",
    "....CCCKKKKKKKKKKKKKKKKKK...................",
    "...CCCKKWKKKKKKKBBBBBBBKKKKK................",
    "..HHHSCKKKCKKKKKBBBBBBBBBBKKKKKKK...........",
    ".HHSSSSSKCCKKKKBBBBBBBBBBBBBBKKKKKKK........",
    ".HSSSSKKKKKKKKKBBBBBBBBBBBBKKKKKKKKKKKK.....",
    "SSSSKKKCKKKKBBKBBBBBBBBBBBKKKKKKKKKKKKKKKK..",
    "SSSKKCCKKKBBBKKBBBBBBBBBBBBKKKKKKKKKKKKBBBBB",
    "SSKC...BBBKKKKKKBBBBBBBBBBBBKKKKKKKKBBKKK...",
    ".......BBBKKKKKKBBDDBBBKKKKKKKKKKBBKKKBBBB..",
    ".......BBKKKKKKKBBDDDBBBKKKKKKKKKKBBBKK.....",
    "......BBKKKKKKKKKBDDDBBBKKKKKKKKKKKBBBBB....",
    ".....BBBKKKKKKKKKBDDDDBBBKKKKKKKBBBBB.......",
    ".....BBBKKKKKKKKKBBDDDBBKKKKKKKKKKDDDB......",
    ".....BBBKKKKKKKKKKKDDDDBBKKKKKKKKKK.DDB.....",
    "....BBBKKKKKKKKKKKKDDDDBBBBKKKKKKKKK.DD.....",
    "....BKBKKKKKKKKKKKKBDDDBKKBBKKKKBBBBK.......",
    "...BBKKKKKKKKKKKKKKKDDDBBBKKKKKKKDDBBB......",
    "...BBKKKKKKKKKKKBBKKDDDDBBBKKKKKKKDDDBB.....",
    "...BKKKKKKKBBKKK.BBKKDDDBBBBKKBBKKK.DDBB....",
    "..BBKKKKKKKBBKKKK.BBKDDDDBKKKBBBBBBK..DD....",
    "..BBKKKKKKKKBBKKK..BBDDDDBBBBKKBDDDDBB......",
    "..BKKKKKKKKK.BBKKK..BBDDDBBBBBBKBDDDDB......",
    "..BKKKKBKKKK.BBKKK....DDDDBBBBBBBBDDDBB.....",
    "..BKKKKBKKKK.BBBKKK...DDDDBBDDDBBBBDDDB.....",
    "..BKKKKBKKKK..BBKKK....DDDBBDDDDBBB.DDB.....",
    ".BBKKK.BBKKK...BBKK....DDDDBBDDDDBBB.DDD....",
    ".BBKKK..BKKKK..BBBK....DDDDBBDDDDDBB..DD....",
    ".BBKKK..BKKKK...BBB....DDDDBB.DDDDDBB.......",
    ".BBKK...BBKKK....BB.....DDDBBB.DDDDBB.......",
    ".BBKK....BBKK...........DDDDBB..DDDBB.......",
    ".BBKK....BBKK...........DDDDBB...DDB........",
    ".BBKK.....BBKK..........DDDDB....DDB........",
    ".BBBK......BBK..........DDDDB.....DD........",
    ".BBBK......BBK..........DDDDB.....D.........",
    "..BB........BB...........DDDB...............",
    "..BB.....................DDDB...............",
    "..BB.....................DDD................",
    "..B.......................DD................",
    "..........................D.................",
];

const FRAME_5: &[&str] = &[
    ".....CCCCKKK................................",
    "....CCCKKKKKKKKKKKK.........................",
    "...CCCKKWKKKKKKKKKKKKKKKK...................",
    "..HHHSCKKKCKKKKKBBBBBBBBBKKKK...............",
    ".HHSSSSSKCCKKKKBBBBBBBBBBBBBBKKKKKK.........",
    ".HSSSSKKKKKKKKKBBBBBBBBBBBBKKKKKKKKKKKK.....",
    "SSSSKKKCKKKKBBKBBBBBBBBBBBKKKKKKKKKKKKKKKK..",
    "SSSKKCCKKKBBBKKKBBBDDBBBBBBKKKKKKKBBKKKBBBBB",
    "SSKC...BBBKKKKKKBBDDDDBBBKKKKKKKKKKBBBKKK...",
    ".......BBBKKKKKKKBDDDDBBBKKKKKKKKBBBBBBBBB..",
    "......BBBKKKKKKKKBDDDDDBBBKKKKKKKKDDDDD.....",
    ".....BBBKKKKKKKKKKBDDDDBBBBBKKKKBBBBKDDD....",
    ".....BBKKKKKKKKKKKKDDDDDBBKKKKKKKDDBBBB.....",
    "....BBKKKKKKKKKKKKKKDDDDBBBKKKKKKKDDDBBB....",
    "....BKKKKKKKKKKKKKKKKDDDDBBBKKBBBBBBDDDB....",
    "...BBKKKKKKKKKKKKBKKKDDDDBBBBKKBDDDDB.DD....",
    "...BKKKKKKKBBKKKKBBKKKDDDDBBBBBKBDDDDB......",
    "..BBKKKKKKKBBBKKK.BBKKDDDDBBBBBBBBDDDBB.....",
    "..BKKKKKKKKKBBBKKK.BB.DDDDDBDDDBBBBDDDB.....",
    "..BKKKKKKKKK.BBKKK....BDDDDBDDDDBBB.DDB.....",
    "..BKKKKBKKKK.BBBKKK...KDDDDBBDDDDDBB.DD.....",
    ".BBKKKKBKKKK..BBBKK...KDDDDDB.DDDDDB..D.....",
    ".BBKKK.BKKKK...BBBK....KDDDDBB..DDDBB.......",
    ".BBKKK.BBKKK....BBB.....DDDDBB..DDDBB.......",
    ".BBKK...BKKKK....BB......DDDDB...DDDB.......",
    ".BBKK...BBKKK............DDDDB....DDB.......",
    ".BBKK....BBKK............DDDDB....DDB.......",
    ".BBKK....BBKK.............DDDB.....DD.......",
    ".BBBK.....BBKK............DDDB.....D........",
    ".BBBK......BBK............DDDB..............",
    "..BB.......BBK............DDDB..............",
    "..BB........BB............DDD...............",
    "..BB.......................DD...............",
    "..B........................D................",
];

const RAVEN_FRAMES: [SpriteFrame; 6] = [
    SpriteFrame { y_offset: 37, rows: FRAME_0 },
    SpriteFrame { y_offset: 38, rows: FRAME_1 },
    SpriteFrame { y_offset: 77, rows: FRAME_2 },
    SpriteFrame { y_offset: 75, rows: FRAME_3 },
    SpriteFrame { y_offset: 76, rows: FRAME_4 },
    SpriteFrame { y_offset: 76, rows: FRAME_5 },
];

#[derive(Debug, Eq, PartialEq)]
pub struct KoiFrame {
    rgba_pixels: Vec<[u8; 4]>,
}

impl KoiFrame {
    pub fn as_rgba_bytes(&self) -> &[u8] {
        self.rgba_pixels.as_flattened()
    }

    #[cfg(test)]
    pub(crate) fn from_rgba_pixels(rgba_pixels: Vec<[u8; 4]>) -> Self {
        Self { rgba_pixels }
    }
}

fn compute_centroid(sprite: &SpriteFrame) -> (f64, f64) {
    let mut mass_x = 0.0;
    let mut mass_y = 0.0;
    let mut mass_count = 0.0;
    for (r_idx, row) in sprite.rows.iter().enumerate() {
        for (c_idx, byte) in row.bytes().enumerate() {
            if byte != b'.' {
                mass_x += c_idx as f64;
                mass_y += (r_idx + sprite.y_offset - 37) as f64;
                mass_count += 1.0;
            }
        }
    }
    if mass_count == 0.0 {
        (0.0, 0.0)
    } else {
        (mass_x / mass_count, mass_y / mass_count)
    }
}

fn get_hash(seed: u64, x: usize, y: usize) -> f64 {
    let mut hash = seed
        ^ (x as u64).wrapping_mul(0x9e3779b97f4a7c15)
        ^ (y as u64).wrapping_mul(0xbf58476d1ce4e5b9);
    hash = (hash ^ (hash >> 30)).wrapping_mul(0xbf58476d1ce4e5b9);
    hash = (hash ^ (hash >> 27)).wrapping_mul(0x94d049bb133111eb);
    hash ^= hash >> 31;
    (hash >> 11) as f64 / (1_u64 << 53) as f64
}

fn get_language_colors(dev: &KoiDevelopment, color_fraction: f64) -> [[u8; 4]; 3] {
    let mut colors = [
        [222, 165, 132, 255],
        [184, 111, 80, 255],
        [140, 80, 60, 255],
    ];

    let mut lang_iter: Vec<_> = dev.language_shares.iter().collect();
    lang_iter.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap());

    if !lang_iter.is_empty() {
        let c0 = dev
            .language_colors
            .get(lang_iter[0].0)
            .copied()
            .flatten()
            .map(|c| c.0)
            .unwrap_or([222, 165, 132]);
        colors[0] = [c0[0], c0[1], c0[2], 255];
        colors[1] = colors[0];
        colors[2] = colors[0];

        if lang_iter.len() > 1 {
            let c1 = dev
                .language_colors
                .get(lang_iter[1].0)
                .copied()
                .flatten()
                .map(|c| c.0)
                .unwrap_or([184, 111, 80]);
            colors[1] = [c1[0], c1[1], c1[2], 255];
            colors[2] = colors[1];
        }
        if lang_iter.len() > 2 {
            let c2 = dev
                .language_colors
                .get(lang_iter[2].0)
                .copied()
                .flatten()
                .map(|c| c.0)
                .unwrap_or([140, 80, 60]);
            colors[2] = [c2[0], c2[1], c2[2], 255];
        }
    }

    let pale = 200.0;
    let f = color_fraction.clamp(0.0, 1.0);
    
    for (i, c) in colors.iter_mut().enumerate() {
        let mix = match i {
            0 => (f / 0.15).clamp(0.0, 1.0),
            1 => ((f - 0.15) / 0.45).clamp(0.0, 1.0),
            _ => ((f - 0.60) / 0.40).clamp(0.0, 1.0),
        };
        
        c[0] = (c[0] as f64 * mix + pale * (1.0 - mix)) as u8;
        c[1] = (c[1] as f64 * mix + pale * (1.0 - mix)) as u8;
        c[2] = (c[2] as f64 * mix + pale * (1.0 - mix)) as u8;
    }
    colors
}

fn get_body_palette(body: KoiBody) -> ([u8; 4], [u8; 4], [u8; 4], [u8; 4]) {
    match body {
        KoiBody::White => ([38, 40, 42, 255], [55, 58, 60, 255], [75, 78, 82, 255], [100, 105, 95, 255]),
        KoiBody::Black => ([35, 38, 42, 255], [50, 54, 58, 255], [65, 70, 75, 255], [80, 90, 80, 255]),
        KoiBody::Asagi => ([32, 40, 50, 255], [48, 58, 70, 255], [65, 78, 90, 255], [90, 105, 120, 255]),
        KoiBody::Tea => ([42, 35, 28, 255], [60, 50, 42, 255], [80, 68, 55, 255], [110, 95, 80, 255]),
        KoiBody::Gold => ([45, 42, 25, 255], [68, 62, 40, 255], [95, 85, 55, 255], [140, 120, 60, 255]),
        KoiBody::Platinum => ([38, 42, 48, 255], [58, 64, 70, 255], [78, 85, 95, 255], [120, 130, 140, 255]),
    }
}

pub fn render_koi(koi_development: &KoiDevelopment, koi_body: KoiBody) -> KoiFrame {
    render_frame_internal(koi_development, koi_body, &crate::motion::STILL_POSE, 0.0)
}

pub fn render_swim_frame(
    koi_development: &KoiDevelopment,
    koi_body: KoiBody,
    behavior: Behavior,
    frame_index: u32,
) -> KoiFrame {
    let loop_frames = frame_count(behavior);
    let normalized = frame_index as f64 / loop_frames as f64;
    let swim_pose = calculate_swim_pose(behavior, frame_index);

    let flap_rate = match behavior {
        Behavior::DeepRest => 0.0,
        Behavior::Rest => 1.0,
        Behavior::Cruise => 4.0,
        Behavior::SurfaceFeeding => 3.0,
        Behavior::Dart => 8.0,
    };

    render_frame_internal(koi_development, koi_body, &swim_pose, normalized * flap_rate)
}

fn render_frame_internal(
    dev: &KoiDevelopment,
    body: KoiBody,
    pose: &SwimPose,
    _flap_progress: f64,
) -> KoiFrame {
    let growth_fraction = calculate_growth_fraction(dev.total_contributions);
    let color_fraction = calculate_growth_fraction(dev.total_contributions) / calculate_growth_fraction(500);

    let streak_marks = calculate_streak_marks(dev);
    let mut lang_colors = get_language_colors(dev, color_fraction);

    let sat_shift = if dev.language_shares.is_empty() {
        0.0
    } else {
        (streak_marks.color_vibrancy - 1.0) * 0.5
    };

    if sat_shift != 0.0 {
        for c in &mut lang_colors {
            let gray = 0.3 * c[0] as f64 + 0.59 * c[1] as f64 + 0.11 * c[2] as f64;
            c[0] = (gray + (c[0] as f64 - gray) * (1.0 + sat_shift)).clamp(0.0, 255.0) as u8;
            c[1] = (gray + (c[1] as f64 - gray) * (1.0 + sat_shift)).clamp(0.0, 255.0) as u8;
            c[2] = (gray + (c[2] as f64 - gray) * (1.0 + sat_shift)).clamp(0.0, 255.0) as u8;
        }
    }

    let streak_color = match body {
        KoiBody::Asagi | KoiBody::Gold => [240, 96, 79, 255],
        _ => [254, 174, 52, 255],
    };

    let (mut outline, mut plumage_dk, mut plumage_base, beak) = get_body_palette(body);

    let is_golden_milestone = dev.total_contributions >= 500 && streak_marks.streak_fin_strength >= 0.95;
    
    if is_golden_milestone {
        outline = [75, 55, 15, 255];
        plumage_dk = [150, 115, 25, 255];
        plumage_base = [235, 190, 50, 255];
    }

    let raw_phase = _flap_progress.rem_euclid(1.0);
    let animation_phase = raw_phase - (raw_phase * TAU).sin() * 0.12;
    let exact_phase = animation_phase * 6.0;
    
    let sprite_idx = exact_phase.floor() as usize % 6;
    let next_idx = (sprite_idx + 1) % 6;
    let fract = exact_phase.fract();

    let sprite = &RAVEN_FRAMES[sprite_idx];
    
    let c0 = compute_centroid(sprite);
    let c1 = compute_centroid(&RAVEN_FRAMES[next_idx]);
    let center_x = c0.0 * (1.0 - fract) + c1.0 * fract;
    let center_y = c0.1 * (1.0 - fract) + c1.1 * fract;

    let scale_x = 1.6 + 0.6 * growth_fraction;
    let scale_y = scale_x * (1.0 + 0.10 * growth_fraction);

    let max_w = 75.0 * scale_x;
    let max_h = 85.0 * scale_y;
    let max_radius_x = ((FRAME_WIDTH_PIXELS as f64 - max_w) / 2.0 - 15.0).max(0.0);
    let max_radius_y = ((FRAME_HEIGHT_PIXELS as f64 - max_h) / 2.0 - 15.0).max(0.0);
    let max_radius = max_radius_x.min(max_radius_y);

    let (drift_x, drift_y) = match pose.course {
        Course::Lap { angle_radians } => {
            let radius = (20.0 + 10.0 * growth_fraction).min(max_radius);
            (angle_radians.cos() * radius, angle_radians.sin() * radius)
        }
        Course::Hover { drift_pixels, .. } => (
            drift_pixels[0].clamp(-max_radius, max_radius), 
            drift_pixels[1].clamp(-max_radius, max_radius)
        ),
    };

    let streak_vitality = 1.0 + streak_marks.streak_fin_strength;
    let breathing_bob = pose.wave_strength * pose.wave_phase_radians.sin() * 2.5;
    let flight_bob = (animation_phase * TAU - TAU / 4.0).sin() * (3.0 + 2.0 * growth_fraction) * streak_vitality;

    let bank_lean_vertical = (drift_x / max_radius) * 4.0;
    let pose_center = [
        FRAME_WIDTH_PIXELS as f64 / 2.0 + drift_x,
        FRAME_HEIGHT_PIXELS as f64 / 2.0 + drift_y + breathing_bob - flight_bob + bank_lean_vertical,
    ];

    let start_x = pose_center[0] - center_x * scale_x;
    let start_y = pose_center[1] - center_y * scale_y;

    let mut rgba_pixels = vec![[0; 4]; (FRAME_WIDTH_PIXELS * FRAME_HEIGHT_PIXELS) as usize];

    for py in 0..FRAME_HEIGHT_PIXELS {
        for px in 0..FRAME_WIDTH_PIXELS {
            let fx = px as f64 - start_x;
            let fy = py as f64 - start_y;

            // REMOVED the restrictive 'if fx >= 0.0 && fy >= 0.0' bounding box optimization. 
            // The engine now safely processes coordinates dynamically shifted below start_x (fx < 0)
            // due to aerodynamic banking, completely preventing the beak/feathers from getting chopped.
            let bank_lean_horizontal = (fy - center_y * scale_y) * (drift_x / max_radius) * 0.12;
            
            let sample_x = (fx - bank_lean_horizontal) / scale_x;
            let sample_y = fy / scale_y;
            
            let sx = sample_x.floor() as isize;
            let sy_union = sample_y.floor() as isize;

            let y_relative = sy_union - (sprite.y_offset as isize - 37);

            let ch = if y_relative >= 0
                && (y_relative as usize) < sprite.rows.len()
                && sx >= 0
                && (sx as usize) < sprite.rows[y_relative as usize].len()
            {
                sprite.rows[y_relative as usize].as_bytes()[sx as usize] as char
            } else {
                '.'
            };

            let mut color = match ch {
                'K' => outline,
                'B' => {
                    let sparse_threshold = 0.30 * streak_marks.streak_fin_strength;
                    if streak_marks.streak_fin_strength > 0.0 && get_hash(dev.identity_seed ^ 0x999, sx as usize, sy_union as usize) < sparse_threshold {
                        let blend = 0.20;
                        [
                            (streak_color[0] as f64 * blend + plumage_dk[0] as f64 * (1.0 - blend)) as u8,
                            (streak_color[1] as f64 * blend + plumage_dk[1] as f64 * (1.0 - blend)) as u8,
                            (streak_color[2] as f64 * blend + plumage_dk[2] as f64 * (1.0 - blend)) as u8,
                            255
                        ]
                    } else {
                        plumage_dk
                    }
                }
                'C' => plumage_base,
                'W' => [255, 255, 255, 255],
                'H' | 'S' => beak,
                'D' => {
                    let hash = get_hash(dev.identity_seed, sx as usize, sy_union as usize);
                    let lang_idx = (hash * 3.0).floor() as usize;
                    let mut lc = lang_colors[lang_idx.clamp(0, 2)];

                    let brightness = (0.3 * lc[0] as f64 + 0.59 * lc[1] as f64 + 0.11 * lc[2] as f64) / 255.0;
                    let blend = 0.25 + 0.10 * brightness;

                    lc[0] = (lc[0] as f64 * blend + plumage_dk[0] as f64 * (1.0 - blend)) as u8;
                    lc[1] = (lc[1] as f64 * blend + plumage_dk[1] as f64 * (1.0 - blend)) as u8;
                    lc[2] = (lc[2] as f64 * blend + plumage_dk[2] as f64 * (1.0 - blend)) as u8;
                    lc
                }
                '.' => {
                    let mut bg_color = [0, 0, 0, 0];
                    let dx = sample_x - center_x;
                    let dy = sample_y - center_y;
                    let dist = (dx * dx + dy * dy).sqrt();
                    
                    if dist > 8.0 && dist < 35.0 && !dev.language_shares.is_empty() && streak_marks.streak_fin_strength > 0.05 {
                        let max_particles = (12.0 * streak_marks.streak_fin_strength).ceil() as usize;
                        
                        for i in 0..max_particles {
                            let h1 = get_hash(dev.identity_seed ^ 111, i, 1);
                            let h2 = get_hash(dev.identity_seed ^ 222, i, 2);
                            let h3 = get_hash(dev.identity_seed ^ 333, i, 3);
                            
                            let radius_x = 18.0 + h1 * 12.0;
                            let radius_y = 10.0 + h2 * 8.0; 
                            let speed = if h2 > 0.5 { 1.0 } else { -1.0 } * (0.8 + h3 * 0.5);
                            let offset = h1 * TAU;
                            
                            let t = animation_phase * TAU * speed + offset;
                            
                            let p_x = center_x + t.sin() * radius_x;
                            let p_y = center_y + (t * 2.0).sin() * radius_y;
                            
                            if (sample_x - p_x).hypot(sample_y - p_y) < 0.8 {
                                let aura_color = if is_golden_milestone {
                                    [255, 245, 180, 255] 
                                } else {
                                    [lang_colors[2][0], lang_colors[2][1], lang_colors[2][2], 255]
                                };
                                
                                let brightness = (1.0 - (dist - 8.0) / 32.0).clamp(0.3, 1.0);
                                bg_color = [
                                    (aura_color[0] as f64 * brightness) as u8,
                                    (aura_color[1] as f64 * brightness) as u8,
                                    (aura_color[2] as f64 * brightness) as u8,
                                    255
                                ];
                                break;
                            }
                        }
                    }
                    bg_color
                },
                _ => [0, 0, 0, 0],
            };

            if color[3] > 0
                && !dev.language_shares.is_empty()
                && ch == 'D'
                && streak_marks.glimmer_strength > 0.0
            {
                let hash = get_hash(dev.identity_seed, sx as usize, sy_union as usize);
                let lighting_angle = (animation_phase * TAU + TAU / 2.0).cos().max(0.0);
                let dynamic_glimmer = streak_marks.glimmer_strength * (0.05 + 0.15 * lighting_angle);
                
                if hash < dynamic_glimmer {
                    color = if is_golden_milestone {
                        [255, 255, 255, 255] 
                    } else {
                        [140, 160, 220, 255] 
                    };
                }
            }

            if color[3] > 0 {
                rgba_pixels[(py * FRAME_WIDTH_PIXELS + px) as usize] = color;
            }
        }
    }

    if let Some(phase) = pose.feeding_phase {
        if phase >= 0.0 {
            let bug_x = (pose_center[0] + 35.0 * (phase * TAU).cos()).round() as usize;
            let bug_y = (pose_center[1] + 12.0 * (phase * TAU).sin()).round() as usize;
            if bug_x < FRAME_WIDTH_PIXELS as usize && bug_y < FRAME_HEIGHT_PIXELS as usize {
                rgba_pixels[bug_y * FRAME_WIDTH_PIXELS as usize + bug_x] = [228, 166, 114, 255];
            }
        }
    }

    if pose.depth > 0.0 {
        let haze = (pose.depth * 0.015).min(0.3);
        let ambient_sky = [70, 90, 110, 255];
        for pixel in rgba_pixels.iter_mut() {
            if pixel[3] > 0 {
                let edge_haze = if pixel[0] <= 20 && pixel[1] <= 20 && pixel[2] <= 20 { haze * 0.5 } else { haze };
                
                pixel[0] = (pixel[0] as f64 * (1.0 - edge_haze) + ambient_sky[0] as f64 * edge_haze).round() as u8;
                pixel[1] = (pixel[1] as f64 * (1.0 - edge_haze) + ambient_sky[1] as f64 * edge_haze).round() as u8;
                pixel[2] = (pixel[2] as f64 * (1.0 - edge_haze) + ambient_sky[2] as f64 * edge_haze).round() as u8;
            }
        }
    }

    for pixel in rgba_pixels.iter_mut() {
        if pixel[3] > 0 {
            pixel[3] = 255;
        }
    }

    KoiFrame { rgba_pixels }
}