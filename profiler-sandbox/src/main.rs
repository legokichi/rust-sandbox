extern crate cpuprofiler;

fn main() {
    use cpuprofiler::PROFILER;

    PROFILER.lock().unwrap().start("./my-prof.profile").unwrap();
    let mut i = 0;
    loop {
        if i > 100000 { break; }
        println!("{}", i);
        i += 1;
    }
    PROFILER.lock().unwrap().stop().unwrap();
}
