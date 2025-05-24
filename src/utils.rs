pub fn filtered_hostmask(hostmask: &str) -> String {
    if !hostmask.contains("!") {
        return hostmask.to_string();
    }

    hostmask.replace("~", "").split("!").collect::<Vec<&str>>()[1].to_lowercase().to_string()
}

pub fn parse_nick(hostmask: &str) -> String {
    hostmask.split("!").collect::<Vec<&str>>()[0].to_string()
}
