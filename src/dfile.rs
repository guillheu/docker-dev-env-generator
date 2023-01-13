use std::fs::File;
use std::io::prelude::*;


pub fn create_dockerfile(github_username: &str) {
    let mut file = File::create("ubuntu-ssh.dockerfile").unwrap();

    let contents = format!(r#"FROM nestybox/ubuntu-bionic-systemd-docker:latest
RUN apt update && apt install  openssh-server sudo python3 -y
RUN  echo 'root:password' | chpasswd
RUN sed -i 's/#PermitRootLogin/PermitRootLogin/g' /etc/ssh/sshd_config
ADD https://github.com/{}.keys /root/.ssh/authorized_keys
RUN chmod 600 ~/.ssh/authorized_keys
RUN service ssh restart
EXPOSE 22
# CMD ["/usr/sbin/sshd","-D"]
    "#, github_username);

    file.write_all(contents.as_bytes()).unwrap();
}

