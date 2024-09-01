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

    def do_add(self, arg):
        'Add a Docker container name and it\'s codes location on the local machine: add CONTAINER_NAME LOCATION'
        # args = arg.split(maxsplit=1)

        container_name = input("Enter the Docker container name: ").strip()
        if not container_name:
            print("Error: Container name cannot be empty.")
            return

        # Prompt the user for the location
        location = input("Enter the location of the code on the local machine: ").strip()
        if not location:
            print("Error: Location cannot be empty.")
            return
        
        if container_name in self.data:
            print(f"Error: Container \"{container_name}\" already exists in store.")
            return

        self.data[container_name] = location
        self.save_data()
        print(f"Container added: {container_name} with code location: {location}")

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
        'List all stored container names and the locations of their code on the local machine: list'

        print("\nStored containers:")
        print("==================")
        print(f"{'Container Name':<40} {'Code Location':<60} {'Status':<10}")
        print(f"{'-'*40} {'-'*60} {'-'*10}")
        
        for container_name, location in self.data.items():
            print(f"{container_name:<40} {location:<60} ({self.check_container(container_name)})")
        
        print("\n")

    def do_ls(self, arg):
        'List all stored container names and the locations of their code on the local machine: ls'
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
    
    def copy_files(self, container_name, location):
        try:
            print(f"Copying files from {location} to {container_name}...")

            # Expand tilde to the full home directory path if present
            location = os.path.expanduser(location)

            if not os.path.exists(location):
                print(f"Error: Location \"{location}\" does not exist.")
                return
            if not self.container_exists(container_name):
                print(f"Error: Container \"{container_name}\" does not exist.")
                return
            
            subprocess.run(['docker', 'cp', f'{location}/.', f'{container_name}:/app'], check=True)
        except subprocess.CalledProcessError as e:
            print(f"Error occurred while copying files: {e}")
        except Exception as e:
            print(f"An unexpected error occurred: {e}")
    
    def do_r(self, arg):
        'Replace a container\'s code: r || r CONTAINER_NAME_ONE, CONTAINER_NAME_TWO, ...'
        if not self.data:
            print("No containers stored.")
            return
        
        if not arg:
            print("Replacing all container's code...")
            for container_name, location in self.data.items():
                print(f"Replacing container: {container_name} with code location: {location}")
                self.copy_files(container_name, location)
        else:
            containers = arg.split(',')
            for container_name in containers:
                container_name = container_name.strip()
                if container_name in self.data:
                    print(f"Replacing container: {container_name}")
                    self.copy_files(container_name, self.data[container_name])
                else:
                    print(f"Error: Container \"{container_name}\" does not exist in ServiceGenie storage.")

if __name__ == '__main__':
    DevToolCLI().cmdloop()
