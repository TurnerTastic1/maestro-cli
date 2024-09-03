# Maestro

<img src="images/maestro.png" alt="Maestro CLI" width="500" />

## Overview

Maestro is a development tool designed to simplify the management and monitoring of large quantities of containerized microservices. It provides functionalities for adding, listing, and updating container code, as well as checking container statuses.

Supported microservice frameworks:

- SpringBoot
- ...

## Features

## Installation

TBD

HOMEBREW

Github Releases

## Usage

#### Basic commands

Add a container

```sh
Maestro> add
```

List tracked containers (list or ls)

```sh
Maestro> ls

Stored containers:
==================
Container Name       Code Location                                    Status      Last Updated
-------------------  -----------------------------------------------  ----------  --------------------
my_container         ~/projects/my_code                               (running)   5 minutes ago
another_container    ~/workspace/another_project                      (stopped)   2 hours ago

Maestro>
```

#### Configuration

Configuring Maestro is simple. The easiest way is to provide a json file structured like the following:

```json
{
  "ContainerNameOne": {
    "container_working_dir": "usr/src/app/",
    "workspace": "~/workspace/repoOne"
  },
  "ContainerNameTwo": {
    "container_working_dir": "usr/src/app/",
    "workspace": "~/workspace/repoTwo"
  }
  ...
}
```

And run the configure command in the CLI:

```sh
Maestro> configure
Enter the path to the configuration file: ~/MaestroConfig.json
Maestro successfully configured.
Maestro>
```

The other option is to go through the CLI's add command...

TBD

## Requirements

## Troubleshooting

## Contributing

## License
