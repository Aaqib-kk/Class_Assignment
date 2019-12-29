//------------Class Assignment------
//------------binary to decimal--------------
fn main() {
    let bin_no = "0111";
    let intval = isize::from_str_radix(bin_no, 2).unwrap();
    println!("{}", intval);
}