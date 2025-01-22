use std::collections::HashSet;

use crate::{answer::{Answer, DisplayableAnswer}, helper::{boundary::Boundary, direction, position::UPosition, table::Table}, solver::Solve};

use super::{fence_unit::FenceUnit, perimiter_calculate::PerimiterCalculate};

pub struct FencePriceCalculator {
    perimiter_calculator: Box<dyn PerimiterCalculate>
}

mod error {
    const PREFIX: &str = "[Solver D-12]";

    pub fn overlflow(acc: u64, perimeter: u64, area: u64) -> String {
        format!("{} overflow occurred during price calculation step: {} + ({} * {})", PREFIX, acc, perimeter, area)
    }
}

impl FencePriceCalculator {
    pub fn new<PC>(perimiter_calculator: PC) -> FencePriceCalculator where PC: PerimiterCalculate + 'static {
        FencePriceCalculator { perimiter_calculator: Box::new(perimiter_calculator) }
    }

    fn region_pricing_step(price_acc: u64, new_region_perimeter: u64, new_region_area: u64) -> Result<u64, String> {
        new_region_perimeter.checked_mul(new_region_area)
            .and_then(|product|product.checked_add(price_acc))
            .ok_or_else(||error::overlflow(price_acc, new_region_perimeter, new_region_area))
    }

    /// Analyses the same plant type region whose member field is at the position provided.
    /// Returns a pair of `(F, P)` where `F` is a set of `FenceUnit` representing the border of the region
    /// and `P` is a set of `UPosition` representing the member positions.
    fn analyse_region(&self, map: &Table<char>, member_position: UPosition) -> (HashSet<FenceUnit>, HashSet<UPosition>) {
        let mut fence = HashSet::new();
        let mut region = HashSet::new();
        let region_plant_type = match map.get_pos(member_position) {
            Some(&plant_type) => plant_type,
            None => return (fence, region),
        };

        let mut queue = vec![member_position];
        while !queue.is_empty() {
            let position = queue.pop().unwrap();

            if region.contains(&position) { continue; }

            for dir in direction::Direction::all() {
                // next is `Some(pos)` if it is on the map and of the same plant type
                let next = map.boundary().apply(dir.movement(), position)
                    .filter(|&pos|*map.get_pos(pos).unwrap() == region_plant_type);
                
                if let Some(pos) = next {
                    queue.push(pos);
                } else {
                    fence.insert(FenceUnit::new(position, dir));
                }
            }
            region.insert(position);
        }

        (fence, region)
    }

    fn price(&self, map: Table<char>) -> Result<u64, String> {
        let mut total_price = Ok(0);
        let mut done = HashSet::new();

        for (position, _) in map.iter() {

            if done.contains(&position) { continue; }

            let (fence, region) = self.analyse_region(&map, position);
            let area = region.len() as u64;
            region.into_iter().for_each(|p|{ done.insert(p); });

            let perimiter_multiplier = self.perimiter_calculator.calculate(fence);
            
            total_price =  total_price
                .and_then(|price|Self::region_pricing_step(price, perimiter_multiplier, area));

        }
        total_price
    }
}

impl Solve<Table<char>> for FencePriceCalculator {
    fn solve(&self, input: Table<char>) -> Result<Answer, String> {
        self.price(input).map(DisplayableAnswer::new)
    }
}