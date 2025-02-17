<div align="center">
  <h1>Lightflus</h1>
  <p>
    <strong>Lightflus is a cloud-native distributed stateful dataflow engine. </strong>
  </p>
  <p>

[![CI](https://github.com/Lady-Summer/lightflus-runtime/actions/workflows/workflow.yml/badge.svg)](https://github.com/Lady-Summer/lightflus-runtime/actions/workflows/workflow.yml)
</p>
</div>

**Important Notes**

1. Lightflus is in **preview** now, some features are still in development. If you want to try a better Lightflus, please wait for the demo version released.

2. We will be very appreciate if you can give us any feedback, including design, features support and more;


## Scenarios for Lightflus

1. Large-scale real-time computation;
2. CDC (Change Data Capture);
3. Data Integration Pipeline;

## Design Philosophy
**Typescript API + Rust Runtime**

Lightflus is designed for most developer teams even no one is familiar with streaming computation. Any of your team member can write a dataflow task and deploy it on production. Lightflus can connect with any event source (Kafka, MQTT, etc) in your cloud infra and emit the output result into the storage sink (mysql, redis, etc) which is processed by user-defined Dataflow. 

Lightflus is powered by [Deno](https://github.com/denoland/deno) and implemented in Rust which can ensure memory safe and real-time performance. We embed `v8` engine into Lightflus engine with minimum dependencies makes it light and fast; With the help of `Deno`, you can run `Typescript` user-defined functions or `WebAssembly` encoded bytes code (for better performance) in Lightflus with stream-like API; 


## Document
You can read the [document](https://humorous-bream-e48.notion.site/Lightflus-Document-217eedc73610413ba2a4f0c374d66c77) for more details about Lightflus


## Roadmap
You can get Lightflus Roadmap in this [Jira Dashboard](https://lightflus.atlassian.net/jira/software/c/projects/LIG/boards/1/roadmap?shared=&atlOrigin=eyJpIjoiOWJhOTRiOGNkZTBlNDY5OWFkZWU4ZGQxYjRkYTg3MTkiLCJwIjoiaiJ9)

## Community

Welcome all to join [Slack](https://lightflus.slack.com/join/shared_invite/zt-1hqwryop3-jWOhWSuQ2B7wulhQM5~sHQ#/shared-invite/email) Community! 

## Preparation

1. Install Rust environment:
    1. Go to official page: https://www.rust-lang.org/ to download installation pkg
2. Visual Studio Code + Rust Extension (**Recommended**);
3. Docker engine and swarm mode opened (**Recommended**);

## Start up

### Installation

Three ways to start Runtime:

1. Binary Packages (**Not Recommanded**)

```shell
$ cargo install --path src/worker

$ cargo install --path src/coordinator
```

Then Run by command

```shell
$ target/release/worker

$ target/release/coordinator
```

2. Docker Compose (**Recommended For Running Background**)

```shell
$ docker-compose up
```

3. Visual Studio Code (**Recommended for Debug**)
  * RocksDB lib should be installed on your PC
  * Rust & CodeLLDB extensions are installed

## For Contributor

Welcome to Lightflus Team. Our mission is to create an advanced, high performance, scalable and stable streaming system which is based on Dataflow Model in the Cloud.

You can read following documents to know more about Lightflus. If you have no access, please contact with the admin.

1. You can read the [Developer Guide](https://www.notion.so/Developer-Guide-bb6579a980844cff9b2702dd107e4ff3) to get more details for contribution.
2. [Architecture Overview](https://www.notion.so/Architecture-Overview-be9b006c61884db58e40dbd00e00b77d) will help you to get familiar with Lightflus Architecture Design
3. [Product Design](https://www.notion.so/Product-Design-efa990263c4b4e80a677243efc95a2f2) will tell you more about why we
   decide to create a streaming system by ourselves, not fork open source projects like Flink.
4. [Design Doc](https://www.notion.so/Design-282e33dc26a0416f9b25d20d78fe69d3) has the information of the historical design details of each version.

## How to release New Tag

### Tag's name template:

* For test (master branch): ``dev.v{yyyyMMdd}.{version}``
* For production (master branch): ``prod.v{yyyyMMdd}.{version}``

### CICD pipeline
After you pull request / release a new tag, cicd pipeline will be triggered.
