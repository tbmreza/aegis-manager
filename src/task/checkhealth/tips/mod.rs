fn apps_running_on_expected_ports() {
    let expectation = "Apps running on port xx and yy.";
    let disappoints = true;
    if disappoints {
        println!("VIOLATED: {expectation}");
        println!("npm run dev");
    }
}

pub fn check() {
    apps_running_on_expected_ports();
}
