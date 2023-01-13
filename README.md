# docker-dev-env-generator
The goal of this project is to quickly create development environments for infrastructure solutions using docker that can quickly be used with ansible. Using docker-compose, it will setup a network with an arbitrary amount of hosts running on a chosen CIDR based on special ubuntu-based images with ssh and systemd enabled.<br>
It will generate (or overwrite) 3 files in the current directory :
 * a `docker-compose.yml` file containing the definition of our services (hosts) and network
 * a `ubuntu-ssh.dockerfile` file that's the dockerfile of the containers the hosts will be running
 * a `inventory.ini` file that's an ansible inventory file that contains a group with all the docker hosts defined in the compose file