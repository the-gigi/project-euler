use crate::geometry::Segment;

const BBS_S0: usize = 290797;
const BBS_MODULU: usize = 50515093;

// BlumBlumShub pseudorandom number generator
//
// See: https://en.wikipedia.org/wiki/Blum_Blum_Shub
fn blum_blum_shub(n: usize) -> usize {
    (0..n).fold(BBS_S0, |sn, _| (sn * sn) % BBS_MODULU)
}

fn generate_segment(n: usize) -> Segment {
    let x1 = blum_blum_shub(n) as i32;
    let y1 = blum_blum_shub(n + 1) as i32;
    let x2 = blum_blum_shub(n + 2) as i32;
    let y2 = blum_blum_shub(n + 3) as i32;

    Segment::new(x1, y1, x2, y2)
}

#[derive(Debug)]
pub struct SegmentStore {
    horiz_sorted_segments: Vec<Segment>,
    vert_sorted_segments: Vec<Segment>,
}

impl SegmentStore {
    pub fn new() -> Self {
        Self {
            horiz_sorted_segments: vec![],
            vert_sorted_segments: vec![],
        }
    }

    pub fn generate_segments(&mut self) {
        let segments = (0..5)
            .map(|i| generate_segment(1 + i * 4))
            .collect::<Vec<_>>();

        self.horiz_sorted_segments = segments.iter().cloned().collect();
        self.vert_sorted_segments = segments;
        self.horiz_sorted_segments.sort_by_key(|s| s.bounding_box.left);
        self.vert_sorted_segments.sort_by_key(|s| s.bounding_box.top);
    }
}
