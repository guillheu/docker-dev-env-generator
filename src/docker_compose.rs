
use std::net::Ipv4Addr;

pub struct ComposeFile {
    pub version: String,
    pub services: Vec<Service>,
    pub networks: Vec<Network>,
}

impl ToString for ComposeFile {
    fn to_string(&self) -> String {
        let version_str = format!("version: {}", self.version);
        let services_str = self.services
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<_>>()
            .join("\n");
        let networks_str = self.networks
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join("\n");

        format!("{}\nservices:\n{}\nnetworks:\n{}", version_str, services_str, networks_str)
    }
}

pub struct Service {
    pub name: String,
    pub hostname: String,
    pub runtime: Option<String>,
    pub image: String,
    pub build: Build,
    pub deploy: Deploy,
    pub networks: Vec<NetworkConnections>,
}

impl ToString for Service {
    fn to_string(&self) -> String {
        let build_str = self.build.to_string();
        let runtime_str = match &self.runtime {
            Some(r) => format!("    runtime: {}", r),
            None => "".to_string(),
        };
        let deploy_str = self.deploy.to_string();
        let networks_str = self
            .networks
            .iter()
            .map(|n| format!("    {}", n.to_string()))
            .collect::<Vec<_>>()
            .join("\n");

        format!(
            "  {}:\n    hostname: {}\n{}\n    image: {}\n{}\n{}\n{}\n",
            self.name, self.hostname, runtime_str, self.image, build_str, deploy_str, networks_str
        )
    }
}

pub struct Build {
    pub context: String,
    pub dockerfile: Option<String>,
}

impl ToString for Build {
    fn to_string(&self) -> String {

        let dockerfile_str = match &self.dockerfile {
            Some(d) => format!("      dockerfile: {}", d),
            None => "".to_string(),
        };
        format!("    build:\n      context: {}\n{}", self.context, dockerfile_str)
    }
}

pub struct Deploy {
    pub resources: Resources,
}


impl ToString for Deploy {
    fn to_string(&self) -> String {
        format!("    deploy:\n{}", self.resources.to_string())
    }
}

pub struct Resources {
    pub limits: ResourceLimits,
    pub reservations: ResourceReservations,
}

impl ToString for Resources {
    fn to_string(&self) -> String {
        format!(
            "      resources:\n{}\n{}",
            self.limits.to_string(),
            self.reservations.to_string()
        )
    }
}

pub struct ResourceLimits {
    pub cpus: String,
    pub memory: String,
}

impl ToString for ResourceLimits {
    fn to_string(&self) -> String {
        format!("        limits:\n          cpus: \"{}\"\n          memory: {}", self.cpus, self.memory)
    }
}

pub struct ResourceReservations {
    pub cpus: String,
    pub memory: String,
}

impl ToString for ResourceReservations {
    fn to_string(&self) -> String {
        format!(
            "        reservations:\n          cpus: \"{}\"\n          memory: {}",
            self.cpus, self.memory
        )
    }
}

pub struct NetworkConnections {
    pub name: String,
    pub ipv4_address: String,
    pub aliases: Vec<String>,
}


impl ToString for NetworkConnections {
    fn to_string(&self) -> String {
        let aliases_str = self.aliases.join(", ");
        format!(
            "networks:\n      {}:\n        ipv4_address: {}\n        aliases: [{}]",
            self.name, self.ipv4_address, aliases_str
        )
    }
}

pub struct Network {
    pub name: String,
    pub driver: Option<String>,
    pub ipam: Option<Ipam>,
}

impl ToString for Network {
    fn to_string(&self) -> String {
        let driver_str = match &self.driver {
            Some(d) => format!("    driver: {}\n", d),
            None => "".to_string(),
        };

        let ipam_str = match &self.ipam {
            Some(i) => format!("{}", i.to_string()),
            None => "".to_string(),
        };

        format!("  {}:\n{}{}", self.name, driver_str, ipam_str)
    }
}

pub struct Ipam {
    pub driver: Option<String>,
    pub config: Vec<IpamConfig>,
}

impl ToString for Ipam {
    fn to_string(&self) -> String {
        let driver_str = match &self.driver {
            Some(d) => format!("      driver: {}\n", d),
            None => "".to_string(),
        };

        let config_str = self
            .config
            .iter()
            .map(|c| format!("{}", c.to_string()))
            .collect::<Vec<_>>()
            .join("\n");
        format!("    ipam:\n{}      config:\n{}", driver_str, config_str)
    }
}

pub struct IpamConfig {
    pub subnet: String,
    pub gateway: Option<String>,
}

impl ToString for IpamConfig {
    fn to_string(&self) -> String {
        let gateway_str = match &self.gateway {
            Some(g) => format!("          gateway: {}", g),
            None => "".to_string(),
        };
        format!("        - subnet: {}\n{}", self.subnet, gateway_str)
    }
}



pub fn make_compose_file(
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

    for hosts_counter in 1..hosts_count+1 {

        let hostname_iter = format!("{}_{}",hostname, hosts_counter);
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
                    ipv4_address: Ipv4Addr::new(ip_parts[0], ip_parts[1], ip_parts[2], ip_parts[3] + hosts_counter).to_string(),
                    aliases: vec![],
                    }],
            }
        )
    }

    let networks = vec![Network {
        name: network_name.to_string(),
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
