use baby_shark::geometry::traits::RealNumber;
use nalgebra::Vector3;
use num_traits::Float;

pub fn opposite_angle<S>(p1: &Vector3<S>, p2: &Vector3<S>, p3: &Vector3<S>) -> S
where
    S: RealNumber + std::convert::From<i8>,
{
    let a_squared = (p1 - p2).norm_squared();
    let b_squared = (p2 - p3).norm_squared();
    let c_squared = (p3 - p1).norm_squared();

    Float::acos(
        (b_squared + c_squared - a_squared)
            / (Into::<S>::into(2 as i8) * Float::sqrt(b_squared) * Float::sqrt(c_squared)),
    )
}
