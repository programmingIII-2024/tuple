fn main()
{
	let tuple = (5, 3.14, '😻');
	println!("{:?}",tuple);

	let assign: (i32, f64, char, bool) = (2, 2.71, '😇',true);
	let (w, x, y, z) = assign;
	println!("w:{}, x:{}, y:{}, z:{}",w,x,y,z);

	println!("3番目は{}",tuple.2);
}
