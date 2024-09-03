"""
ServiceGenie CLI - A Docker Compose Helper for Microservices

This script provides a command-line interface (CLI) for managing Docker containers
in a microservices environment. It allows users to ...

Commands:
- ...

Usage:

"""

import cmd
import json
import os
import subprocess
from datetime import datetime

DATA_FILE = 'containers.json'

class DevToolCLI(cmd.Cmd):
    intro = 'Welcome to the DevTool CLI. Type help or ? to list commands.\n'
    prompt = '\033[95m(ServiceGenie)\033[0m '

    def __init__(self):
        super().__init__()
        self.data = self.load_data()

    def load_data(self):
        if os.path.exists(DATA_FILE):
            with open(DATA_FILE, 'r') as file:
                return json.load(file)
        return {}
    
    def save_data(self):
        with open(DATA_FILE, 'w') as file:
            json.dump(self.data, file, indent=4)

    def add_container(self, container_name, workspace):
        if container_name in self.data:
            print(f"Error: Container \"{container_name}\" already exists in store.")
            return

        self.data[container_name] = {
            'workspace': workspace,
            'last_updated': datetime.now().isoformat()  # Add the current timestamp
        }
        self.save_data()
        print(f"Container added: {container_name} with code workspace: {workspace}")

    def do_add(self, arg):
        'Add a Docker container name and it\'s codes workspace on the local machine: add'
        # args = arg.split(maxsplit=1)

        container_name = input("Enter the Docker container name: ").strip()
        if not container_name:
            print("Error: Container name cannot be empty.")
            return

        # Prompt the user for the workspace
        workspace = input("Enter the path to the code workspace: ").strip()
        if not workspace:
            print("Error: workspace cannot be empty.")
            return
        
        self.add_container(container_name, workspace)

    def container_exists(self, container_name):
        result = subprocess.run(['docker', 'ps', '-a', '--format', '{{.Names}}'], capture_output=True, text=True)
        return container_name in result.stdout.split()

    def check_container(self, container_name):
        try:
            if not self.container_exists(container_name):
                return "Unknown"

            # Get container status using `docker inspect`
            result = subprocess.run(
                ['docker', 'inspect', '--format', '{{.State.Status}}', container_name],
                check=True,
                stdout=subprocess.PIPE,
                stderr=subprocess.PIPE,
                text=True
            )
            status = result.stdout.strip()
            return status

        except subprocess.CalledProcessError as e:
            return "Unknown"

    def do_list(self, arg):
        'List all stored container names and the workspaces of their code on the local machine: list'

        print("\nStored containers:")
        print("==================")
        print(f"{'Container Name':<40} {'Code workspace':<60} {'Status':<10} {'Last Updated':<20}")
        print(f"{'-'*40} {'-'*60} {'-'*10} {'-'*20}")
        
        for container_name, info in self.data.items():
            workspace = info.get('workspace', '-')
            status = self.check_container(container_name)
            last_updated = (datetime.now() - datetime.fromisoformat(info.get('last_updated', '-'))).total_seconds() / 60

            updated = f"{int(last_updated)} minutes ago" if last_updated >= 1 else "Just now"
            print(f"{container_name:<40} {workspace:<60} {status:<10} {updated:<20}")
        
        print("\n")

    def do_ls(self, arg):
        'List all stored container names and the workspaces of their code on the local machine: ls'
        self.do_list(arg)
        

    def do_remove(self, arg):
        'Remove a stored Docker container\'s data: remove CONTAINER_NAME'

        container_name = input("Enter the Docker container name: ").strip()
        if not container_name:
            print("Error: Container name cannot be empty.")
            return
        
        if container_name in self.data:
            del self.data[container_name]
            self.save_data()
            print(f"Container data removed: {container_name}")
        else:
            print(f"Error: Container \"{container_name}\" does not exist.")

    def do_remove_all(self, arg):
        'Remove all stored Docker container data: remove_all'
        if not self.data:
            print("No containers stored.")
            return
        self.data.clear()
        self.save_data()
        print("All container data removed.")

    def do_exit(self, arg):
        'Exit the CLI'
        print('Exiting...')
        return True
    
    def copy_files(self, container_name, workspace):
        try:
            print(f"Copying files from {workspace} to {container_name}...")

            # Expand tilde to the full home directory path if present
            workspace = os.path.expanduser(workspace)

            if not os.path.exists(workspace):
                print(f"Error: workspace \"{workspace}\" does not exist.")
                return
            if not self.container_exists(container_name):
                print(f"Error: Container \"{container_name}\" does not exist.")
                return
            
            subprocess.run(['docker', 'cp', f'{workspace}/.', f'{container_name}:/app'], check=True)
        except subprocess.CalledProcessError as e:
            print(f"Error occurred while copying files: {e}")
        except Exception as e:
            print(f"An unexpected error occurred: {e}")

    def update_timestamp(self, container_name):
        if container_name in self.data:
            self.data[container_name]['last_updated'] = datetime.now().isoformat()
            self.save_data()
        else:
            print(f"Error: Container \"{container_name}\" does not exist in ServiceGenie storage.")
    
    def do_copy(self, arg):
        'Copy a container\'s code: copy || copy CONTAINER_NAME_ONE, CONTAINER_NAME_TWO, ...'
        if not self.data:
            print("No containers stored.")
            return
        
        if not arg:
            print("Copying all container's code...")
            for container_name, info in self.data.items():
                workspace = info.get('workspace', '-')
                print(f"Copying code to container: {container_name} with code workspace: {workspace}")
                self.copy_files(container_name, workspace)
                self.update_timestamp(container_name)
        else:
            containers = arg.split(',')
            for container_name in containers:
                container_name = container_name.strip()
                if container_name in self.data:
                    workspace = self.data[container_name]['workspace']
                    print(f"Copying code to container: {container_name} with code workspace: {workspace}")
                    self.copy_files(container_name, workspace)
                    self.update_timestamp(container_name)
                else:
                    print(f"Error: Container \"{container_name}\" does not exist in ServiceGenie storage.")

    def do_c(self, args):
        'Copy a container\'s code: copy || copy CONTAINER_NAME_ONE, CONTAINER_NAME_TWO, ...'
        self.do_copy(args)

if __name__ == '__main__':
    DevToolCLI().cmdloop()
