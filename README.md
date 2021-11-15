# Simple Linux PAM module in RUST
## About
Simple, stupid Linux PAM module to demonstrate how to write a PAM module in RUST. When you sudo any command, the module
will ask for your password and challenge that password with the Authentication Service, which rotates it afterwards.
# Requirements
* Docker
* Docker-Compose
* GNU Make
#Workflow
##Building
```shell
make build
```
build without cache
```shell
make rebuild
```
##Running
```shell
make up
```
##Attaching
Attach to the dummy Linux container, where sudo using this PAM module
```shell
make attach
```
