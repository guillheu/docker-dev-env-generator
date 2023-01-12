mod docker_compose;
use docker_compose::*;

use std::fs::File;
use std::io::prelude::*;
use std::net::Ipv4Addr;



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
    aliases: Vec<String>
) -> String {


    let mut ip_counter = 1;  // starting with first usable IP address

    let mut hostname_iter = format!("{}_{}",hostname, ip_counter);

    




    let ip_parts: Vec<u8> = cidr
        .split('/')
        .next()
        .unwrap()
        .split('.')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();



    let services = vec![
        Service {
            name: hostname_iter.clone(),
            hostname: hostname_iter.clone(),
            runtime: runtime,
            image: image.to_string(),
            build: Build {
                context: ".".to_string(),
                dockerfile: dockerfile,
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
                ipv4_address: Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], ip_counter).to_string(),
                aliases: aliases,
                }],
        },
    ];
    // increment the IP counter for the next service
    ip_counter += 1;

    let networks = vec![Network {
        name: "consul".to_string(),
        driver: Some("bridge".to_string()),
        ipam: Some(Ipam {
            driver: Some("default".to_string()),
            config: vec![IpamConfig {
                subnet: cidr.to_string(),
                gateway: Some(Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], ip_counter).to_string()),
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
    let hosts_amount = 1;
    let cidr = "10.0.0.0/24";
    let hostname = "host";
    let cpus_limits = "0.5";
    let memory_limits = "256M";
    let cpus_reservations = "0.1";
    let memory_reservations = "50M";
    let runtime = Some("sysbox-runc".to_string());
    let image = "ubuntu-ssh";
    let dockerfile = Some("./ubuntu-ssh.dockerfile".to_string());
    let network_name = "consul";
    let aliases = vec![];
    let yaml = make_compose_file(cidr, hostname, cpus_limits, memory_limits, cpus_reservations, memory_reservations, runtime, image, dockerfile, network_name, aliases);
    let mut file = std::fs::File::create("docker-compose.yml").unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
    println!("docker-compose file created!");
}
