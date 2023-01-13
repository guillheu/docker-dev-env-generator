mod docker_compose;
mod dfile;
mod ansible_inventory;

use docker_compose::*;
use dfile::*;
use ansible_inventory::*;

use std::io::prelude::*;
use clap::{Arg, Command, value_parser};






fn main() {


    let matches = Command::new("Docker-Ansible dev environment generator")
        .arg(Arg::new("hosts_count")
            .required(true)
            .value_parser(value_parser!(u8))
            .help("Number of hosts to run"))
        .arg(Arg::new("cidr")
            .short('c')
            .long("cidr")
            .default_value("10.122.88.0/24")
            .help("CIDR of the network to create"))
        .arg(Arg::new("base-hostname")
            .short('h')
            .long("base-hostname")
            .default_value("host")
            .help("Base hostname to which will be appended the host counter, e.g. host_1, hots_2 etc."))
        .arg(Arg::new("runtime")
            .default_value("sysbox-runc")
            .short('r')
            .long("runtime")
            .help("Which runtime to run the containers with. for more info https://docs.docker.com/compose/compose-file/compose-file-v2/#runtime"))
        .arg(Arg::new("image")
            .default_value("ubuntu-ssh")
            .short('i')
            .long("image")
            .help("Name of the image to run containers from"))
        .arg(Arg::new("dockerfile")
            .default_value("./ubuntu-ssh.dockerfile")
            .short('d')
            .long("dockerfile")
            .help("Name of the dockerfile to build the container image from"))
        .arg(Arg::new("network-name")
            .default_value("docker-custom-network")
            .short('n')
            .long("network-name")
            .help("Name of the network to create and run the hosts in. Note that this is purely cosmetic"))
        .arg(Arg::new("cpus-limits")
            .default_value("0.5")
            .long("cpus-limits")
            .help("Maximum CPU each host can use, in multiples of cores (\"0.5\" would mean half a core)"))
        .arg(Arg::new("memory-limits")
            .default_value("256M")
            .long("memory-limits")
            .help("Maximum RAM each host can use. Uses standard units notation (B, K, M, G, T)"))
        .arg(Arg::new("cpus-reservations")
            .default_value("0.1")
            .long("cpus-reservations")
            .help("Minimum CPU each host can use, in multiples of cores (\"0.1\" would mean a tenth a core)"))
        .arg(Arg::new("memory-reservations")
            .default_value("50M")
            .long("memory-reservations")
            .help("Minimum RAM each host can use. Uses standard units notation (B, K, M, G, T)"))
        .get_matches();

    let hosts_count = matches.get_one::<u8>("hosts_count").expect("hosts count is required").clone();
    let cidr = matches.get_one::<String>("cidr").unwrap();
    let base_hostname = matches.get_one::<String>("base-hostname").unwrap();
    let cpus_limits = matches.get_one::<String>("cpus-limits").unwrap();
    let memory_limits = matches.get_one::<String>("memory-limits").unwrap();
    let cpus_reservations = matches.get_one::<String>("cpus-reservations").unwrap();
    let memory_reservations = matches.get_one::<String>("memory-reservations").unwrap();
    let runtime = Some(matches.get_one::<String>("runtime").unwrap().to_string());
    let image = matches.get_one::<String>("image").unwrap();
    let dockerfile = Some(matches.get_one::<String>("dockerfile").unwrap().to_string());
    let network_name = matches.get_one::<String>("network-name").unwrap();


    let yaml = make_compose_file(hosts_count, cidr, base_hostname, cpus_limits, memory_limits, cpus_reservations, memory_reservations, runtime, image, dockerfile, network_name);
    let mut file = std::fs::File::create("docker-compose.yml").unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
    create_dockerfile();
    make_inventory_file(hosts_count, cidr, base_hostname);
}
