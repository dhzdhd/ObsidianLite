# ObsidianLite

## Setup

- Server
  - AWS
    - Create an AWS IAM user and configure permissions for the same
    - Download AWS CLI and run `aws configure`
  - Download and install pulumi
    - Create a python virtual env in `iac/pulumi` as per `iac/pulumi/requirements.txt`
  - Download and install ansible
    - Create a `hosts.yml` in `iac/ansible` and fill it as per `hosts.yml.example`
- Docker
  - Download and install Docker desktop/engine

## Running the bot

- Server
  - From the `ObsidianLite` directory, run the commands
    - `pulumi up --cwd ./iac/vm/pulumi`
    - `scp -i ~/.ssh/aws.pem .env ubuntu@<ip>:/home/ubuntu/.env`
    - `ansible-playbook -i ./iac/vm/ansible/hosts.yml --private-key ~/.ssh/aws.pem iac/vm/ansible/playbook.yml`
- Docker
  - Building image locally
    - Run `docker compose up -d`
  - Use docker registry
    - Run `docker compose -f docker-compose-prod.yaml up -d`
