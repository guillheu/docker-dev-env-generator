mod docker_compose;
use docker_compose::*;

use std::fs::File;
use std::io::prelude::*;
use std::net::Ipv4Addr;
use clap::{Arg, Command, value_parser};






fn create_dockerfile() {
    let mut file = File::create("ubuntu-ssh.dockerfile").unwrap();

    let contents = r#"FROM nestybox/ubuntu-bionic-systemd-docker:latest
RUN apt update && apt install  openssh-server sudo python3 -y
RUN  echo 'root:password' | chpasswd
RUN sed -i 's/#PermitRootLogin/PermitRootLogin/g' /etc/ssh/sshd_config
COPY authorized_keys /root/.ssh/authorized_keys
RUN chmod 600 ~/.ssh/authorized_keys
RUN service ssh restart
EXPOSE 22
# CMD ["/usr/sbin/sshd","-D"]
    "#;

    file.write_all(contents.as_bytes()).unwrap();
}



fn make_compose_file(
    hosts_count: u8,
    cidr: &str, 
    hostname: &str, 
    cpus_limits: &str, 
    memory_limits: &str, 
    cpus_reservations: &str,
    memory_reservations: &str,
    runtime: Option<String>,
    image: &str,
    dockerfile: Option<String>,
    network_name: &str,
) -> String {




    


    let ip_parts: Vec<u8> = cidr
        .split('/')
        .next()
        .unwrap()
        .split('.')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut services = Vec::<Service>::new();

    for service_counter in 1..hosts_count+1 {

        let hostname_iter = format!("{}_{}",hostname, service_counter);
        services.push(
            Service {
                name: hostname_iter.clone(),
                hostname: hostname_iter.clone(),
                runtime: runtime.clone(),
                image: image.to_string(),
                build: Build {
                    context: ".".to_string(),
                    dockerfile: dockerfile.clone(),
                },
                deploy: Deploy {
                    resources: Resources {
                        limits: ResourceLimits {
                            cpus: cpus_limits.to_string(),
                            memory: memory_limits.to_string(),
                        },
                        reservations: ResourceReservations {
                            cpus: cpus_reservations.to_string(),
                            memory: memory_reservations.to_string(),
                        },
                    },
                },
                networks: vec![NetworkConnections {
                    name: network_name.to_string(),
                    ipv4_address: Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3] + service_counter).to_string(),
                    aliases: vec![],
                    }],
            }
        )
    }

    let networks = vec![Network {
        name: "consul".to_string(),
        driver: Some("bridge".to_string()),
        ipam: Some(Ipam {
            driver: Some("default".to_string()),
            config: vec![IpamConfig {
                subnet: cidr.to_string(),
                gateway: Some(Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3] + hosts_count +1).to_string()),
            }],
        }),
    }];

    let compose_file = ComposeFile {
        version: "'3'".to_string(),
        services,
        networks,
    };

    compose_file.to_string()
}


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
    let hostname = matches.get_one::<String>("base-hostname").unwrap();
    let cpus_limits = matches.get_one::<String>("cpus-limits").unwrap();
    let memory_limits = matches.get_one::<String>("memory-limits").unwrap();
    let cpus_reservations = matches.get_one::<String>("cpus-reservations").unwrap();
    let memory_reservations = matches.get_one::<String>("memory-reservations").unwrap();
    let runtime = Some(matches.get_one::<String>("runtime").unwrap().to_string());
    let image = matches.get_one::<String>("image").unwrap();
    let dockerfile = Some(matches.get_one::<String>("dockerfile").unwrap().to_string());
    let network_name = matches.get_one::<String>("network-name").unwrap();

    // Necessary
    // let hosts_count:u8 = 3;


    // Optional with default values
    // let cidr = "10.122.88.0/24";
    // let hostname = "host";
    // let cpus_limits = "0.5";
    // let memory_limits = "256M";
    // let cpus_reservations = "0.1";
    // let memory_reservations = "50M";
    // let runtime = Some("sysbox-runc".to_string());
    // let image = "ubuntu-ssh";
    // let dockerfile = Some("./ubuntu-ssh.dockerfile".to_string());
    // let network_name = "docker-custom-network";


    let yaml = make_compose_file(hosts_count, cidr, hostname, cpus_limits, memory_limits, cpus_reservations, memory_reservations, runtime, image, dockerfile, network_name);
    let mut file = std::fs::File::create("docker-compose.yml").unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
    create_dockerfile();
    println!("docker-compose file and dockerfile created!");
}
