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
    services_count: u8,
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
    aliases: Vec<Vec<String>>
) -> String {




    


    let ip_parts: Vec<u8> = cidr
        .split('/')
        .next()
        .unwrap()
        .split('.')
        .map(|s| s.parse::<u8>().unwrap())
        .collect();

    let mut services = Vec::<Service>::new();

    for service_counter in 1..services_count+1 {

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
                    aliases: aliases[service_counter as usize -1 as usize].clone(),
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
                gateway: Some(Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3] + services_count +1).to_string()),
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

    // Necessary
    let services_count:u8 = 3;


    // Optional with default values
    let cidr = "10.122.88.0/24";
    let hostname = "host";
    let cpus_limits = "0.5";
    let memory_limits = "256M";
    let cpus_reservations = "0.1";
    let memory_reservations = "50M";
    let runtime = Some("sysbox-runc".to_string());
    let image = "ubuntu-ssh";
    let dockerfile = Some("./ubuntu-ssh.dockerfile".to_string());
    let network_name = "docker-custom-network";
    let aliases = vec![vec![]; services_count as usize];


    let yaml = make_compose_file(services_count, cidr, hostname, cpus_limits, memory_limits, cpus_reservations, memory_reservations, runtime, image, dockerfile, network_name, aliases.clone());
    let mut file = std::fs::File::create("docker-compose.yml").unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
    create_dockerfile();
    println!("docker-compose file and dockerfile created!");
}
