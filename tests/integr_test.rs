mod common;

#[test]
fn super_case() {
	common::get_samp();
	//common::run_samp();

	common::compile_samp_plugin();

	panic!("something goes wrong");
}
