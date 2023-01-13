use std::fs::File;
use std::io::prelude::*;
use std::net::Ipv4Addr;


pub fn make_inventory_file(hosts_count: u8, cidr: &str, base_hostname: &str) {
    let mut hosts = vec![];

    let ip_parts: Vec<u8> = cidr
        .split('/')
        .next()
        .unwrap()
        .split('.')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    for hosts_counter in 1..hosts_count+1 {
        let ip = Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3] + hosts_counter);
        hosts.push((format!("{}_{}",base_hostname, hosts_counter), ip.to_string()));
    }

    let groups = vec![("docker-dev", &hosts)];

    let mut inventory = String::new();

    for (group, hosts) in &groups {
        inventory.push_str(&format!("[{}]\n", group));
        for host in hosts.iter() {
            inventory.push_str(&format!("{} ansible_host={}\n", host.0, host.1));
        }
        inventory.push_str("\n");
    }

    let mut file = File::create("inventory").unwrap();
    file.write_all(inventory.as_bytes()).unwrap();
}
