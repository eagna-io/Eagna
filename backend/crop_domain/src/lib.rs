#[macro_use]
extern crate derive_more;
#[macro_use]
extern crate serde;

// # Develop Design Note
// ドメインモデルはインフラ層やアプリケーション層に関する知識を持た>ない。
// つまり、インフラ層のモデルからの変換や、
// アプリケーション層のモデルへの変換はここでは行わない。
// インフラ層のモデルからの変換はrepositoryで、
// アプリケーション層のモデルへの変換はアプリケーション層で行う。
pub mod lmsr;
pub mod market;
pub mod num;
pub mod point;
pub mod user;