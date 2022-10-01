fn apps_running_on_expected_ports() {
    let expectation = "Apps running on port xx and yy.";
    println!("CHECKING: {expectation}");

    let disappoints = true;
    if disappoints {
        println!("FAILED: npm run dev");
    }
}

pub fn check() {
    apps_running_on_expected_ports();
}
