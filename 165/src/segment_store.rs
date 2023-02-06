use crate::geometry::{Coordinates, Segment};
use std::collections::HashMap;
use std::collections::HashSet;
use num_rational::Rational64;

const BBS_S0: usize = 290797;
const BBS_MODULU: usize = 50515093;
const SEGMENT_COUNT: usize = 500;
const COORD_FACTOR: usize = 500;

// BlumBlumShub pseudorandom number generator
//
// See: https://en.wikipedia.org/wiki/Blum_Blum_Shub
fn blum_blum_shub(n: usize) -> usize {
    (0..n).fold(BBS_S0, |sn, _| (sn * sn) % BBS_MODULU)
}

fn generate_segment(n: usize) -> Segment {
    let x1 = blum_blum_shub(n) % COORD_FACTOR;
    let y1 = blum_blum_shub(n + 1) % COORD_FACTOR;
    let x2 = blum_blum_shub(n + 2) % COORD_FACTOR;
    let y2 = blum_blum_shub(n + 3) % COORD_FACTOR;

    Segment::new(x1 as i64,
                 y1 as i64,
                 x2 as i64,
                 y2 as i64,
    )
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
        let segments = (0..SEGMENT_COUNT)
            .map(|i| generate_segment(1 + i * 4))
            .collect::<Vec<_>>();

        self.horiz_sorted_segments = segments.iter().cloned().collect();
        self.vert_sorted_segments = segments;
        self.horiz_sorted_segments
            .sort_by_key(|s| s.bounding_box.left);
        self.vert_sorted_segments
            .sort_by_key(|s| s.bounding_box.top);
    }

    // Prepare a set of candidate segments whose bounding box intersects
    fn prepare_intersection_candidates(
        &self,
        horiz_index: usize,
        vert_index: usize,
    ) -> HashSet<&Coordinates> {
        let mut horiz_candidates: HashSet<&Coordinates> = HashSet::new();
        let mut vert_candidates: HashSet<&Coordinates> = HashSet::new();

        // Prepare horizontal candidates
        let right = self.horiz_sorted_segments[horiz_index].bounding_box.right;
        for i in (horiz_index + 1)..self.horiz_sorted_segments.len() {
            let candidate = &self.horiz_sorted_segments[i];
            if candidate.bounding_box.left >= right {
                break;
            }
            horiz_candidates.insert(&candidate.coords);
        }

        // Prepare vertical candidates
        let bottom = self.vert_sorted_segments[vert_index].bounding_box.bottom;
        for i in (vert_index + 1)..self.vert_sorted_segments.len() {
            let candidate = &self.vert_sorted_segments[i];
            if candidate.bounding_box.top >= bottom {
                break;
            }
            vert_candidates.insert(&candidate.coords);
        }

        // Return the intersection of the horizontal and vertical candidates
        horiz_candidates
            .intersection(&vert_candidates)
            .cloned()
            .collect()
    }

    pub fn count_intersecting_segments(&self) -> i64 {
        let mut distinct_intersections: HashSet<(Rational64, Rational64)> = HashSet::new();
        let mut vert_segment_map: HashMap<Coordinates, usize> = HashMap::new();
        for (i, s) in self.vert_sorted_segments.iter().cloned().enumerate() {
            vert_segment_map.insert(s.coords, i);
        }

        for (horiz_index, &s) in self.horiz_sorted_segments.iter().enumerate() {
            let vert_index = vert_segment_map[&s.coords];
            let seg = &self.horiz_sorted_segments[horiz_index];
            let candidates = self.prepare_intersection_candidates(horiz_index, vert_index);
            for candidate_coords in candidates.iter() {
                let candidate_vert_index = vert_segment_map[candidate_coords];
                let candidate = &self.vert_sorted_segments[candidate_vert_index];
                let (x, y, ok) = seg.intersect(candidate);
                if ok {
                    distinct_intersections.insert((x, y));
                }
            }
        }
        distinct_intersections.len() as i64
    }
}
