fn main() {
    // 非常神奇,必须要指定out_dir目录为src的子目录才能够构建
    prost_build::Config::new()
        .out_dir("src/pb")
        .compile_protos(&["src/abi.proto"], &["."])
        .unwrap();
}