//! Type-witness & surface type into rust value.

struct BrownBear;
impl BrownBear {
    fn do_brown_bear_things(&self) -> &str {
        "eating loads of honey"
    }
}
struct PolarBear;
struct Dog;

trait Animal {
    fn pet(&self) -> &str {
        "petting vigourously..."
    }
}
impl Animal for BrownBear {}
impl Animal for PolarBear {}
impl Animal for Dog {}

trait Bear {
    fn growl(&self) -> &str;
}
impl Bear for BrownBear {
    fn growl(&self) -> &str {
        "<brown bear growl>"
    }
}
impl Bear for PolarBear {
    fn growl(&self) -> &str {
        "<menacing polar bear growl>"
    }
}

struct Certified<T>(T);
impl<T> std::ops::Deref for Certified<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn bear_witness<T>(bear: T) -> Certified<T>
where
    T: Bear,
{
    Certified(bear)
}

#[test]
fn certified_bear() {
    let certified_bear = bear_witness(BrownBear);
    println!("{}", certified_bear.growl());
    println!("{}", certified_bear.do_brown_bear_things());

    let certified_bear = bear_witness(PolarBear);
    println!("{}", certified_bear.growl());
    // println!("{}", certified_bear.do_brown_bear_things());

    // let certified_bear = bear_witness(Dog);  // error: the trait `Bear` is not implemented for `Dog`
}
