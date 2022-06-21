use rand::Rng;

pub fn gen_ran() -> u8 {
    let mut rng = rand::thread_rng(); // 乱数の生成器の作成
    let n: u8 = rng.gen(); // 生成器のgen（）をコール
    n
}
