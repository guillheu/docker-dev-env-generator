mod docker_compose;
use docker_compose::*;

use std::fs::File;
use std::io::prelude::*;



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


fn main() {
    let ipam_config = IpamConfig {
        subnet: "10.33.0.0/16".to_string(),
        gateway: Some("10.33.0.254".to_string()),
    };
    let ipam = Ipam {
        driver: Some("default".to_string()),
        config: vec![ipam_config],
    };
    let network_consul = Network {
        name: "consul".to_string(),
        driver: Some("bridge".to_string()),
        ipam: Some(ipam),
    };
    let network_conn = NetworkConnections {
        name: "consul".to_string(),
        ipv4_address: "10.33.1.1".to_string(),
        aliases: vec!["dc1_server_1".to_string()],
    };
    let limits = ResourceLimits {
        cpus: "0.5".to_string(),
        memory: "256M".to_string(),
    };
    let reservations = ResourceReservations {
        cpus: "0.1".to_string(),
        memory: "50M".to_string(),
    };
    let resources = Resources {
        limits: limits,
        reservations: reservations,
    };
    let deploy = Deploy { resources: resources };
    let build = Build {
        context: ".".to_string(),
        dockerfile: "./ubuntu-ssh.dockerfile".to_string(),
    };
    let service = Service {
        name: "dc1_server_1".to_string(),
        hostname: "dc1_server_1".to_string(),
        runtime: "sysbox-runc".to_string(),
        image: "ubuntu-ssh".to_string(),
        build: build,
        deploy: deploy,
        networks: vec![network_conn],
    };
    let compose_file = ComposeFile {
        version: "'3.8'".to_string(),
        services: vec![service],
        networks: vec![network_consul],
    };

    let yaml = compose_file.to_string();
    let mut file = File::create("docker-compose.yml").unwrap();
    file.write_all(yaml.as_bytes()).unwrap();
}

