const SNOWBALL_WEIGHT_KG: f64 = 0.2;
const SNOWBALL_WEIGHT_LB: f64 = 0.441;
pub struct SnowKg(pub f64);
impl SnowKg {
    pub fn new(kg: f64) -> Self {
        SnowKg(kg)
    }
}
pub struct SnowLb(pub f64);
impl SnowLb {
    pub fn new(lb: f64) -> Self {
        SnowLb(lb)
    }
}
pub struct Snowball(pub i64);
impl Snowball {
    pub fn new(snowballs: i64) -> Self {
        Snowball(snowballs)
    }
}
impl From<SnowKg> for Snowball {
    fn from(kg: SnowKg) -> Self {
        Self((kg.0 / SNOWBALL_WEIGHT_KG).round() as i64)
    }
}

impl From<SnowLb> for Snowball {
    fn from(lb: SnowLb) -> Self {
        Self((lb.0 / SNOWBALL_WEIGHT_LB).round() as i64)
    }
}

fn main() {
    let snowball = Snowball::from(SnowKg::new(1.0));
    println!("{}", snowball.0);
}
