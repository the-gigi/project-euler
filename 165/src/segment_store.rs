use std::ops::Deref;
use crate::geometry::Segment;

const BBS_S0: usize = 290797;
const BBS_MODULU: usize = 50515093;

// BlumBlumShub pseudorandom number generator
//
// See: https://en.wikipedia.org/wiki/Blum_Blum_Shub
fn blum_blum_shub(n: usize) -> usize {
    let mut sn = BBS_S0;
    for _ in 0..n {
        sn = (sn * sn) % BBS_MODULU
    }
    sn
}

fn generate_segment(n: usize) -> Segment {
    let x1= blum_blum_shub(n) as i32;
    let y1 = blum_blum_shub(n + 1) as i32;
    let x2 = blum_blum_shub(n + 2) as i32;
    let y2 = blum_blum_shub(n + 3) as i32;

    Segment::new(x1, y1, x2, y2)
}

#[derive(Debug)]
pub struct SegmentStore {
    horiz_sorted_segments:Vec<Segment>,
    vert_sorted_segments: Vec<Segment>,
}

impl SegmentStore {
    pub fn new() -> Self {
        SegmentStore{
            horiz_sorted_segments: vec![],
            vert_sorted_segments: vec![],
        }
    }

    pub fn generate_segments(&mut self){
        for i in 0..5 {
            let n = 1 + i * 4;
            let segment = generate_segment(n);
            self.horiz_sorted_segments.push(segment.clone());
            self.vert_sorted_segments.push(segment);
        }

        self.horiz_sorted_segments.sort_by(|a, b| a.bounding_box.left.cmp(&b.bounding_box.left));
        self.vert_sorted_segments.sort_by(|a, b| a.bounding_box.top.cmp(&b.bounding_box.top));
    }
}
