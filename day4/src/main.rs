fn main() {
    let input = "yzbqklnj";

    let mut count = 1;

    loop {
        let to_compute = format!("{input}{count}");

        let digest: md5::Digest = md5::compute(to_compute);
        let digest_string = format!("{:x}", digest);

        if digest_string.starts_with("00000") {
            println!("{}", digest_string);
            println!("part 1 answer: {}", count);
            break;
        }

        count += 1;
    }

    count = 0;
    
    loop {
        let to_compute = format!("{input}{count}");

        let digest: md5::Digest = md5::compute(to_compute);
        let digest_string = format!("{:x}", digest);

        if digest_string.starts_with("000000") {
            println!("\n{}", digest_string);
            println!("part 2 answer: {}", count);
            break;
        }

        count += 1;
    }

    // assert_eq!(format!("{:x}", digest), "c3fcd3d76192e4007dfb496cca67e13b");
}
