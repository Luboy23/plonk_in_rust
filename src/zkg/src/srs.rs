use crate::{G1Point, G2Point}; // 从当前 crate 中导入 G1 和 G2 仿射点类型
use ark_bls12_381::Fr; // 导入有限域 Fr，BLS12-381 曲线的标量域
use ark_ec::AffineCurve; // 导入椭圆曲线库中的仿射曲线接口
use ark_ff::UniformRand; // 导入用于随机数生成的接口
use std::iter::Iterator; // 导入迭代器 trait

#[derive(Debug)]
pub struct Srs {
    g1: Vec<G1Point>,
    g2: G2Point,
    g2s: G2Point,
}

impl Srs {
    fn g1(s: Fr, length: usize) -> Vec<G1Point> {
        let generator = G1Point::prime_subgroup_generator();
        let first = std::iter::once(generator);
        let rest = std::iter::successors(
            Some(s),
            |previous| Some(*previous * s))
            .map(|sx|{
                generator.mul(sx).into()
            });
        let srs = first.chain(rest).take(length);
        srs.collect()
    }

    fn g2(s: Fr) -> (G2Point, G2Point) {
        let generator = G2Point::prime_subgroup_generator();
        let d = generator.mul(s).into();
        (generator, d)
    }

    pub fn from_secret(s: Fr, gates: usize) -> Self {
        let g1 = Self::g1(s, gates + 3);
        let (g2, g2s) = Self::g2(s);
        Self { g1, g2, g2s }
    }

    #[cfg(feature = "rand")]
    pub fn random(gates:usize) -> Self {
        let mut rng = rand::thread_rng();
        let s = Fr::rand(&mut rng);
        Self::from_secret(s, gates)
    }

    pub fn g1_ref(&self) -> &Vec<G1Point> {
        &self.g1
    }

    pub fn g2_ref(&self) -> &G2Point {
        &self.g2
    }

    pub fn g2s_ref(&self) -> &G2Point {
        &self.g2s
    }


}