use probability::prelude::*;


fn main() {
    println!("Hello, world!");
    let mut source = source::default();
    let distribution = Gaussian::new(1.2, 0.5);//(mu,sigma)=(平均中央,分散)
    dbg!(&distribution);
    dbg!(&distribution.mean());//期待値平均値
    dbg!(&distribution.mode());//最頻値
    dbg!(&distribution.median());//中央値
    dbg!(&distribution.variance());//分散
    dbg!(&distribution.deviation());//標準偏差
    dbg!(&distribution.distribution(1.0));//累積分布関数
    let sampler = Independent(&distribution, &mut source);
    let samples = sampler.take(10).collect::<Vec<_>>();
    dbg!(&samples);
}
