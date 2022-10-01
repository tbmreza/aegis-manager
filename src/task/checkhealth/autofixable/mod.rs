pub mod vpn;

pub fn aegis_apps_is_empty() -> Option<(String, String)> {
    use crate::task::docker_ps_quiet;

    if docker_ps_quiet("aegis").is_empty() {
        return None;
    }
    Some((
        String::from("No aegis apps are running. Start?"),
        String::from("aegis start 1"),
    ))
}
